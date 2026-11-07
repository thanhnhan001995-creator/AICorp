# T-01 — Fix02

- **Grouped source:** T-01.02-Fix02.1.md, T-01.03-Fix02.2.md, T-01.03-Fix02.3.md, T-01.03-Fix02.4.md, T-01.04-Fix02.5.md, T-01.05-Fix02.6.md


---

# T-01.02-Fix02.1

- **Source report:** [Executive index](../../README.md)

## EXE-Fix02.1 — T-01.02-Fix02.1

- **Status:** Implemented.
- **Kết quả:** DTO runtime UI không còn trả `connectionReference`; UI chỉ dùng ID/provider/display name/status.




---

# T-01.03-Fix02.2

- **Source report:** [Executive index](../../README.md)

## EXE-Fix02.2 — T-01.03-Fix02.2

- **Status:** Implemented.
- **Kết quả:** Form policy chọn source theo nhân viên hoặc chức vụ; policy role được truy vấn về đúng nhân viên có role đó.




---

# T-01.03-Fix02.3

- **Source report:** [Executive index](../../README.md)

## EXE-Fix02.3 — T-01.03-Fix02.3

- **Status:** Implemented.
- **Kết quả:** Thu hẹp dùng transaction và bị từ chối nếu policy kế nhiệm không là tập con của scope/quota/hạn/delegate/self-approve cũ.




---

# T-01.03-Fix02.4

- **Source report:** [Executive index](../../README.md)

## EXE-Fix02.4 — T-01.03-Fix02.4

- **Status:** Implemented.
- **Kết quả:** Evaluator có wildcard scope, ưu tiên employee policy trước role policy và regression cho scope/quota/hết hạn/revoke/self-approve.




---

# T-01.04-Fix02.5

- **Source report:** [Executive index](../../README.md)

## EXE-Fix02.5 — T-01.04-Fix02.5

- **Status:** Implemented.
- **Kết quả:** UI hiển thị source, status, giới hạn, reason và predecessor; không nhận runtime reference nội bộ từ Rust.




---

# T-01.05-Fix02.6

- **Source report:** [Executive index](../../README.md)

## EXE-Fix02.6 — T-01.05-Fix02.6

- **Status:** Implemented — technical evidence complete; CEO runtime/UI verification pending.
- **Kết quả:** Build/test pass và evidence/handoff runtime tách riêng; không tự ghi CEO acceptance.

================================================================================

================================================================================



