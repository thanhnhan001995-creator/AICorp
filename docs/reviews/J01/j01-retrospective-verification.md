# Đối chiếu và nghiệm thu hồi tố — J-01

- **Review ID:** `AIC-REV-J01-20261106-01`
- **Date:** 2026-11-06 (Asia/Bangkok)
- **Scope:** `T-01.01` đến `T-01.05`; đối chiếu [`brief.md`](../../brief.md) mục 5, 7, 11, 13; [`ideas.md`](../../ideas.md) I-02, I-03; [`Jobs.md`](../../Jobs.md) J-01.
- **Method:** đọc hợp đồng/code/UI hiện có và chỉ công nhận bằng chứng đã quan sát hoặc có test; không suy diễn từ build thành runtime proof.

| Task | Đối chiếu Brief/Ideas | Kết luận | Sai lệch hoặc giới hạn cần giữ lại |
| --- | --- | --- | --- |
| T-01.01 | I-02 yêu cầu identity bền vững, role tách runtime; I-03 yêu cầu quyền có giới hạn. | **Đúng.** Contract nêu ID prefix, entity tách biệt, ví dụ hợp lệ/bị từ chối. | Contract nói tuyến báo cáo không chu trình, nhưng code chưa thực thi kiểm tra chu trình. |
| T-01.02 | Brief local-first; I-02 yêu cầu profile, role, runtime, reporting/status. | **Đúng một phần.** SQLite Rust-owned, UI chỉ `invoke`, dữ liệu employee/role/runtime/status bền vững đã có. | `create/update_employee` chưa chặn manager cycle; role/runtime chỉ đọc từ mẫu, chưa có CEO CRUD; runtime reference đang hiển thị ở UI, nên chưa phù hợp nếu sau này nó mang secret. |
| T-01.03 | I-03 cần chức vụ/công việc/phạm vi/thời gian/ngân sách, quyền thao tác/delegate/self-approve. | **Sai lệch đáng kể.** Backend có action/scope/time/budget và revoke/evaluate cơ bản. | Chưa có quyền theo chức vụ; chưa có quyền delegate/self-approve trong schema/UI; không có “thu hẹp” envelope; evaluator chưa kiểm tra delegation/self-approval hay work ID thật. Không thể gọi là hoàn thành đầy đủ. |
| T-01.04 | CEO tạo/sửa profile và xem/cấp/thu quyền, không suy quyền từ local/API. | **Đúng một phần.** CEO UI tạo/sửa profile, thấy role/runtime/manager/status, cấp/thu quyền mẫu; local runtime không tự có quyền. | UI chỉ cấp mẫu `prepare_report`; CEO chưa nhập/sửa phạm vi, hạn, budget, delegate/self-approve. Chưa có bằng chứng runtime sau bản UI mới nhất. |
| T-01.05 | Phải kiểm tra identity, reporting line, quyền không vượt phạm vi và giao diện tối thiểu. | **Sai lệch đáng kể.** Có 1 Rust test persistence + ledger trigger sau này, và đã quan sát UI cũ. | Chưa có test ID invalid, reporting cycle, authority scope/time/budget/revoke; chưa có kiểm tra UI CEO thao tác toàn bộ uỷ quyền. Không đủ điều kiện “CEO-verified”. |

## Kết luận J-01

J-01 nên có trạng thái **Implemented with deviations — cần sửa trước CEO-verified**, không phải “đã nghiệm thu hoàn toàn”. Những phần đạt là nền tảng profile/local-first và hợp đồng miền. Các sai lệch T-01.03/T-01.05 phải được xử lý bằng Task sửa lỗi/bổ sung trước khi J-03 dựa vào quyền để chạy agent thật.

## Mẫu xác nhận CEO cho mỗi Task

Mỗi Task trong J-01 dùng cùng ba trạng thái: `Not reviewed` / `CEO-verified` / `Rejected or needs follow-up`. CEO chỉ đánh dấu `CEO-verified` khi đã làm đúng checklist thao tác và xem bằng chứng đính kèm.

| Task | Trạng thái CEO | Ngày / chữ ký CEO | Ghi chú hoặc ID follow-up |
| --- | --- | --- | --- |
| T-01.01 | Not reviewed | — | — |
| T-01.02 | Not reviewed | — | — |
| T-01.03 | Needs follow-up | — | quyền theo chức vụ/delegate/self-approve/thu hẹp còn thiếu |
| T-01.04 | Needs follow-up | — | UI hộp uỷ quyền đầy đủ còn thiếu |
| T-01.05 | Needs follow-up | — | test/UI verification còn thiếu |
