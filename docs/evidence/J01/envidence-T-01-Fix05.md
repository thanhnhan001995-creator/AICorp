# Evidence kỹ thuật — T-01 Fix05

- **Evidence ID:** `AIC-EVD-FIX05-20261107-01`
- **Task referenced:** [T-01 Fix05](../../Tasks/J01/T-01-Fix05.md)
- **Boundary:** technical evidence, không phải CEO runtime/UI verification hoặc AcceptanceReport.

## Kiểm tra đã chạy

| Lệnh / kiểm tra | Kết quả | Phạm vi |
| --- | --- | --- |
| `npm exec tsc -- -p tsconfig.app.json --noEmit` tại `app` | Pass. | UI tổ chức/role/năng lực, lịch sử hồ sơ, policy inspector vẫn type-safe. |
| `npm run build` tại `app` | Pass. | TypeScript build và Vite production bundle hoàn tất. |
| `C:\Users\Owner\.cargo\bin\cargo.exe test -q` tại `app/src-tauri` | Pass: 8 tests. | Schema migration, policy evaluator/atomicity, runtime boundary, lịch sử và persistence hoạt động trong regression SQLite. |
| Static source scan | Pass. | Có API tổ chức/role, trường capabilities, `employee_profile_events`, policy grant transaction, migration transaction và runtime seed boundary. |

## Lỗi đã phát hiện và khắc phục trong verification

- Lần chạy đầu bắt lỗi Rust E0502: statement đọc legacy giữ immutable borrow khi migration mở transaction. Đã `drop` statement trước transaction.
- Khi thêm test persistence, test song song phát hiện collision ID tạm theo timestamp. `uuid_suffix` đã đổi thành timestamp + atomic counter; regression cuối pass 8/8.

## Chưa thực hiện

- CEO runtime/UI walkthrough: chưa thực hiện; chỉ CEO xác nhận sau khi technical regression không còn sai lệch trọng yếu.

## Artefact

- `app/src-tauri/src/lib.rs`: tổ chức/role CRUD, capabilities, employee profile history, transaction grant/migration policy, runtime immutable seed boundary.
- `app/src/App.tsx`: CEO chỉnh tổ chức, tạo/sửa role, nhập năng lực, xem lịch sử hồ sơ rỗng hoặc event cập nhật.
- `docs/contracts/runtime-binding-contract.md`: contract runtime binding seed-only/secret-safe.

## Regression cần chạy khi có Cargo

1. grant policy rollback khi insert policy event bị ép lỗi;
2. legacy migration rollback khi event migration bị ép lỗi;
3. runtime boundary reject secret-like value trước persistence;
4. organization/role/capabilities/profile history persistence qua reopen.

Các regression đã chạy trong `app/src-tauri/src/lib.rs`, gồm `grant_and_legacy_migration_rollback_when_policy_event_fails`, `runtime_seed_boundary_and_profile_history_have_explicit_states` và `organization_role_capabilities_and_profile_history_persist_after_reopen`.
