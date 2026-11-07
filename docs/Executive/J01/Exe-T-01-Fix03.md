# Exe-T-01 — Fix03

- **Tasks:** [`T-01-Fix03.md`](../../Tasks/J01/T-01-Fix03.md)
- **Evidence:** [`envidence-T-01-Fix03.md`](../../Evidence/J01/envidence-T-01-Fix03.md)
- **Boundary:** chỉ ghi kết quả triển khai kỹ thuật; CEO acceptance chưa thực hiện.

---

## EXE-Fix03.1 — T-01.03-Fix03.1

- **Status:** Implemented.
- **Kết quả:** `is_narrower` bắt buộc policy kế nhiệm giữ nguyên employee/role subject; transition và successor vẫn cùng transaction.
- **Bằng chứng:** regression có assertion đổi employee sang role bị từ chối.

## EXE-Fix03.2 — T-01.03-Fix03.2

- **Status:** Implemented.
- **Kết quả:** Hợp đồng precedence được lưu trong [`authority-precedence-contract.md`](../../contracts/authority-precedence-contract.md). Employee policy cùng action là override giới hạn; role policy chỉ xét khi không có employee override cùng action.

## EXE-Fix03.3 — T-01.03-Fix03.3

- **Status:** Implemented.
- **Kết quả:** Decision có allow/deny, `reasonCode`, policy ID/source và matched limits. Regression gồm wildcard, quota, expiry, revoke, delegate/self-approve, subject và precedence cases.

## EXE-Fix03.4 — T-01.04-Fix03.4

- **Status:** Implemented.
- **Kết quả:** UI có source/history policy và khu vực kiểm tra quyền hiệu lực, hiển thị decision metadata; runtime DTO không chứa connection reference.

## EXE-Fix03.5 — T-01.05-Fix03.5

- **Status:** Implemented — technical evidence complete; CEO runtime verification pending.
- **Kết quả:** React build và Rust regression pass; evidence/handoff cho review lần 4 lưu riêng, không tuyên bố CEO acceptance.
