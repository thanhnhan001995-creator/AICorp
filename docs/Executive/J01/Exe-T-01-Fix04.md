# Execute — T-01 Fix04

- **Execute ID:** `AIC-EXE-J01-FIX04-20261107-01`
- **Task referenced:** [T-01 Fix04](../../Tasks/J01/T-01-Fix04.md)
- **Job referenced:** [J-01](../../Jobs.md)
- **Brief / Ideas:** [brief](../../brief.md) `AIC-BRF-20261105-01`; [ideas](../../ideas.md) I-02/I-03.
- **Review source:** [Kiểm tra hồi tố lần 4](../../Reviews/J01/j01-retrospective-verification-04.md) — D-04-01 đến D-04-07.
- **Boundary:** báo cáo thuần túy kết quả triển khai và kiểm tra kỹ thuật; không chứa xác nhận nghiệm thu của CEO.

=============================================================================================================================================================================

## T-01.03-Fix04.1 — Hợp nhất đường đi Hộp uỷ quyền

- **Kết quả:** Đã bỏ các command/evaluator legacy khỏi Tauri handler và source runtime. `authority_envelopes` chỉ còn là bảng nguồn cho migration một lần sang `authority_policy_versions`; evaluator được hỗ trợ duy nhất là `evaluate_authority_policy`.
- **Artefact kiểm tra:** `app/src-tauri/src/lib.rs`; `docs/contracts/authority-precedence-contract.md` luật 7.
- **Lỗi:** Không có lỗi triển khai được phát hiện bằng kiểm tra tĩnh.

## T-01.03-Fix04.2 — Subject policy độc nhất

- **Kết quả:** Schema mới có CHECK XOR, trigger insert/update cho database đã tồn tại, API guard và kiểm tra dữ liệu legacy mơ hồ khi mở DB. Query list/evaluator chỉ join role khi policy không có employee subject.
- **Artefact kiểm tra:** `authority_policy_exactly_one_subject_*`, `validate_existing_policy_subjects`, `list_authority_policies_from`, `evaluate_policy_from` trong `app/src-tauri/src/lib.rs`.
- **Lỗi:** Rust regression chưa chạy vì `cargo` không có trong PATH; xem evidence.

## T-01.03-Fix04.3 — Lịch sử thay đổi quyền

- **Kết quả:** Đã thêm `authority_policy_events` append-only, event grant/narrow/revoke cùng thời điểm, actor/nguồn, lý do và policy liên quan. UI gọi `list_authority_policy_events` và hiển thị lịch sử.
- **Artefact kiểm tra:** `app/src-tauri/src/lib.rs`; `app/src/App.tsx` mục **LỊCH SỬ POLICY**.
- **Lỗi:** Chưa có runtime UI verification; thuộc bước kiểm tra CEO sau technical regression.

## T-01.02-Fix04.4 — Chặn nhãn runtime nhạy cảm

- **Kết quả:** Đã bổ sung `validate_runtime_display_field`, từ chối marker secret-like; runtime DTO vẫn không chứa `connection_reference`.
- **Artefact kiểm tra:** `app/src-tauri/src/lib.rs`, test `runtime_labels_reject_secret_like_values_and_ledger_remains_append_only`.
- **Lỗi:** Test Rust chưa chạy do thiếu `cargo`.

## T-01.03-Fix04.5 — Regression evaluator và atomicity

- **Kết quả:** Đã thay một regression lớn bằng năm test có tên, gồm subject XOR/role isolation, metadata/override, expiry/revoke/migration, narrow/history/rollback, runtime label/ledger.
- **Artefact kiểm tra:** module `#[cfg(test)]` trong `app/src-tauri/src/lib.rs`.
- **Lỗi:** `cargo test -q` không khởi động trong môi trường hiện tại: `cargo` không được nhận diện.

## T-01.04-Fix04.6 — Trình kiểm tra quyền hiệu lực

- **Kết quả:** Đã thay kiểm tra policy mẫu bằng form request gồm action, work scope, resource scope, quota, delegate và self-approve. Kết quả hiển thị allow/deny, reason code, policy/source/limits.
- **Artefact kiểm tra:** `app/src/App.tsx`; `npm exec tsc -- -p tsconfig.app.json --noEmit` pass.
- **Lỗi:** `npm exec vite build` không tạo được file tạm trong `node_modules/.vite-temp` do `EPERM`; đây là giới hạn quyền ghi của môi trường, không phải lỗi TypeScript.

## T-01.05-Fix04.7 — Truy vết tài liệu và evidence

- **Kết quả:** Đã xóa heading J-02 bị lẫn khỏi T-01 Fix03, sửa các link Task index sai trong các bundle J-01, cập nhật contract precedence và tạo evidence này.
- **Artefact kiểm tra:** `docs/Tasks/J01/`, `docs/contracts/authority-precedence-contract.md`, [evidence Fix04](../../Evidence/J01/envidence-T-01-Fix04.md).
- **Lỗi:** Không có lỗi tài liệu còn lại được phát hiện qua `rg` ở phạm vi `docs/Tasks/J01`.
