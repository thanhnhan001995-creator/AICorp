# T-01 — Fix01

- **Grouped source:** T-01.02-Fix01.1.md, T-01.03-Fix01.2.md, T-01.03-Fix01.3.md, T-01.04-Fix01.4.md, T-01.05-Fix01.5.md


---

# T-01.02-Fix01.1

- **Source report:** [Executive index](../../README.md)

## EXE-Fix01.1 — T-01.02-Fix01.1: Bảo toàn tuyến báo cáo và tham chiếu runtime

- **Status:** Implemented.
- **Kết quả:** Rust hiện từ chối nhân viên tự báo cáo cho chính mình và mọi gán manager tạo chu trình gián tiếp. Đường tạo/cập nhật nhân sự dùng cùng kiểm tra trước khi ghi SQLite. `connectionReference` hiện chỉ là reference hiển thị an toàn của runtime mẫu; không có secret/API key trong profile, UI hoặc database schema hiện hành.
- **Bằng chứng tự động:** `cargo test` pass. Test tạo nhân viên thứ hai, kiểm tra self-cycle và indirect-cycle bị từ chối, đồng thời kiểm tra manager `null` hợp lệ.
- **Lỗi còn lại:** Không có lỗi tự động đã biết; kiểm tra UI runtime hiện hành vẫn cần CEO thực hiện.

================================================================================
Đã nghiệm thu. Xác nhận hoàn thành J-01. Đóng Task từ T-01.01 đến T-01.05.


 ============================================= Phân chia EXE 01 và EXE 02 ==================================================================

================================================================================

# Fix02 — Kết quả triển khai kỹ thuật J-01

- **Evidence:** [`envidence-T-01-Fix02.md`](../../Evidence/J01/envidence-T-01-Fix02.md) — `AIC-EVD-FIX02-20261107-01`.
- **Boundary:** Các entry dưới đây chỉ xác nhận implementation/build/test. Runtime/UI do CEO kiểm tra ở vòng hồi tố tiếp theo.




---

# T-01.03-Fix01.2

- **Source report:** [Executive index](../../README.md)

## EXE-Fix01.2 — T-01.03-Fix01.2

- **Status:** Implemented.
- **Kết quả:** Thêm policy version theo nhân viên/chức vụ, scope, hạn, quota, delegate, self-approve, lý do và `supersedesId`; tạo/thu hẹp/thu hồi giữ lịch sử.
- **Bằng chứng:** Rust test xác nhận policy cũ chuyển `narrowed` và policy kế nhiệm tồn tại.




---

# T-01.03-Fix01.3

- **Source report:** [Executive index](../../README.md)

## EXE-Fix01.3 — T-01.03-Fix01.3

- **Status:** Implemented.
- **Kết quả:** Evaluator kiểm tra employee/chức vụ, work/action/resource, hạn, quota, delegate và self-approve, trả allow/deny có lý do.
- **Bằng chứng:** Test pass case hợp lệ và từ chối sai work/resource, quá quota, self-approve chưa cấp.




---

# T-01.04-Fix01.4

- **Source report:** [Executive index](../../README.md)

## EXE-Fix01.4 — T-01.04-Fix01.4

- **Status:** Implemented.
- **Kết quả:** UI policy có form cho toàn bộ field, tạo/thu hẹp/thu hồi và hiển thị status/version theo nhân sự.
- **Bằng chứng:** `npm run build` pass; UI chỉ gọi Rust commands.




---

# T-01.05-Fix01.5

- **Source report:** [Executive index](../../README.md)

## EXE-Fix01.5 — T-01.05-Fix01.5

- **Status:** Implemented.
- **Kết quả:** Test bao phủ persistence, reporting cycle, policy history và các từ chối quyền chính; checklist CEO nằm riêng trong `AcceptanceReport.md`.
- **Bằng chứng:** `cargo test` và `npm run build` pass. Runtime/UI cần được CEO kiểm tra theo AR-J01-Fix01.

================================================================================

 


