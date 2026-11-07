# Kiểm tra hồi quy J-01 — Lần 6

- **Review ID:** `AIC-REV-J01-20261107-06`
- **Date:** 2026-11-07 (Asia/Bangkok)
- **Scope:** J-01, T-01 gốc và Fix01–Fix05.
- **Yêu cầu đối chiếu:** [Brief](../../brief.md) `AIC-BRF-20261105-01`, mục 4–5, 7–10, 13; [Ideas](../../ideas.md) I-02/I-03; [Jobs](../../Jobs.md) J-01.
- **Nguồn triển khai/evidence:** [T-01 Fix05](../../Tasks/J01/T-01-Fix05.md), [Execute Fix05](../../Executive/J01/Exe-T-01-Fix05.md), [evidence Fix05](../../Evidence/J01/envidence-T-01-Fix05.md), [contract precedence](../../contracts/authority-precedence-contract.md), [contract runtime](../../contracts/runtime-binding-contract.md).
- **Ranh giới:** review kỹ thuật đã có build/regression chạy thực; CEO runtime/UI verification và acceptance vẫn là bước riêng.

## 1. Đối chiếu Brief, Ideas và J-01

| Yêu cầu | Bằng chứng chạy thực / implementation | Kết quả |
| --- | --- | --- |
| Nhân viên có identity dài hạn, role, runtime, tuyến báo cáo, trạng thái và năng lực do CEO khai báo; role tách runtime. | SQLite `employees`/`roles`/`runtime_bindings`, UI hồ sơ, regression persistence organization/role/capabilities. | Đạt kỹ thuật. |
| CEO duy trì tổ chức đa chức năng thay vì bị khóa vào role demo. | API/UI update tổ chức; create/update role; create/update employee; role mới được kiểm chứng persistence. | Đạt kỹ thuật trong phạm vi một tổ chức local-first. |
| Tuyến quản lý không vòng lặp. | `ensure_valid_manager` và regression giữ từ các lượt trước. | Đạt kỹ thuật. |
| Hộp uỷ quyền thể hiện action, work/resource scope, hạn, quota, delegate, self-approve và source. | `authority_policy_versions`, UI policy form/list, evaluator structured decision. | Đạt kỹ thuật. |
| Quyền theo employee/role có precedence rõ, subject không mơ hồ và không có đường legacy song song. | XOR trigger/API, canonical evaluator, migration legacy transaction, regression role/override/rollback. | Đạt kỹ thuật. |
| CEO cấp/thu hẹp/thu hồi và xem history quyền. | Grant/narrow/revoke transaction + `authority_policy_events` append-only; UI history; regression 8/8. | Đạt kỹ thuật. |
| Dữ liệu runtime nhạy cảm không ra UI; không cho persistence nhãn secret-like. | Safe DTO, seed-only immutable boundary, contract runtime, regression reject `sk-secret`. | Đạt kỹ thuật trong phạm vi runtime J-01 seed-only. |
| Hồ sơ mới phân biệt trạng thái lịch sử rỗng với policy history/experience. | `employee_profile_events`, UI **LỊCH SỬ HỒ SƠ**, regression persistence. | Đạt kỹ thuật. |
| Mọi thay đổi quan trọng liên quan policy/hồ sơ có history đọc được. | Policy events append-only; update hồ sơ ghi event trong transaction. | Đạt kỹ thuật trong phạm vi J-01; J-02 vẫn là ledger tổ chức bất biến rộng hơn. |

## 2. Kết quả verification kỹ thuật

| Kiểm tra | Kết quả |
| --- | --- |
| Rust regression — `cargo test -q` | Pass: **8/8**. |
| Frontend production — `npm run build` | Pass. |
| TypeScript check | Pass. |
| Verification failure đã được sửa | Migration borrow E0502 và collision ID SQLite test song song; final regression pass. |

## 3. Ranh giới còn lại, không phải sai lệch J-01

1. AI Corp hiện chủ đích hỗ trợ **một tổ chức local-first**; đa tổ chức/đa chi nhánh thuộc phạm vi sau, không phải thiếu sót J-01.
2. Runtime bindings ở J-01 là **seed-only/immutable**; kết nối runtime thật và quản trị credential là phạm vi J-03, theo contract runtime.
3. `employee_profile_events` là history hồ sơ tối thiểu; kinh nghiệm cá nhân có nguồn vẫn thuộc J-08, còn ledger tổ chức bất biến thuộc J-02.

## 4. Kết luận regression J-01

**J-01: `Technically ready for CEO runtime/UI verification — not yet CEO acceptance`.**

Không còn sai lệch kỹ thuật trọng yếu đã biết so với Brief/I-02/I-03/J-01. Không tạo Fix06 ở thời điểm này. Bước còn lại là CEO chạy UI và kiểm tra hành vi thực; chỉ sau đó mới lập AcceptanceReport hoặc ghi nhận issue mới.

## 5. Checklist CEO runtime/UI

1. Đổi tên tổ chức, tạo hai chức vụ và tạo/cập nhật ít nhất hai nhân viên có năng lực khai báo, runtime, tuyến báo cáo và trạng thái khác nhau.
2. Mở hồ sơ nhân viên mới để xác nhận dòng “chưa có lịch sử”; cập nhật hồ sơ rồi xác nhận event lịch sử có thời điểm/nguồn.
3. Cấp role policy và employee override cùng action; kiểm tra allow/deny với scope/quota/delegate/self-approve qua **QUYỀN HIỆU LỰC**.
4. Thu hẹp policy, thu hồi với lý do, kiểm tra **LỊCH SỬ POLICY** và `supersedesId`.
5. Xác nhận UI runtime chỉ hiển thị tên/provider/status, không có technical reference/credential.

Nếu CEO phát hiện hành vi khác checklist, ghi lại bước tái lập và kết quả quan sát; đó là đầu vào cho Fix06, không sửa review/Execute lịch sử.
