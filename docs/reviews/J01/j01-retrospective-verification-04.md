# Kiểm tra hồi tố J-01 — Lần 4

- **Review ID:** `AIC-REV-J01-20261107-04`
- **Date:** 2026-11-07 (Asia/Bangkok)
- **Scope:** J-01, T-01 gốc, Fix01, Fix02 và Fix03.
- **Nguồn yêu cầu:** [Brief](../brief.md) — `AIC-BRF-20261105-01`, đặc biệt mục 5, 7, 8, 9, 11 và 13; [Ideas](../ideas.md) — I-02/I-03; [Jobs](../Jobs.md) — J-01.
- **Nguồn triển khai đã đối chiếu:** [T-01 Fix03](../Tasks/J01/T-01-Fix03.md), [Execute Fix03](../Executive/J01/Exe-T-01-Fix03.md), [hợp đồng precedence](../contracts/authority-precedence-contract.md), [evidence Fix03](../Evidence/J01/envidence-T-01-Fix03.md), `app/src-tauri/src/lib.rs` và `app/src/App.tsx`.
- **Ranh giới:** Đây là kiểm tra hồi tố kỹ thuật và truy vết tài liệu. Không suy diễn CEO runtime/UI verification hoặc acceptance từ build, test hay Execute.

## 1. Cách kiểm tra

1. Đọc yêu cầu gốc trước, không lấy tuyên bố `Execute` làm bằng chứng đủ.
2. Đối chiếu từng điều kiện đầu ra của J-01 với implementation Rust/SQLite, React UI, contract và regression hiện có.
3. Tái chạy build/test nếu môi trường hiện tại cho phép; tách rõ kết quả hiện tại với evidence đã ghi của Fix03.

### Kết quả lệnh trong lượt review này

| Lệnh | Kết quả | Diễn giải |
| --- | --- | --- |
| `npm run build` tại `app` | Không chạy hết: TypeScript không ghi được `node_modules/.tmp/*.tsbuildinfo` do `Access is denied`. | Đây là giới hạn quyền ghi của môi trường review hiện tại trên `D:\Moi-Truong_AI`, không phải bằng chứng source bị lỗi. Không thay thế kết quả pass đã ghi trong evidence Fix03. |
| `cargo test -q` | Không chạy trong lượt này vì lệnh chuỗi đã dừng sau lỗi build. | Evidence Fix03 ghi pass 1 regression test; tuy nhiên evidence đó không tự xử lý các thiếu sót coverage nêu dưới. |

## 2. Những phần đã khớp và được giữ lại

| Yêu cầu Brief / Ideas / J-01 | Bằng chứng đối chiếu | Kết luận |
| --- | --- | --- |
| Hồ sơ nhân viên có danh tính, chức vụ, runtime tách rời, cấp quản lý và trạng thái. | `organizations`, `roles`, `employees`, `runtime_bindings`; UI hồ sơ; `ensure_valid_manager`. | Đạt về implementation hiện có. |
| Tuyến quản lý không được tự tham chiếu hoặc tạo vòng. | `ensure_valid_manager`; regression self-cycle và indirect-cycle. | Đạt. |
| Reference runtime không đi ra DTO/UI thường. | `RuntimeBinding` và `list_runtime_bindings` chỉ chọn ID/provider/display/status; React dùng DTO này. | Đạt một phần; validation dữ liệu nhạy cảm còn thiếu ở D-04-04. |
| Hộp uỷ quyền có action, work scope, resource scope, hạn, quota, delegate, self-approve, source và trạng thái. | `authority_policy_versions`, form policy, danh sách policy. | Đạt về mô hình dữ liệu/UI cơ bản. |
| Fix03 giữ chủ thể trong thao tác narrow. | `is_narrower` so sánh cả `employee_id` và `role_id`; `narrow_authority_policy` dùng transaction. | Đã khắc phục D-03-01 ở mức implementation. |
| Precedence role–employee được tuyên bố rõ và evaluator có employee override boundary. | Contract `AIC-CON-AUTH-20261107-01`; `evaluate_policy_from`. | Đã khắc phục D-03-02 ở semantics chính. |
| Quyết định evaluator có allow/deny, reason code, ID/source/limits. | `AuthorityDecision`, `evaluate_authority_policy`. | Đạt một phần; UI và assertion còn thiếu tại D-04-03/D-04-05. |

