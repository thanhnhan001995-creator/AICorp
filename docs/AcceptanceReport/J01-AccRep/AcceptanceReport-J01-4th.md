# Acceptance Report — J-01 — Lượt 4

- **Report ID:** `AIC-ACC-J01-20261107-04`
- **Status:** `Accepted` — CEO đã kiểm tra runtime/UI.
- **Lịch sử:** [Lượt 1](AcceptanceReport-J01-1st.md) và [lượt 2](AcceptanceReport-J01-2nd.md) lỗi khởi chạy; [lượt 3](AcceptanceReport-J01-3rd.md) `Rejected` tại thu hồi policy. Các biên bản trước được giữ nguyên.
- **Technical gate:** [Review J-01 lần 7](../../reviews/J01/j01-retrospective-verification-07.md) — Rust regression 9/9, TypeScript check và production build pass.
- **Evidence:** [Fix08](../../evidence/J01/envidence-T-01-Fix08.md).

## 1. Khởi chạy bằng Command Prompt

Mở **Command Prompt** (`cmd.exe`) và chạy:

```cmd
cd /d D:\Moi-Truong_AI\app
set "PATH=%PATH%;C:\Users\Owner\.cargo\bin"
npm exec -- tauri dev
```

Giữ cửa sổ Command Prompt mở trong lúc kiểm tra. Nếu app không mở, dừng checklist và ghi nguyên văn lỗi trong kết luận.

## 2. Checklist thu hồi policy

| # | Thao tác CEO | Kết quả cần thấy | Ghi nhận CEO |
| ---: | --- | --- | --- |
| 1 | Chọn một nhân viên và cấp policy mới, có lý do. | Policy xuất hiện với status `active`. | |
| 2 | Bấm **Thu hẹp**, giữ cùng chủ thể/action, giới hạn scope hoặc quota và lưu. | Policy gốc thành `narrowed`; policy mới `active`, có thông tin “từ {policy gốc}”. | |
| 3 | Nhập **Lý do thu hồi**, sau đó bấm **Thu hồi** tại policy kế nhiệm đang `active`. | Có thông báo thành công; policy kế nhiệm thành `revoked`. | |
| 4 | Xem **LỊCH SỬ POLICY**. | Có `authority.narrowed`, `authority.granted` cho successor và `authority.revoked` với lý do CEO vừa nhập. | |
| 5 | Nếu thử thao tác trên policy `narrowed`/`revoked` qua dữ liệu cũ hoặc tình huống có thể tái lập. | Thông báo có policy ID/status và nêu chỉ policy `active` được thu hồi; không có dữ liệu bị sửa đè. | |

## 3. Kết luận CEO

- **Kết quả:** `Accepted`
- **CEO:** Nguyễn Thành Nhân    
- **Evidence runtime:** ảnh/video, policy ID/status, hoặc log lỗi:
- **Issue cần Fix tiếp theo (nếu có):** bước tái lập, mong đợi, quan sát:

Chỉ đổi kết quả thành `Accepted` khi toàn bộ checklist đạt. Nếu bất kỳ bước nào fail, đổi thành `Rejected`, giữ nguyên report này và ghi policy ID/status cùng nguyên văn thông báo để tạo Fix tiếp theo.
