# Execute — T-01 Fix05

- **Execute ID:** `AIC-EXE-J01-FIX05-20261107-01`
- **Task referenced:** [T-01 Fix05](../../Tasks/J01/T-01-Fix05.md)
- **Review source:** [Review J-01 lần 5](../../Reviews/J01/j01-retrospective-verification-05.md) — D-05-01 đến D-05-05.
- **Boundary:** ghi kết quả triển khai/lỗi/cách kiểm tra; không có acceptance của CEO.

=============================================================================================================================================================================

## T-01.01-Fix05.1 — Tổ chức, chức vụ và năng lực

- **Kết quả:** Đã thêm create/update organization, create/update role; employee có trường `capabilities` do CEO nhập. UI có vùng **TỔ CHỨC** và **CHỨC VỤ**, hồ sơ nhân viên hiển thị/ghi năng lực.
- **Artefact:** `app/src-tauri/src/lib.rs`, `app/src/App.tsx`.
- **Lỗi:** Chưa có runtime walkthrough CEO.

## T-01.03-Fix05.2 — Atomicity policy/history

- **Kết quả:** `create_authority_policy` gọi implementation transaction; legacy migration ghi policy và event migration trong cùng transaction. Narrow/revoke tiếp tục transaction.
- **Artefact:** `create_authority_policy_from`, `migrate_legacy_authority_envelopes` trong `app/src-tauri/src/lib.rs`.
- **Lỗi:** Lỗi borrow migration và collision ID test song song đã được phát hiện/sửa trong regression; `cargo test -q` cuối pass 8/8.

## T-01.02-Fix05.3 — Runtime write boundary

- **Kết quả:** Runtime J-01 được chốt seed-only/immutable bằng trigger update/delete và helper insert có validation secret-like trước persistence; DTO UI không có technical reference.
- **Artefact:** [runtime-binding contract](../../contracts/runtime-binding-contract.md), `insert_seed_runtime_binding`.
- **Lỗi:** Không có lỗi regression; test runtime boundary đã nằm trong 8/8 Rust pass.

## T-01.01-Fix05.4 — Lịch sử hồ sơ rỗng

- **Kết quả:** Đã thêm `employee_profile_events` append-only. Hồ sơ mới hiển thị trạng thái lịch sử rỗng rõ ràng; cập nhật hồ sơ ghi event CEO có thời điểm/nguồn, tách riêng policy history.
- **Artefact:** `list_employee_profile_events`, vùng **LỊCH SỬ HỒ SƠ** trong UI.
- **Lỗi:** Chưa có runtime UI verification.

## T-01.05-Fix05.5 — Verification/evidence

- **Kết quả:** TypeScript type-check pass. Evidence được lưu riêng theo Job/task.
- **Artefact:** [envidence-T-01-Fix05.md](../../Evidence/J01/envidence-T-01-Fix05.md).
- **Lỗi:** Environment PATH của Codex vẫn không tự nhận `cargo`, nhưng chạy bằng executable Rustup tuyệt đối đã pass 8/8; `npm run build` production cũng pass. CEO runtime/UI verification vẫn chưa thực hiện.