## 3. Sai lệch còn lại

### D-04-01 — Hai cơ chế Hộp uỷ quyền cùng tồn tại, có thể cho kết quả trái nhau

`authority_envelopes` cùng các command `create_authority_envelope`, `revoke_authority_envelope`, `evaluate_authority` vẫn được đăng ký trong Tauri handler. Chúng dùng schema/evaluator cũ, exact-match scope, không xét role-policy precedence và trả các mã `allowed_legacy_envelope`/`no_matching_legacy_envelope`. Trong khi đó UI mới dùng `authority_policy_versions`.

**Tác động:** J-01/I-03 yêu cầu một Hộp uỷ quyền đọc được và có ranh giới quản trị rõ ràng. Hai nguồn quyền có thể đưa ra hai quyết định khác nhau cho cùng nhân viên; command cũ cũng tạo đường đi bỏ qua semantics Fix03. Đây là sai lệch trọng yếu trước khi J-03 dùng quyền để chạy agent thật.

### D-04-02 — Server chưa ép policy có đúng một chủ thể (employee XOR role)

Schema chỉ kiểm tra `employee_id IS NOT NULL OR role_id IS NOT NULL`; `create_authority_policy` cũng chỉ từ chối khi **cả hai** cùng trống. Một caller ngoài UI vẫn có thể gửi cả `employeeId` lẫn `roleId`. Với truy vấn evaluator hiện tại (`p.employee_id = ? OR e.id = ?`), bản ghi đó vừa có thể tác động nhân viên được chỉ định, vừa có thể được xét cho các nhân viên mang role đó.

**Tác động:** trái với khái niệm nguồn quyền theo **nhân viên hoặc chức vụ** và làm mơ hồ subject/precedence. UI chọn một trong hai không thay thế validation server-side.

### D-04-03 — Bề mặt “Quyền hiệu lực” không cho CEO kiểm tra request thực tế hoặc chứng minh case deny

`inspectPolicy` tự lấy action/scope/resource từ policy đang active, tạo một scope mẫu luôn nằm trong policy, đồng thời luôn gửi `requiresDelegate: false` và `requiresSelfApprove: false`. CEO không nhập/chọn work scope, resource, quota, delegate hay self-approve; không thể chủ động kiểm tra employee override hẹp, expiry, quota hoặc denial reason qua UI.

**Tác động:** chưa đạt đầu ra T-01.04-Fix03.4 và T-01.04-Fix02.5: CEO phải kiểm tra request role/employee override qua UI và nhìn lý do deny cụ thể. Phần UI hiện tại là kiểm tra policy thuận lợi, không phải effective-decision inspector.

### D-04-04 — Yêu cầu validation chống secret-like value chưa được hiện thực và kiểm thử

DTO an toàn chỉ che `connection_reference` khi đọc, nhưng không có validation nào cho `display_name` hoặc `provider_kind` để từ chối giá trị trông như secret. Không có API quản trị runtime trong implementation hiện tại, nhưng nhiệm vụ T-01.02-Fix02.1 yêu cầu cả validation lẫn test chứng minh secret-like value không thể được ghi vào trường hiển thị.

**Tác động:** chưa thể kết luận requirement “không hiển thị bí mật một cách vô tình” đã được đóng ở ranh giới dữ liệu. Che field ở một DTO là cần thiết nhưng không đủ.

### D-04-05 — Regression Fix03 chưa phủ đủ các assertion mà task yêu cầu

Chỉ có một test Rust lớn. Nó có positive delegate và nhiều deny scope/quota/expiry/revoke, nhưng vẫn thiếu assertion riêng cho:

