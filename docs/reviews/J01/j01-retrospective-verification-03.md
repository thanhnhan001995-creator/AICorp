# Kiểm tra hồi tố J-01 — Lần 3

- **Review ID:** `AIC-REV-J01-20261107-03`
- **Date:** 2026-11-07 (Asia/Bangkok)
- **Scope:** J-01, Fix01 và Fix02 kỹ thuật.
- **Nguồn:** [`brief.md`](../brief.md) mục 5, 7, 11, 13; [`ideas.md`](../ideas.md) I-02/I-03; [`Jobs.md`](../Jobs.md) J-01; evidence [`envidence-T-01-Fix02.md`](../Evidence/J01/envidence-T-01-Fix02.md).
- **Ranh giới:** review này đánh giá mã/evidence kỹ thuật mới nhất; CEO runtime/UI verification vẫn chưa thực hiện và không bị suy diễn từ build/test.

## 1. Những phần hiện khớp Brief và Ideas

| Yêu cầu | Bằng chứng | Trạng thái kỹ thuật |
| --- | --- | --- |
| Identity bền vững, role tách runtime, local-first. | Contract + Rust SQLite/Tauri + React invoke. | Đạt. |
| Không tạo chu trình quản lý. | `ensure_valid_manager` + regression self/indirect-cycle. | Đạt. |
| UI runtime không nhận technical connection reference. | `RuntimeBinding` DTO/query chỉ còn ID/provider/display/status. | Đạt theo code path hiện tại. |
| Policy có scope, hạn, quota, delegate, self-approve, source và lịch sử. | `authority_policy_versions`; UI policy; test. | Đạt về data model. |
| Revoke/expiry/scope/quota được evaluator xét. | `evaluate_policy_from` + regression. | Đạt một phần, xem sai lệch dưới đây. |
| Policy theo chức vụ có thể tạo từ UI. | Source selector + `roleId` in UI/API. | Đạt về implementation; chưa có runtime CEO evidence. |

## 2. Sai lệch còn lại

### D-03-01 — Thu hẹp policy có thể đổi chủ thể quyền

`narrow_authority_policy` gọi `is_narrower` chỉ so action/scope/quota/hạn/delegate/self-approve. Nó không bắt buộc `employeeId` và `roleId` của policy mới giống policy cũ. Vì vậy một policy theo nhân viên có thể bị “thu hẹp” thành policy theo role (hoặc ngược lại), làm thay đổi tập người nhận quyền thay vì chỉ giảm quyền.

**Tác động:** trái với I-03 và T-01.03-Fix02.3: thao tác thu hẹp phải là tập con quyền cũ và không được mở rộng chủ thể.

### D-03-02 — Override nhân viên không chặn fallback role rộng hơn

Evaluator sắp employee policy trước role policy, nhưng nếu employee policy không khớp request, vòng lặp tiếp tục xét role policy. Một override hẹp vì thế không có hiệu lực hạn chế trước policy role rộng hơn; nó chỉ là một grant ưu tiên. Không có quy tắc deny/override boundary rõ ràng.

**Tác động:** chưa đáp ứng yêu cầu Fix02.4 về precedence role–employee override rõ ràng và I-03 về quyền theo ngữ cảnh có giới hạn.

### D-03-03 — Thiếu coverage/hiển thị quyết định hiệu lực

- Test chưa có case policy role áp dụng đúng/sai nhân viên; positive self-approve; positive delegate; và từ chối đổi subject khi narrow.
- UI hiển thị reason cấp policy, không hiển thị kết quả evaluator/lý do hiệu lực của một request cụ thể. Vì vậy T-01.04-Fix02.5 chỉ đạt một phần.

### D-03-04 — Evidence CEO runtime/UI chưa có

Evidence Fix02 chỉ có build/test/kết luận tĩnh. CEO chưa chạy bản app mới để xác minh DTO runtime an toàn, role policy, narrow rejection, history/status và behavior UI.

## 3. Kết luận đối chiếu Job 1

**J-01: `Implemented with residual deviations — not ready for CEO acceptance`.**

Fix02 hoàn thành technical implementation theo `execute.md`, nhưng output vẫn chưa khớp hoàn toàn Brief/Ideas vì D-03-01 và D-03-02 là lỗi semantics quyền trọng yếu. Trước khi dùng Hộp uỷ quyền làm ranh giới cho agent thật (J-03), cần một lượt Fix tiếp theo xử lý các sai lệch này, sau đó thực hiện review lại. Acceptance Report vẫn để trống.

## 4. Evidence cần giữ cho lần review sau

1. Regression riêng chứng minh narrow không đổi subject và không mở rộng principal set.
2. Hợp đồng precedence/override: khi nào employee override chỉ bổ sung, khi nào nó giới hạn/chặn role grant; test mỗi luật.
3. API/UI trả được effective-decision có policy ID, source, allow/deny và reason.
4. Test role policy đúng/sai employee, positive delegate/self-approve, expiry/revoke.
5. CEO runtime/UI evidence sau khi review kỹ thuật không còn sai lệch trọng yếu.
