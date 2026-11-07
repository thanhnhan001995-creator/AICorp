# Kiểm tra hồi quy J-01 — Lần 5

- **Review ID:** `AIC-REV-J01-20261107-05`
- **Date:** 2026-11-07 (Asia/Bangkok)
- **Scope:** J-01, T-01 gốc và Fix01–Fix04.
- **Yêu cầu gốc:** [Brief](../brief.md) `AIC-BRF-20261105-01`, mục 4–5, 7–10, 13; [Ideas](../ideas.md) I-02/I-03; [Jobs](../Jobs.md) J-01.
- **Nguồn triển khai:** [T-01 Fix04](../Tasks/J01/T-01-Fix04.md), [Execute Fix04](../Executive/J01/Exe-T-01-Fix04.md), [evidence Fix04](../Evidence/J01/envidence-T-01-Fix04.md), [contract quyền](../contracts/authority-precedence-contract.md), `app/src-tauri/src/lib.rs`, `app/src/App.tsx`.
- **Ranh giới:** Review đánh giá mã/tài liệu/evidence hiện có. Build, Rust regression và CEO runtime/UI verification chỉ được xem là hoàn tất khi có bằng chứng chạy thực; không suy diễn từ source hay TypeScript pass.

## 1. Kết quả đã được khắc phục sau Fix04

| Điều kiện | Bằng chứng implementation | Trạng thái trong review |
| --- | --- | --- |
| Một nguồn policy/evaluator duy nhất. | Command legacy đã bị bỏ khỏi handler; `authority_envelopes` chỉ dùng migration; `evaluate_authority_policy` là đường evaluator còn lại. | Khớp ở mức code tĩnh. |
| Subject quyền không mơ hồ. | CHECK/trigger XOR, API guard, query role chỉ join khi policy không có employee subject. | Khớp ở mức code tĩnh. |
| CEO có thể kiểm tra request quyền cụ thể. | UI có action/work/resource/quota/delegate/self-approve và trả decision metadata. | Đã hiện thực; chưa có runtime evidence. |
| Lịch sử grant/narrow/revoke đọc được. | `authority_policy_events` append-only, history UI, revoke bắt buộc lý do. | Đã hiện thực; xem atomicity còn lại ở D-05-02. |
| Runtime DTO không trả `connection_reference`. | `RuntimeBinding`/`list_runtime_bindings` chỉ trả ID/provider/display/status. | Khớp ở code path hiện có. |
| Documentation J-01 không còn lẫn J-02/link Task sai. | Bundle J-01 và contract/evidence đã được chuẩn hóa. | Đạt trong phạm vi kiểm tra `docs/Tasks/J01`. |

## 2. Sai lệch còn lại so với Brief, Ideas và output J-01

### D-05-01 — CEO chưa tạo/duy trì được tổ chức, chức vụ và năng lực như một mô hình điều hành

J-01 yêu cầu CEO tạo được một tổ chức với hồ sơ nhân viên; I-02 yêu cầu hồ sơ có chức vụ, năng lực khai báo, runtime, tuyến báo cáo, trạng thái, lịch sử và phạm vi quyền. Implementation hiện chỉ có `initialize_demo_organization` seed cố định `org_aicorp_hq`/`role_operations`; handler chỉ có `get_organization`, `list_roles`, `create_employee`, `update_employee`. Không có command/UI để CEO tạo/sửa tổ chức hoặc role, và schema/UI không có trường năng lực khai báo.

**Tác động:** CEO chưa thực sự xây và duy trì công ty đa chức năng của mình; đang thao tác bên trong một tổ chức demo cố định. Đây là sai lệch trọng yếu của J-01/I-02, không thuộc phạm vi Fix04 nhưng phải được tính trong regression đối chiếu yêu cầu gốc.

### D-05-02 — Grant policy và migration legacy chưa bảo toàn history theo transaction nguyên tử

