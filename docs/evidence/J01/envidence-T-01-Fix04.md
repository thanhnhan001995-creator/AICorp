# Evidence kỹ thuật — Fix04 J-01

- **Evidence ID:** `AIC-EVD-FIX04-20261107-01`
- **Scope:** `T-01.03-Fix04.1` đến `T-01.05-Fix04.7`.
- **Boundary:** evidence kỹ thuật; không phải CEO runtime/UI verification hoặc AcceptanceReport.

## Kiểm tra đã chạy

| Lệnh / kiểm tra | Kết quả | Phạm vi chứng minh |
| --- | --- | --- |
| `npm exec tsc -- -p tsconfig.app.json --noEmit` tại `app` | Pass. | TypeScript của UI policy inspector, history và request form không có lỗi kiểu. |
| `rg` kiểm tra command legacy trong `app/src-tauri/src/lib.rs` | Không còn kết quả cho `create_authority_envelope`, `revoke_authority_envelope`, `evaluate_authority`, `list_authority_envelopes`. | Không còn command legacy được đăng ký/giữ trong source runtime. |
| `rg` kiểm tra trigger/history/UI | Có `authority_policy_exactly_one_subject_*`, `authority_policy_events`, `list_authority_policy_events`, request `requiresDelegate`/`requiresSelfApprove`, và `LỊCH SỬ POLICY`. | Các artifact mã cần cho D-04-02, D-04-03 và D-04-06 tồn tại. |
| `rg` tài liệu J-01 | Không còn `Tasks của J-02` hoặc `../../README.md` trong `docs/Tasks/J01`. | Sửa D-04-07 ở các bundle J-01. |

## Regression Rust chưa chạy trong lượt này

`cargo test -q` tại `app/src-tauri` không khởi động vì shell hiện tại trả về: `cargo: The term 'cargo' is not recognized`. Đây là thiếu toolchain/PATH của môi trường review, không được coi là pass hoặc fail của test.

Các test đã bổ sung trong `app/src-tauri/src/lib.rs` cần chạy khi `cargo` khả dụng:

1. subject chính xác một và role isolation;
2. decision metadata, quota/scope và employee override;
3. expiry/revoke cùng legacy migration một lần;
4. narrow giữ subject, event history và transaction rollback;
5. runtime label secret-like và ledger append-only.

## Artefact chính

- `app/src-tauri/src/lib.rs`: canonical policy path, XOR subject trigger/API guard, migration legacy, policy history append-only, evaluator query và regression cases.
- `app/src/App.tsx`: form kiểm tra request thật, kết quả decision có metadata, lịch sử policy, lý do revoke bắt buộc.
- `app/src/App.css`: trạng thái decision/history.
- `docs/contracts/authority-precedence-contract.md`: contract Fix04.

## Checklist runtime/UI dành cho CEO sau khi regression Rust pass

1. Tạo role policy và employee override cùng action; kiểm tra request trong scope và ngoài scope.
2. Kiểm tra quota, delegate và self-approve cả allow/deny.
3. Thu hồi policy với lý do; kiểm tra history có thời điểm/lý do/policy liên quan.
4. Thu hẹp policy; kiểm tra successor/history và thử đổi subject bị từ chối.
5. Xác nhận UI runtime không hiển thị technical reference/credential.
