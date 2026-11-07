# Evidence kỹ thuật — Fix03 J-01

- **Evidence ID:** `AIC-EVD-FIX03-20261107-01`
- **Scope:** `T-01.03-Fix03.1` đến `T-01.05-Fix03.5`.
- **Boundary:** evidence kỹ thuật, không phải CEO acceptance.

## Commands

| Command | Result |
| --- | --- |
| `npm run build` in `app` | Pass. |
| `cargo test -q` in `app/src-tauri` | Pass: 1 regression test. |

## Assertions

- `is_narrower` yêu cầu cùng `employeeId`/`roleId`; attempt đổi subject bị từ chối.
- `narrow_authority_policy` dùng transaction để narrow cũ + ghi successor.
- Employee policy cùng action tạo override boundary; role policy không fallback khi employee override giới hạn request.
- Khi employee policy bị revoke, role policy hợp lệ có thể được xét lại.
- Decision trả `reasonCode`, `policyId`, `policySource`, `matchedLimits`.
- UI có khu vực **QUYỀN HIỆU LỰC**, cho phép kiểm tra policy active và xem allow/deny cùng metadata decision.

## CEO runtime handoff

CEO mở app mới, tạo role policy và employee override cùng action, kiểm tra request ngoài scope employee override bị deny; thu hồi override rồi kiểm tra role policy được phép. Evidence runtime này dùng cho review lần 4, không ghi acceptance ở đây.
