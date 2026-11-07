# Evidence kỹ thuật — Fix02 J-01

- **Evidence ID:** `AIC-EVD-FIX02-20261107-01`
- **Date:** 2026-11-07 (Asia/Bangkok)
- **Scope:** `T-01.02-Fix02.1` đến `T-01.05-Fix02.6`.
- **Boundary:** Đây là evidence kỹ thuật của nhóm triển khai. Nó không là CEO acceptance và không thay thế thao tác runtime/UI của CEO.

## Kết quả tự động

| Kiểm tra | Lệnh | Kết quả |
| --- | --- | --- |
| React/TypeScript production build | `npm run build` trong `app` | Pass. |
| Rust regression | `cargo test -q` trong `app/src-tauri` | Pass: 1 test. |

## Assertions được Rust test

- SQLite persistence cho tổ chức/nhân viên/policy.
- Từ chối self-cycle và indirect-cycle reporting line.
- Policy version cũ được giữ trạng thái `narrowed`.
- Allow đúng wildcard child scope khi quota/delegate phù hợp.
- Deny sai work scope, sai resource scope, vượt quota, self-approve thiếu quyền.
- Deny policy hết hạn và policy đã bị revoke.
- Ledger append-only và correction link vẫn pass regression.

## Kiểm tra tĩnh / contract

- `RuntimeBinding` DTO từ Rust sang UI không còn `connectionReference`; query UI chỉ trả ID, provider, display name, status.
- `authority_policy_versions` tách subject employee/role, có scope/expiry/quota/delegate/self-approve/status/supersedes/reason.
- `narrow_authority_policy` đọc policy active, kiểm tra tập con và dùng transaction cho update + insert kế nhiệm.
- UI policy có source employee/role, các giới hạn, reason, trạng thái, history predecessor và thao tác create/narrow/revoke.

## Handoff runtime/UI cho CEO

CEO cần mở bản app mới và kiểm tra theo [`Tasks.md`](../../Tasks.md), Fix02.1–.6: không thấy runtime reference nhạy cảm; tạo policy theo role; thử thu hẹp bằng quyền rộng hơn để thấy bị chặn; xem history/source/status; thử policy hết hạn/revoke. Kết quả CEO quan sát là evidence runtime, được ghi sau vòng kiểm tra hồi tố tiếp theo, không ghi là acceptance trong file này.
