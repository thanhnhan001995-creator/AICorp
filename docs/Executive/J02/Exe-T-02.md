# Exe-T-02 — Executive J-02

- **Grouped source:** T-02.01.md, T-02.02.md, T-02.03.md, T-02.04.md, T-02.05.md.

---

# T-02.01

- **Source report:** [Executive index](../../README.md)

## EXE-06 — T-02.01: Xác định sự kiện ledger và liên kết nguồn

- **Task referenced:** [`Tasks.md`](Tasks.md#t-0201--xác-định-sự-kiện-ledger-và-liên-kết-nguồn) — `T-02.01`
- **Job:** J-02 — Sổ cái bất biến và báo cáo điều hành.
- **Status:** Completed.
- **Started / completed:** 2026-11-06 (Asia/Bangkok).

### Đầu vào đã dùng

- [`brief.md`](brief.md), mục 5.3 và vòng điều hành mục 11.
- [`ideas.md`](ideas.md), I-05 (báo cáo/artifact) và I-06 (Operational Ledger).
- Kết quả J-01: định danh tổ chức, nhân viên, runtime binding và Hộp uỷ quyền.

### Kết quả thực hiện

Đã tạo hợp đồng ledger xác định event là record append-only có ID, `occurredAt`, `recordedAt`, actor, source, subject references, context, evidence references và schema version. Hợp đồng phân biệt rõ quyết định CEO, hành vi agent theo quyền, runtime signal, report/artifact, delegation và correction. Nó cũng định nghĩa report sáu phần cùng metadata tối thiểu để artifact có thể mở/kiểm tra.

Đính chính là event `ledger.correction_recorded` mới, bắt buộc liên kết event gốc và lý do; không có cơ chế sửa/xoá event cũ, kể cả CEO. T-02.02 được chỉ dẫn triển khai Rust là điểm append duy nhất, UI không có đường update/delete ledger.

### Artifact và cách kiểm tra

1. Mở [`docs/contracts/operational-ledger-contract.md`](docs/contracts/operational-ledger-contract.md) — `AIC-CON-LED-20261106-01`.
2. Kiểm tra JSON event tại mục 3, nhóm event tại mục 4, liên kết report/artifact tại mục 5 và correction tại mục 6.
3. Kiểm tra bảng trường hợp bị từ chối ở mục 7: có cấm `UPDATE`/`DELETE`, tự duyệt không hợp lệ và evidence không kiểm tra được.
4. Đã chạy kiểm tra nội dung bắt buộc (`LEDGER_CONTRACT_CHECK_OK`) cho document ID, event đính chính, thời điểm ghi, evidence references, authority reference và điều kiện bàn giao T-02.02.

### Lỗi và nguyên nhân

- **Lỗi:** Không có.
- **Nguyên nhân lỗi:** Không áp dụng.

### Điều kiện chuyển Task

T-02.01 hoàn thành. T-02.02 có thể bắt đầu với hợp đồng `AIC-CON-LED-20261106-01` làm đầu vào duy nhất cho schema và API append-only.

---




---

# T-02.02

- **Source report:** [Executive index](../../README.md)

## EXE-07 — T-02.02: Lưu ledger chỉ-ghi-thêm

- **Status:** Completed.
- **Kết quả:** Thêm bảng `ledger_events` trong SQLite, Rust command append/list và trigger `ledger_events_no_update`/`ledger_events_no_delete`. Rust là API append duy nhất; React không có driver SQLite hay đường update/delete.
- **Kiểm tra:** `cargo test` tạo event, thử `UPDATE` và `DELETE`; cả hai bị trigger từ chối. Database được đóng/mở lại trong test và event vẫn tồn tại.
- **Artifact:** [`app/src-tauri/src/lib.rs`](app/src-tauri/src/lib.rs).




---

# T-02.03

- **Source report:** [Executive index](../../README.md)

## EXE-08 — T-02.03: Luồng đính chính ledger

- **Status:** Completed.
- **Kết quả:** `ledger.correction_recorded` bắt buộc `correctionOfEventId`; API từ chối correction không có nguồn. Correction là event mới, không sửa event gốc.
- **Kiểm tra:** Rust test append event gốc, append correction liên kết ID gốc, rồi xác nhận event gốc không thể update/delete.
- **Artifact:** [`app/src-tauri/src/lib.rs`](app/src-tauri/src/lib.rs), hợp đồng [`docs/contracts/operational-ledger-contract.md`](docs/contracts/operational-ledger-contract.md).




---

# T-02.04

- **Source report:** [Executive index](../../README.md)

## EXE-09 — T-02.04: Cấu trúc báo cáo và artifact

- **Status:** Completed.
- **Kết quả:** Thêm SQLite `reports`/`artifacts`, command tạo/liệt kê report và tự append event `report.submitted`. Report có sáu phần CEO đã duyệt; artifact location được đăng ký cùng report. UI có mục Báo cáo điều hành, mở được đầy đủ sáu phần và tạo report kiểm chứng.
- **Kiểm tra:** [`app/src/App.tsx`](app/src/App.tsx) dùng command Rust `create_report`/`list_reports`; `npm run build` pass.
- **Giới hạn đúng phạm vi:** report mẫu chưa là bàn giao từ agent thật; điều đó thuộc J-03.




---

# T-02.05

- **Source report:** [Executive index](../../README.md)

## EXE-10 — T-02.05: Kiểm tra bất biến và báo cáo

- **Status:** Completed.
- **Kết quả kiểm tra:** `npm run build` pass; `cargo test` pass 1/1. Test chứng minh persistence SQLite, append event, cấm update/delete, và correction có liên kết event gốc. Giao diện report build thành công và chỉ gọi Rust commands.
- **Cách CEO tái kiểm tra:** chạy `npm run build`; chạy `cargo test` trong `app/src-tauri`; chạy app và chọn **Tạo báo cáo mẫu**, sau đó thấy sáu phần report. Xem trigger và test tại [`app/src-tauri/src/lib.rs`](app/src-tauri/src/lib.rs).

## Trạng thái nghiệm thu J-02

J-02 hoàn thành triển khai T-02.01 đến T-02.05 và đang **chờ CEO nghiệm thu**. J-03 chưa bắt đầu.

================================================================================

# Fix01 — Kết quả khắc phục J-01

- **Fix scope:** [`Tasks.md`](Tasks.md), Fix01 — Task `T-01.02-Fix01.1` đến `T-01.05-Fix01.5`.
- **Nguồn:** review hồi tố `AIC-REV-J01-20261106-01`.
- **Quy tắc ghi nhận:** Mỗi Task Fix01 chỉ ghi trạng thái triển khai, kết quả, lỗi và artifact kỹ thuật. Việc CEO nghiệm thu được ghi tách biệt trong `AcceptanceReport.md`.

--------------------------------------------------------------------------------



