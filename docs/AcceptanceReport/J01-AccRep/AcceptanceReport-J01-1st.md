# Acceptance Report — J-01

- **Report ID:** `AIC-ACC-J01-20261107-01`
- **Job:** J-01 — Nền tảng tổ chức, nhân viên và uỷ quyền.
- **Status:** `Chờ CEO runtime/UI verification` — tài liệu này không tự xác nhận nghiệm thu.
- **Brief / Ideas:** [brief](../brief.md) `AIC-BRF-20261105-01`, mục 4–5, 7–10, 13; [ideas](../ideas.md) I-02/I-03.
- **Technical gate passed:** [Review lần 6](../Reviews/J01/j01-retrospective-verification-06.md); Rust regression 8/8 và production frontend build pass.
- **Evidence:** [Fix05](../Evidence/J01/envidence-T-01-Fix05.md).

## 1. Mục tiêu nghiệm thu

CEO xác nhận AI Corp đã cung cấp đúng nền tảng J-01: một tổ chức local-first có nhân viên AI với danh tính, chức vụ, năng lực khai báo, runtime, tuyến báo cáo, trạng thái, lịch sử hồ sơ và Hộp uỷ quyền theo ngữ cảnh; CEO luôn xem, thu hẹp và thu hồi được quyền.

Không kiểm tra ở report này: agent thật chạy việc (J-03), ledger tổ chức bất biến toàn diện (J-02), công việc phụ (J-04), Command Desk (J-05), văn phòng ảo (J-06) hay kinh nghiệm cá nhân (J-08).

## 2. Tóm tắt các task chính đã làm

| Bundle task | Nội dung đã hiện thực | Bằng chứng kỹ thuật |
| --- | --- | --- |
| [T-01 gốc](../Tasks/J01/T-01.md) | Domain tổ chức/nhân viên/role/runtime; SQLite local-first; hồ sơ CEO; Hộp uỷ quyền và evaluator cơ bản. | [Execute T-01](../Executive/J01/Exe-T-01.md) |
| [Fix01](../Tasks/J01/T-01-Fix01.md) | Bảo vệ tuyến báo cáo, hoàn chỉnh policy theo ngữ cảnh và quyền CEO quản trị policy. | [Execute Fix01](../Executive/J01/Exe-T-01-Fix01.md) |
| [Fix02](../Tasks/J01/T-01-Fix02.md) | Safe runtime DTO; policy role/employee; narrow không mở rộng; policy history/effective decision nền tảng. | [Evidence Fix02](../Evidence/J01/envidence-T-01-Fix02.md) |
| [Fix03](../Tasks/J01/T-01-Fix03.md) | Giữ subject khi narrow; precedence employee override; decision metadata có cấu trúc. | [Evidence Fix03](../Evidence/J01/envidence-T-01-Fix03.md) |
| [Fix04](../Tasks/J01/T-01-Fix04.md) | Canonical policy path, subject XOR, policy history append-only, runtime secret-like guard và request inspector. | [Evidence Fix04](../Evidence/J01/envidence-T-01-Fix04.md) |
| [Fix05](../Tasks/J01/T-01-Fix05.md) | CEO quản trị tổ chức/role/năng lực; lịch sử hồ sơ; policy/migration transaction; runtime seed-only boundary; regression/build. | [Execute Fix05](../Executive/J01/Exe-T-01-Fix05.md), [Evidence Fix05](../Evidence/J01/envidence-T-01-Fix05.md) |

## 3. Chuẩn bị chạy thật

Từ `D:\Moi-Truong_AI\app`, mở **Command Prompt** (`cmd.exe`) và chạy:

```cmd
set "PATH=%PATH%;C:\Users\Owner\.cargo\bin"
npm run tauri dev
```

Giữ cửa sổ terminal mở trong khi kiểm tra. Nếu app không khởi động, ghi nguyên văn lỗi vào cột **Ghi nhận CEO** bên dưới; đó là issue kiểm chứng, không tự sửa report lịch sử.

## 4. Checklist CEO tương tác thật

