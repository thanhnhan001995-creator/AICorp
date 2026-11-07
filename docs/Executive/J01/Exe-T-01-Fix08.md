# Execute — T-01 Fix08

- **Execute ID:** `AIC-EXE-J01-FIX08-20261107-01`
- **Ngày thực hiện:** 07/11/2026 (Asia/Bangkok)
- **Task nguồn:** [`T-01-Fix08.md`](../../Tasks/J01/T-01-Fix08.md)
- **Phạm vi:** chỉ xử lý lỗi thu hồi policy được ghi tại [AcceptanceReport J-01 lượt 3](../../AcceptanceReport/J01-AccRep/AcceptanceReport-J01-3rd.md).

================================================================================

## T-01.03-Fix08.1 — Tái lập và định vị lỗi

- Rà soát đường UI `revokePolicy` và API Tauri `revoke_authority_policy`.
- Tái lập bằng fixture SQLite: cấp `revoke_parent` → thu hẹp tạo successor `active` → thu hồi successor. Chuỗi thành công ở Rust/SQLite; parent là `narrowed`, successor là `revoked`, event `authority.revoked` có lý do.
- AcceptanceReport lượt 3 không có policy ID hoặc nguyên văn lỗi nên không thể khẳng định trạng thái duy nhất của policy CEO đã thao tác.
- Điểm gây khó kiểm tra: phiên bản cũ trả lỗi chung cho policy không tồn tại và policy không còn `active`.

## T-01.03-Fix08.2 — Sửa luồng thu hồi và thông báo lỗi

- Tách `revoke_authority_policy_from` để command Tauri và regression dùng chung write boundary.
- Backend truy vấn status trước khi ghi: policy không tồn tại báo ID; policy `narrowed`/`revoked` báo ID, status và yêu cầu chọn successor `active`; policy `active` được revoke cùng event trong một transaction.
- Không xóa/sửa policy hoặc event lịch sử.
- **File thay đổi:** `D:\Moi-Truong_AI\app\src-tauri\src\lib.rs`.

## T-01.05-Fix08.3 — Regression và handoff

| Kiểm tra | Cách kiểm | Kết quả |
| --- | --- | --- |
| Rust regression | `cargo.exe test -q` tại `app\src-tauri` | Pass: 9 tests. |
| Narrow → revoke → rollback event | `revoke_successor_records_history_and_rolls_back_when_revoke_event_fails` | Pass. |
| TypeScript | `npm exec tsc -- -p tsconfig.app.json --noEmit` tại `app` | Pass. |
| Production build | `npm run build` tại `app` | Pass. |

- `cargo fmt --check` báo dự án vốn chưa được format; không chạy format để tránh diff hàng loạt ngoài Fix08.
- Một lần check TypeScript dùng nhầm thư mục `src-tauri` nên không tìm thấy `tsconfig`; đã chạy lại đúng `app` và pass.
- Evidence: [`envidence-T-01-Fix08.md`](../../evidence/J01/envidence-T-01-Fix08.md). Review: [`j01-retrospective-verification-07.md`](../../reviews/J01/j01-retrospective-verification-07.md).
- File này không ghi approval/acceptance. CEO cần kiểm tra runtime/UI ở lượt nghiệm thu tiếp theo.