- role policy chỉ áp dụng đúng nhân viên/role và không áp dụng role khác;
- positive `self-approve`;
- `policyId`, `policySource`, `matchedLimits`, `reasonCode` của từng decision;
- gọi thực `narrow_authority_policy` để chứng minh attempt đổi subject bị reject và DB không đổi trạng thái;
- atomicity khi insert successor lỗi, không để policy cũ narrowed mà thiếu successor.

**Tác động:** T-01.03-Fix03.3 yêu cầu test mỗi case với assertion riêng và truy ngược được decision; evidence hiện có mới chứng minh một phần semantic, chưa đủ để làm ranh giới quyền đáng tin cho agent thật.

### D-04-06 — Lịch sử thay đổi quyền chưa đủ đọc được/truy nguyên cho CEO

Policy có `status` và `supersedes_id`, nhưng không lưu/hiển thị thời điểm tạo, thời điểm revoke/narrow, actor hoặc lý do revoke. `revoke_authority_policy` cập nhật trạng thái tại chỗ không tạo version/event lý do. Vì J-01 cam kết CEO xem được lịch sử thay đổi quyền, trạng thái hiện tại và predecessor đơn thuần chưa đủ trả lời ai đổi gì, khi nào và vì sao.

**Tác động:** chưa khớp hoàn toàn Brief mục 8/13 (lịch sử đọc được, CEO luôn can thiệp/truy nguyên) và đầu ra J-01. J-02 sẽ cung cấp ledger bất biến sau này, nhưng không loại bỏ requirement J-01 phải có history quyền có thể xem.

### D-04-07 — Truy vết tài liệu Task bị lẫn Job và có liên kết nguồn sai

Cuối file [T-01 Fix03](../Tasks/J01/T-01-Fix03.md) bị kèm heading `## 4. Tasks của J-02`; các mục Fix03 liên kết `../../README.md`, từ thư mục `Tasks/J01` không trỏ tới `Tasks/README.md`. Điều này không làm đổi runtime, nhưng làm yếu khả năng CEO truy nguyên đúng phạm vi Job/Task khi audit.

## 4. Kết luận đối chiếu J-01

**J-01: `Implemented with residual deviations — not ready for CEO acceptance`.**

Fix03 đã sửa đúng hai sai lệch trọng yếu của review 3: narrow giữ subject và employee override không fallback sang role policy. Tuy vậy D-04-01 và D-04-02 vẫn làm nguồn quyền không đơn nghĩa; D-04-03 đến D-04-06 còn thiếu khả năng kiểm tra/coverage/lịch sử mà Brief và I-03 yêu cầu. Vì vậy chưa chuyển sang AcceptanceReport và chưa dùng J-01 như nền quyền đã hoàn tất cho agent thật ở J-03.

## 5. Điều kiện cho lượt Fix tiếp theo và review sau

1. Chọn một nguồn policy duy nhất hoặc migration/deprecate command/table envelope cũ; mọi evaluator đi qua semantics đã chốt.
2. Ép chính xác một subject ở schema lẫn API (`employee XOR role`) và thêm test cho request gửi cả hai.
3. Thay nút kiểm tra policy bằng form effective decision: CEO chọn/nhập action, work/resource scope, quota, delegate, self-approve và xem allow/deny cùng policy/source/limits/reason.
4. Thêm validation và test cho nhãn runtime secret-like; giữ `connection_reference` ngoài DTO/UI.
5. Tách regression thành case có tên, phủ các thiếu sót D-04-05, bao gồm transaction rollback.
6. Hoàn thiện history policy có timestamp, actor/reason cho change/revoke hoặc ghi nối với ledger theo thiết kế kế thừa rõ ràng.
7. Sửa ranh giới/liên kết tài liệu đã lẫn để Task/Execute/Review truy về đúng J-01.

Sau khi các điều kiện kỹ thuật không còn sai lệch trọng yếu, CEO mới thực hiện runtime/UI verification rồi cân nhắc AcceptanceReport.