`narrow_authority_policy` và `revoke_authority_policy` dùng transaction. Ngược lại, `create_authority_policy` ghi policy rồi mới gọi `append_policy_event` ngoài transaction; `migrate_legacy_authority_envelopes` cũng insert policy và event theo hai thao tác rời. Nếu event insert lỗi, policy vẫn tồn tại nhưng thiếu lịch sử grant/migration.

**Tác động:** trái với Brief mục 8 (“mọi thay đổi quan trọng phải để lại lịch sử đọc được”) và contract Fix04 luật 8. Cần transaction hoá grant/migration, đồng thời thêm test ép event failure rollback policy.

### D-05-03 — Validation secret-like chưa là ranh giới ghi dữ liệu runtime hoàn chỉnh

`validate_runtime_display_field` hiện được gọi khi seed runtime mẫu; không có create/update runtime command hay một write path tổng quát áp dụng validation cho mọi nhãn runtime. Test mới gọi helper trực tiếp. Vì vậy chưa chứng minh được bất kỳ đường ghi runtime trong tương lai/ngoài seed đều bị chặn trước khi persistence.

**Tác động:** D-04-04 đã được cải thiện, nhưng chưa đóng hoàn toàn yêu cầu “không vô tình lưu/hiển thị secret” theo Fix02 và Brief mục 9. Cần một write boundary runtime thực sự (hoặc tuyên bố/ép runtime bindings là immutable seed-only) và test qua boundary đó.

### D-05-04 — Lịch sử riêng của hồ sơ nhân viên ban đầu chưa được biểu diễn

J-01 output nói mỗi hồ sơ có “lịch sử rỗng ban đầu”; I-02 yêu cầu lịch sử công việc và trạng thái. UI hiện chỉ có **LỊCH SỬ POLICY**, không có surface hay model cho lịch sử nhân viên/work history rỗng. Lịch sử kinh nghiệm sâu hơn thuộc J-08, nhưng không thay thế trạng thái/lịch sử hồ sơ cơ bản được J-01 hứa.

**Tác động:** CEO chưa phân biệt được “chưa có lịch sử công việc” với “không có dữ liệu lịch sử”, làm chưa khớp hoàn toàn I-02/J-01.

### D-05-05 — Verification bắt buộc vẫn chưa thực hiện được

Evidence Fix04 ghi TypeScript type-check pass, nhưng `cargo test -q` không chạy vì toolchain/PATH không có `cargo`; `vite build` bị `EPERM` khi ghi file tạm. CEO runtime/UI verification cũng chưa diễn ra. Do đó các case migration, trigger, rollback, decision và UI không được nâng từ “có source” thành “đã chứng minh chạy đúng”.

**Tác động:** J-01 là nền quyền cho J-03/agent thật; không thể coi build hoặc Execute là bằng chứng thay thế regression/runtime.

## 3. Kết luận regression J-01

**J-01: `Implemented with residual deviations — not ready for CEO acceptance`.**

Fix04 đã giải quyết phần lớn D-04-01…D-04-07 ở implementation và làm Hộp uỷ quyền nhất quán hơn đáng kể. Tuy nhiên D-05-01 là thiếu phạm vi điều hành gốc, D-05-02 là lỗ hổng atomicity/history, và D-05-05 ngăn kết luận kỹ thuật. Vì vậy không lập AcceptanceReport và chưa coi J-01 là dependency đã hoàn tất cho J-03/J-04/J-05/J-06/J-08.

## 4. Điều kiện cho Fix tiếp theo

1. Bổ sung CEO quản trị tổ chức/role và năng lực khai báo của nhân viên; tránh khóa dữ liệu vào seed demo.
2. Transaction hoá grant/migration policy-event, thêm regression ép lỗi event để chứng minh rollback.
3. Thiết kế ranh giới ghi runtime được validate hoặc loại bỏ hẳn write path không an toàn; test qua ranh giới này.
4. Hiển thị lịch sử hồ sơ rỗng/initial state tách biệt với lịch sử policy và J-08 experience.
5. Khôi phục toolchain `cargo`, chạy Rust regression/build; sau khi technical review không còn sai lệch trọng yếu, CEO thực hiện checklist runtime/UI.
