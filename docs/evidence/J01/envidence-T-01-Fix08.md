# Evidence — T-01 Fix08

- **Evidence ID:** `AIC-EVD-J01-FIX08-20261107-01`
- **Task nguồn:** [`T-01-Fix08.md`](../../Tasks/J01/T-01-Fix08.md)
- **Execute nguồn:** [`Exe-T-01-Fix08.md`](../../Executive/J01/Exe-T-01-Fix08.md)
- **Loại bằng chứng:** source-level, regression, type-check và production build; không phải bằng chứng CEO đã kiểm tra UI/runtime.

## E-08-01 — Luồng thu hồi và trạng thái policy

- **Vị trí:** `D:\Moi-Truong_AI\app\src-tauri\src\lib.rs`, `revoke_authority_policy_from`.
- API chỉ thu hồi policy `active`. Lỗi giờ phân biệt policy không tồn tại với policy `narrowed`/`revoked`, có ID/status và gợi ý chọn successor active.
- Successful revoke ghi `authority.revoked` cùng transaction; event failure làm rollback status về `active`.

## E-08-02 — Regression narrow → revoke

- **Case:** `revoke_successor_records_history_and_rolls_back_when_revoke_event_fails`.
- Kiểm tra parent `narrowed`, successor `revoked`, event/lý do đúng; thử revoke parent trả lỗi có ID/status; ép insert event fail giữ policy thử nghiệm `active` và không có event một phía.
- **Lệnh:**

```cmd
cd /d D:\Moi-Truong_AI\app\src-tauri
C:\Users\Owner\.cargo\bin\cargo.exe test -q
```

- **Kết quả:** `9 passed; 0 failed`.

## E-08-03 — Type-check và build

```cmd
cd /d D:\Moi-Truong_AI\app
npm exec tsc -- -p tsconfig.app.json --noEmit
npm run build
```

- **Kết quả:** type-check pass; Vite build pass, tạo `dist/index.html` và assets production.

## Giới hạn evidence

- AcceptanceReport lượt 3 không có policy ID/raw error, nên không thể chứng minh nguyên nhân duy nhất của phiên CEO đã kiểm.
- Chỉ CEO có thể xác nhận thao tác UI Tauri thật đạt yêu cầu J-01.