| # | Thao tác CEO | Kết quả cần thấy | Ghi nhận CEO |
| ---: | --- | --- | --- |
| 1 | Tại **TỔ CHỨC**, đổi tên tổ chức và tên CEO, bấm **Lưu tổ chức**. | Header cập nhật; đóng/mở lại app vẫn giữ tên mới. | Pass / Fail; tên đã dùng; ghi chú. |
| 2 | Tại **CHỨC VỤ**, tạo hai role có mô tả khác nhau; bấm từng role để sửa lại mô tả. | Hai role xuất hiện trong danh sách và dropdown **Chức vụ** của hồ sơ. | Pass / Fail; tên role; ghi chú. |
| 3 | Tạo hai nhân viên, chọn hai role khác nhau; điền runtime, năng lực CEO khai báo, tuyến báo cáo và trạng thái khác nhau. | Mỗi hồ sơ hiển thị đúng tên, role, runtime, manager, status và capabilities; role không bị suy ra từ runtime. | Pass / Fail; ID/tên nhân viên; ghi chú. |
| 4 | Mở nhân viên mới trước khi cập nhật, rồi đổi một field hồ sơ và lưu. | Ban đầu có dòng **Chưa có lịch sử hồ sơ/công việc**; sau lưu có event `employee.profile_updated` với thời điểm/nguồn. | Pass / Fail; nhân viên; ghi chú. |
| 5 | Cấp một policy **theo chức vụ**: action `execute`, work `work/*`, resource `local_workspace/*`, quota 10, bật delegate/self-approve. | Policy hiển thị source role, scope, quota và các cờ quyền đúng. | Pass / Fail; policy ID; ghi chú. |
| 6 | Cấp employee override cùng action nhưng scope hẹp hơn. Trong **QUYỀN HIỆU LỰC**, thử một request trong scope và một request ngoài scope employee nhưng role policy rộng có thể cho. | Request trong scope allow từ employee; request ngoài scope deny `employee_override_denied`, không fallback role. | Pass / Fail; input/decision; ghi chú. |
| 7 | Trong request inspector, thử quota vượt giới hạn, bật `Request cần giao việc phụ`, rồi bật `Request cần tự duyệt`. | Quyết định hiển thị allow/deny, reason code, policy/source/limits; thiếu quyền hoặc quota vượt giới hạn bị deny. | Pass / Fail; các decision; ghi chú. |
| 8 | Thu hẹp một policy hợp lệ, sau đó thu hồi policy active với lý do. | Successor giữ đúng subject và `supersedesId`; **LỊCH SỬ POLICY** có grant/narrow/revoke cùng thời điểm/lý do. | Pass / Fail; policy ID; ghi chú. |
| 9 | Quan sát dropdown/runtime và các history/decision hiển thị. | Chỉ có display name/provider/status; không có `connection_reference`, token hay credential. | Pass / Fail; ghi chú. |
| 10 | Đóng app, mở lại, kiểm tra organization/role/nhân viên/policy/history vừa tạo. | Dữ liệu local vẫn tồn tại; không có vòng manager hoặc quyền đổi bất ngờ. | Pass / Fail; ghi chú. |

## 5. Kết luận CEO

- **Kết quả:**  `Rejected`
- **Ngày kiểm tra:** `2026-11-07 19:57`
- **CEO:** Nguyễn Thành Nhân
- **Evidence runtime đính kèm:** ảnh, video, ID policy/nhân viên hoặc log lỗi:
npm error Missing script: "tauri"
npm error
npm error To see a list of scripts, run:
npm error   npm run
npm error A complete log of this run can be found in: C:\Users\Owner\AppData\Local\npm-cache\_logs\2026-11-07T12_56_51_297Z-debug-0.log
- **Issue cần Fix06 (nếu có):** bước tái lập, kết quả mong đợi, kết quả quan sát: Khi chạy "npm run tauri dev" trong cmd, báo lỗi như trong evidence, không thể kiểm thêm nội dung khác.

Chỉ khi CEO đổi **Kết quả** thành `Accepted` mới coi J-01 đã nghiệm thu. Nếu có fail, giữ report này như biên bản quan sát và tạo task Fix06 từ issue; không sửa đè các review/evidence/Execute trước.
