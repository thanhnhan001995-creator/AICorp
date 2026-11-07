# Tasks — AI Corp

- **Index:** Split by Job and Fix rounds.
- **Legacy compatibility:** [Tasks.md](../Tasks.md) points here.

# Tasks — AI Corp

- **Document ID:** `AIC-TSK-20261106-01`
- **Created:** 2026-11-06 (Asia/Bangkok)
- **Last updated:** 2026-11-06 (Asia/Bangkok)
- **Brief referenced:** [`brief.md`](brief.md) — `AIC-BRF-20261105-01`
- **Ideas referenced:** [`ideas.md`](ideas.md) — `AIC-IDE-20261106-01`
- **Jobs referenced:** [`Jobs.md`](Jobs.md) — `AIC-JOB-20261106-01`
- **Trạng thái:** J-01 đã được CEO nghiệm thu hoàn tất chính thức ngày 2026-11-07 qua `AcceptanceReport-J01-Final`; J-02 đã hoàn thành triển khai và chờ CEO nghiệm thu; các Job còn lại chưa bắt đầu.
- **Architecture baseline (CEO bổ sung 2026-11-06):** Tauri + React/TypeScript + Rust + SQLite. Mọi Task triển khai UI phải gọi API Rust được giới hạn; UI không giữ secret, không tự kiểm tra quyền cuối cùng và không ghi SQLite trực tiếp.

## 1. Quy ước Task

- Mỗi Task là một phần việc xác định của đúng một Job.
- Một Task chỉ hoàn thành khi đầu ra nêu trong Task đã được kiểm tra; thay đổi mã, build thành công hoặc log agent tự nhận không tự là bằng chứng đủ.
- `Phụ thuộc` chỉ Task/Job phải hoàn thành trước khi Task bắt đầu. Mọi đầu vào được kế thừa phải chỉ rõ nguồn.
- Các Task cùng Job có thể thực hiện song song khi không phụ thuộc nhau, nhưng không được bỏ qua điều kiện nghiệm thu của Job cha.
- Không Job nào trong kế hoạch này vượt quá 12 Task, nên không cần dùng cấu trúc Task lớn/Task nhỏ.

## 2. Chỉ mục Task theo Job

| Job | Số Task | Task |
| --- | ---: | --- |
| J-01 — Tổ chức, nhân viên, uỷ quyền | 5 + 5 Fix01 + 6 Fix02 + 5 Fix03 + 7 Fix04 + 5 Fix05 + 2 Fix06 + 2 Fix07 + 3 Fix08 | T-01.01 → T-01.05; Fix01.1 → Fix01.5; Fix02.1 → Fix02.6; Fix03.1 → Fix03.5; Fix04.1 → Fix04.7; Fix05.1 → Fix05.5; Fix06.1 → Fix06.2; Fix07.1 → Fix07.2; Fix08.1 → Fix08.3 |
| J-02 — Ledger và báo cáo | 5 | T-02.01 → T-02.05 |
| J-03 — Công việc và agent thật | 6 | T-03.01 → T-03.06 |
| J-04 — Uỷ quyền phụ | 5 | T-04.01 → T-04.05 |
| J-05 — Command Desk và Attention | 5 | T-05.01 → T-05.05 |
| J-06 — Văn phòng ảo | 4 | T-06.01 → T-06.04 |
| J-08 — Kinh nghiệm nhân viên | 5 | T-08.01 → T-08.05 |
| J-07 — Chứng minh vòng vận hành | 6 | T-07.01 → T-07.06 |

## 3. Tasks của J-01 — Nền tảng tổ chức, nhân viên và uỷ quyền


## J01 bundles
- [T-01](J01/T-01.md)
- [T-01-Fix01](J01/T-01-Fix01.md)
- [T-01-Fix02](J01/T-01-Fix02.md)
- [T-01-Fix03](J01/T-01-Fix03.md)
- [T-01-Fix04](J01/T-01-Fix04.md)
- [T-01-Fix05](J01/T-01-Fix05.md)
- [T-01-Fix06](J01/T-01-Fix06.md)
- [T-01-Fix07](J01/T-01-Fix07.md)
- [T-01-Fix08](J01/T-01-Fix08.md)


## Job bundles J02–J08
- [T-02](J02/T-02.md)
- [T-03](J03/T-03.md)
- [T-04](J04/T-04.md)
- [T-05](J05/T-05.md)
- [T-06](J06/T-06.md)
- [T-07](J07/T-07.md)
- [T-08](J08/T-08.md)

