# Acceptance Report — J-01 — Lượt 3

- **Report ID:** `AIC-ACC-J01-20261107-03`
- **Status:** `Not reviewed` — chờ CEO runtime/UI verification.
- **Lịch sử:** [Lượt 1](AcceptanceReport-J01-1st.md) `Rejected` do npm script không tồn tại; [lượt 2](AcceptanceReport-J01-2nd.md) `Rejected` do `npx` không chọn được executable. Hai biên bản được giữ nguyên.
- **Technical gate:** [Review J-01 lần 6](../Reviews/J01/j01-retrospective-verification-06.md), Rust regression 8/8 và production build pass.
- **Command evidence:** [Fix07](../Evidence/J01/envidence-T-01-Fix07.md).

## 1. Khởi chạy bằng Command Prompt

Mở **Command Prompt** (`cmd.exe`) và chạy:

```cmd
cd /d D:\Moi-Truong_AI\app
set "PATH=%PATH%;C:\Users\Owner\.cargo\bin"
npm exec -- tauri dev
```

Đây là lệnh đã xác minh local CLI. Giữ Command Prompt mở khi kiểm tra. Nếu app không mở, dừng checklist và ghi nguyên văn lỗi ở kết luận CEO.

## 2. Checklist CEO

| # | Kiểm tra | Kết quả cần thấy | Ghi nhận CEO |
| ---: | --- | --- | --- |
| 1 | Đổi tên tổ chức/tên CEO, lưu và mở lại app. | Dữ liệu tổ chức local được giữ. | Pass  |
| 2 | Tạo hai role, sửa mô tả một role. | Role hiện trong danh sách và dropdown hồ sơ. | Pass  |
| 3 | Tạo hai nhân viên khác role; đặt runtime, capabilities, manager, status. | Hồ sơ hiển thị đúng mọi field; role không phụ thuộc runtime. | Pass |
| 4 | Mở nhân viên mới rồi cập nhật hồ sơ. | Có trạng thái lịch sử rỗng trước cập nhật và event hồ sơ sau lưu. | Pass |
| 5 | Cấp role policy `execute`/`work/*`/`local_workspace/*`, quota 10, delegate/self-approve. | Policy source/scope/quota/cờ quyền đúng. | Pass |
| 6 | Cấp employee override hẹp; thử request trong và ngoài scope tại **QUYỀN HIỆU LỰC**. | Allow từ employee trong scope; ngoài scope deny `employee_override_denied`. | Pass |
| 7 | Thử quota vượt giới hạn, delegate và self-approve. | Decision có allow/deny, reason code, source và limits. | Pass |
| 8 | Thu hẹp rồi thu hồi policy có lý do. | `supersedesId` và lịch sử grant/narrow/revoke đúng. | Fail; "không thể thu hồi policy" |
| 9 | Rà soát runtime/history/decision. | Không có connection reference, token hay credential. | Pass |
| 10 | Đóng/mở app rồi kiểm tra lại dữ liệu. | Organization, role, employee, policy/history còn nguyên. | Pass |

## 3. Kết luận CEO

- **Kết quả:**  `Rejected`
- **CEO:** Nguyễn Thành Nhân
- **Evidence runtime:** ảnh, video, ID nhân viên/policy hoặc log lỗi:
tất cả các policy
- **Issue cần Fix tiếp theo (nếu có):** bước tái lập, mong đợi, quan sát: không thể thu hồi policy đã cấp.

Chỉ `Accepted` mới hoàn tất nghiệm thu J-01. Nếu fail, giữ nguyên report này và tạo Fix mới từ issue.
