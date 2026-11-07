# Kiểm tra hồi tố J-01 — Lần 2

- **Review ID:** `AIC-REV-J01-20261107-02`
- **Date:** 2026-11-07 (Asia/Bangkok)
- **Scope:** J-01 nguyên bản (`T-01.01` → `T-01.05`) và phần khắc phục Fix01 (`T-01.02-Fix01.1` → `T-01.05-Fix01.5`).
- **Nguồn đối chiếu:** [`brief.md`](../../brief.md) mục 5, 7, 11, 13; [`ideas.md`](../../ideas.md) I-02, I-03; [`Jobs.md`](../../Jobs.md) J-01; [`Tasks.md`](../../Tasks.md); [`execute.md`](../../execute.md); [`AcceptanceReport.md`](../../AcceptanceReport.md).
- **Phương pháp:** kiểm tra hợp đồng, mã hiện tại, test hiện có và các tuyên bố evidence. Build/test không được suy diễn thành CEO runtime verification.

## 1. Kết quả so khớp ở trạng thái mới nhất

| Nhu cầu Brief / Idea | Bằng chứng hiện có | Kết luận |
| --- | --- | --- |
| Nhân viên có identity bền vững; role tách runtime (I-02). | Contract ID/entity; SQLite employee/role/runtime; UI hồ sơ. | **Đạt ở mức implemented.** |
| Local-first; UI không ghi SQLite trực tiếp. | Rust `rusqlite`, Tauri commands, React `invoke`; build/test pass. | **Đạt ở mức implemented.** CEO chưa xác nhận runtime bản mới. |
| Tuyến báo cáo không chu trình. | `ensure_valid_manager`; test self/indirect cycle. | **Đạt ở mức automated evidence.** |
| Quyền không suy ra từ local/API (I-03). | Policy chỉ được tạo qua CEO-facing command/UI; runtime không cấp quyền. | **Đạt ở mức thiết kế/mã.** |
| Quyền theo chức vụ hoặc công việc; scope/time/budget/delegate/self-approve. | `authority_policy_versions` + evaluator có field tương ứng. | **Đúng một phần.** Backend hỗ trợ `role_id`, nhưng UI luôn gửi `roleId: null`, nên CEO chưa cấp policy theo chức vụ. |
| Thu hẹp/thu hồi quyền có lịch sử. | Status `narrowed`/`revoked`, `supersedesId`, UI buttons. | **Đúng một phần.** History có, nhưng `narrow_authority_policy` không kiểm chứng policy mới thực sự hẹp hơn policy cũ; CEO có thể vô tình mở rộng quyền qua luồng “thu hẹp”. |
| Quyết định quyền từ chối vượt phạm vi với lý do. | `evaluate_policy_from` và test sai work/resource/quota/self-approve. | **Đúng một phần.** Thiếu test hết hạn, revoke và positive self-approve; matching hiện exact string, chưa có ngữ nghĩa scope/pattern được contract gợi ý. |
| CEO quản trị Hộp uỷ quyền đầy đủ (T-01.04). | Form policy nhập scope/action/resource/hạn/quota/delegate/self-approve/lý do; tạo/thu hẹp/thu hồi. | **Đúng một phần.** Không có lựa chọn source theo chức vụ; chưa hiển thị rõ source/`supersedesId`/quyền hiệu lực và lý do evaluator trong UI. |
| T-01.05 kiểm tra tự động + UI tối thiểu. | Một Rust test tổng hợp; `npm run build`; Acceptance checklist. | **Chưa đủ.** Không có test UI/runtime bản mới, không test hết hạn/revoke/role policy, không có evidence CEO thao tác. |

## 2. Đối chiếu theo Task gốc

| Task gốc | Trạng thái sau Fix01 | Kết luận hồi tố lần 2 |
| --- | --- | --- |
| T-01.01 | Contract còn hợp lệ. | **Implemented; không thấy sai lệch mới.** |
| T-01.02 | Fix01.1 thêm graph validation. | **Implemented, còn theo dõi:** runtime reference cần tiếp tục tách khỏi secret khi connector thật xuất hiện; CEO CRUD role/runtime chưa thuộc phạm vi hiện có. |
| T-01.03 | Fix01.2/.3 thêm policy version/evaluator. | **Implemented with residual deviations:** role policy chưa thao tác được từ UI; narrowing chưa bảo đảm không mở rộng; coverage evaluator chưa đủ. |
| T-01.04 | Fix01.4 thay UI quyền mẫu bằng form policy. | **Implemented with residual deviations:** thiếu role source, hiển thị source/history/effective decision và runtime UI proof. |
| T-01.05 | Fix01.5 thêm test/checklist. | **Implemented with residual deviations:** test không bao phủ toàn bộ tiêu chí; chưa có CEO runtime evidence. |

## 3. Sai lệch phải giữ lại, không được coi là đã nghiệm thu

1. **Policy theo chức vụ chưa dùng được từ UI.** Đây là sai lệch trực tiếp với T-01.03-Fix01.2.
2. **Thu hẹp chưa bị ràng buộc là hẹp hơn.** Đây là rủi ro quyền mở rộng sai tên thao tác.
3. **Coverage quyền còn thiếu:** expired, revoked, positive self-approve, role-derived policy và đường UI mới.
4. **Evidence runtime/UI chưa có:** build và Rust test không xác nhận CEO thực hiện được toàn bộ kịch bản.
5. **`connectionReference` vẫn được Rust trả cho UI.** Hiện không chứa secret, nhưng cần guard/masking trước khi connector thật đưa credential/reference nhạy cảm vào hệ thống.

## 4. Kết luận và trạng thái đề xuất

**J-01: `Implemented with residual deviations — Not CEO-accepted`.**

Fix01 đã nâng đáng kể nền tảng và giải quyết các sai lệch lớn của review lần 1, nhưng không đủ căn cứ để chuyển J-01 thành `Accepted`. CEO dùng [`AcceptanceReport.md`](../../AcceptanceReport.md) để xác nhận những phần đã kiểm tra; các sai lệch ở mục 3 cần tạo một lượt Fix mới nếu CEO yêu cầu J-01 đạt đầy đủ trước J-03.

## 5. Evidence tái kiểm tra

1. Chạy `npm run build` trong `app`.
2. Chạy `cargo test` trong `app/src-tauri`.
3. Mở `app/src-tauri/src/lib.rs`: `ensure_valid_manager`, `authority_policy_versions`, `narrow_authority_policy`, `evaluate_policy_from` và test module.
4. Mở `app/src/App.tsx`: policy form, nút Thu hẹp/Thu hồi và các field nhập.
5. CEO chỉ ghi quyết định tại [`AcceptanceReport.md`](../../AcceptanceReport.md), không ghi vào `execute.md`.
