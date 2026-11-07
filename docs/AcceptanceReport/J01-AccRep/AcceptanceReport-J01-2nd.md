# Acceptance Report — J-01 — Lượt 2

- **Report ID:** `AIC-ACC-J01-20261107-02`
- **Status:** `Not reviewed` — chờ CEO chạy kiểm chứng thực tế.
- **Lý do mở lượt 2:** [AcceptanceReport-J01-1st](AcceptanceReport-J01-1st.md) bị `Rejected` trước khi UI khởi động vì hướng dẫn cũ gọi npm script không tồn tại. Biên bản lần 1 được giữ nguyên làm evidence lịch sử.
- **Technical gate:** [Review J-01 lần 6](../Reviews/J01/j01-retrospective-verification-06.md) — Rust regression 8/8 và production frontend build pass.
- **Launch evidence:** [Fix06](../Evidence/J01/envidence-T-01-Fix06.md).

## 1. Khởi chạy bằng Command Prompt

Mở **Command Prompt** (`cmd.exe`), rồi chạy lần lượt:

```cmd
cd /d D:\Moi-Truong_AI\app
set "PATH=%PATH%;C:\Users\Owner\.cargo\bin"
npx tauri dev
```

Giữ Command Prompt mở trong lúc kiểm tra. Nếu lệnh không mở app, dừng checklist và ghi nguyên văn lỗi vào phần kết luận CEO. Không đổi report lần 1.

## 2. Checklist CEO

| # | Thao tác | Kết quả cần thấy | Ghi nhận CEO |
| ---: | --- | --- | --- |
| 1 | Đổi tên tổ chức/tên CEO tại **TỔ CHỨC**, lưu, rồi mở lại app. | Header và dữ liệu local vẫn giữ tên mới. | Pass / Fail; ghi chú. |
| 2 | Tạo hai chức vụ, sửa mô tả một chức vụ. | Role xuất hiện trong danh sách và dropdown hồ sơ. | Pass / Fail; role đã dùng. |
| 3 | Tạo hai nhân viên thuộc hai role, đặt runtime, năng lực, manager và status khác nhau. | Hồ sơ hiển thị đúng các field; role độc lập runtime. | Pass / Fail; nhân viên đã dùng. |
| 4 | Mở nhân viên mới, sau đó cập nhật hồ sơ. | Ban đầu báo lịch sử rỗng; sau cập nhật có event hồ sơ với thời điểm/nguồn. | Pass / Fail; ghi chú. |
| 5 | Cấp policy role `execute`, `work/*`, `local_workspace/*`, quota 10, delegate/self-approve. | Policy hiển thị source/scope/quota/cờ quyền. | Pass / Fail; policy ID. |
| 6 | Cấp employee override hẹp; thử request trong và ngoài scope qua **QUYỀN HIỆU LỰC**. | Trong scope allow từ employee; ngoài scope deny `employee_override_denied`, không fallback role. | Pass / Fail; input/decision. |
| 7 | Thử quota vượt giới hạn, delegate và self-approve. | Decision nêu allow/deny, reason code, policy/source/limits. | Pass / Fail; ghi chú. |
| 8 | Thu hẹp policy, rồi thu hồi với lý do. | `supersedesId` và **LỊCH SỬ POLICY** thể hiện grant/narrow/revoke. | Pass / Fail; policy ID. |
| 9 | Kiểm tra runtime/history/decision hiển thị. | Không có connection reference, token hoặc credential. | Pass / Fail; ghi chú. |
| 10 | Đóng/mở lại app. | Organization, role, nhân viên, policy và history vẫn tồn tại. | Pass / Fail; ghi chú. |

## 3. Kết luận CEO

- **Kết quả:** `Rejected`
- **Ngày kiểm tra:** `2026/11/07 20:11`
- **CEO:**
- **Evidence runtime:** ảnh, video, ID nhân viên/policy hoặc log lỗi:
npm error could not determine executable to run
npm error A complete log of this run can be found in: C:\Users\Owner\AppData\Local\npm-cache\_logs\2026-11-07T13_10_05_542Z-debug-0.log
- **Issue cần Fix tiếp theo (nếu có):** bước tái lập, mong đợi, quan sát: Không thể khởi chạy từ cmd, báo lỗi như trên evidence

Chỉ khi CEO ghi `Accepted` mới coi J-01 được nghiệm thu. Nếu fail, tạo Fix mới từ issue và giữ nguyên biên bản lần 1/lần 2.
