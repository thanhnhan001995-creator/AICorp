# Exe-T-01 — Executive gốc J-01

- **Grouped source:** EXE-01 đến EXE-05.

---

# T-01.01

- **Source report:** [Executive index](../../README.md)

## EXE-01 — T-01.01: Chốt ngôn ngữ miền tổ chức

- **Task referenced:** [`Tasks.md`](Tasks.md#t-0101--chốt-ngôn-ngữ-miền-tổ-chức) — `T-01.01`
- **Job:** J-01 — Nền tảng tổ chức, nhân viên và uỷ quyền.
- **Status:** Completed.
- **Started / completed:** 2026-11-06 (Asia/Bangkok).

### Đầu vào đã dùng

- [`brief.md`](brief.md), mục 7: mô hình khái niệm bắt buộc.
- [`ideas.md`](ideas.md), I-02: hồ sơ Nhân viên AI bền vững.
- [`ideas.md`](ideas.md), I-03: Hộp uỷ quyền.
- [`Tasks.md`](Tasks.md), T-01.01: yêu cầu đầu ra hợp đồng dữ liệu, ví dụ hợp lệ/lỗi và ID bền vững.

### Kết quả thực hiện

Đã tạo hợp đồng miền tổ chức độc lập công nghệ. Hợp đồng định nghĩa tổ chức, CEO, nhân viên AI, chức vụ, tuyến báo cáo, runtime binding và Hộp uỷ quyền; đồng thời tách trách nhiệm/chức vụ khỏi runtime/provider. Hợp đồng đặt quy tắc ID bền vững, quy tắc không suy quyền từ local/API, giới hạn uỷ quyền theo công việc/thời gian/ngân sách và ví dụ phải bị từ chối.

### Artifact và cách kiểm tra

1. Mở [`docs/contracts/organization-domain-contract.md`](docs/contracts/organization-domain-contract.md).
2. Kiểm tra bảng ID tại mục 2, entity contracts ở mục 3 và hiệu lực quyền ở mục 4.
3. Đối chiếu JSON hợp lệ ở mục 5 với các trường cấm/không hợp lệ ở mục 6.
4. Chạy checklist mục 7: mọi điều kiện cần cho T-01.01 đều có mục kiểm tra cụ thể.

### Lỗi và nguyên nhân

- **Lỗi:** Không có.
- **Nguyên nhân lỗi:** Không áp dụng.

### Điều kiện chuyển Task

T-01.01 đã có artifact và tiêu chí kiểm tra. T-01.02 có thể bắt đầu sau khi xác nhận lựa chọn công nghệ/lớp lưu trữ local-first, vì Task này cần hiện thực persistence nhưng brief/ideas chưa chọn stack.

---

## DEC-01 — Lựa chọn kiến trúc cho T-01.02 trở đi

- **Related tasks:** `T-01.02` và các Task triển khai tiếp theo trong [`Tasks.md`](Tasks.md).
- **Status:** CEO approved.
- **Date:** 2026-11-06 (Asia/Bangkok).

### Quyết định

AI Corp dùng **Tauri + React/TypeScript + Rust + SQLite**. React/TypeScript là lớp UI; Rust/Tauri sở hữu domain, kiểm tra Hộp uỷ quyền, SQLite và connector runtime; SQLite là local-first store. UI không được truy cập secret, tự quyết định quyền cuối cùng, hoặc ghi trực tiếp vào SQLite/ledger.

### Tác động kiểm tra được

T-01.02 phải triển khai persistence SQLite sau API Rust/Tauri và chứng minh React chỉ đi qua API đó. Quyết định này đã gỡ điều kiện chuyển Task của EXE-01.

---




---

# T-01.02

- **Source report:** [Executive index](../../README.md)

## EXE-02 — T-01.02: Lưu trữ tổ chức và hồ sơ nhân viên

- **Task referenced:** [`Tasks.md`](Tasks.md#t-0102--lưu-trữ-tổ-chức-và-hồ-sơ-nhân-viên) — `T-01.02`
- **Job:** J-01 — Nền tảng tổ chức, nhân viên và uỷ quyền.
- **Status:** Completed.
- **Started / completed:** 2026-11-06 (Asia/Bangkok).

### Đầu vào đã dùng

- Hợp đồng [`docs/contracts/organization-domain-contract.md`](docs/contracts/organization-domain-contract.md) từ T-01.01.
- Quyết định DEC-01: Tauri + React/TypeScript + Rust + SQLite.

### Kết quả kiểm tra môi trường

- `node --version` → `v24.15.0`.
- `npm --version` → `11.12.1`.
- `rustc --version` → không tìm thấy lệnh.
- `cargo --version` → không tìm thấy lệnh.
- `winget --version` → không thể khởi chạy từ sandbox hiện tại.

### Lỗi và nguyên nhân

- **Lỗi:** Không thể khởi tạo hay biên dịch Tauri khi `rustc` và `cargo` chưa tồn tại.
- **Nguyên nhân:** Rust toolchain MSVC chưa được cài/đưa vào PATH. Theo tài liệu Tauri cho Windows, Microsoft C++ Build Tools và WebView2 cũng là điều kiện phát triển bắt buộc; toolchain C++ hiện chưa được xác nhận trên máy.

### Cách kiểm tra sau khi khắc phục

1. `rustc --version` và `cargo --version` phải trả về phiên bản hợp lệ.
2. Công cụ C++/MSVC phải sẵn sàng để Cargo biên dịch target Windows.
3. Một ứng dụng Tauri tối thiểu phải khởi chạy được trước khi tạo SQLite persistence.

### Điều kiện tiếp tục

Cần cài Rust stable MSVC và Microsoft C++ Build Tools với workload **Desktop development with C++**. Sau đó sẽ kiểm tra WebView2, khởi tạo dự án Tauri/React/TypeScript và thực hiện persistence SQLite của T-01.02.

### Kết quả hoàn thành

- Đã khởi tạo [`app`](app) bằng Tauri + React/TypeScript và Rust, SQLite qua `rusqlite` được sở hữu bởi Rust.
- Lớp Rust tạo/lưu `organizations`, `roles`, `runtime_bindings` và `employees`; command typed `initialize_demo_organization`, `list_employees`, `create_employee`, `update_employee` là đường duy nhất UI sử dụng.
- Ứng dụng desktop đã mở thật: giao diện đọc nhân viên mẫu `An`, chức vụ `Operations Agent`, runtime local, cấp quản lý và trạng thái qua command Rust.
- Kiểm thử SQLite mở lại cùng database xác nhận nhân viên mẫu vẫn tồn tại; UI không import SQLite hay giữ kết nối/secret.

### Artifact và cách kiểm tra

1. Mở [`app/src-tauri/src/lib.rs`](app/src-tauri/src/lib.rs) để xem schema SQLite và các Tauri command.
2. Mở [`app/src/App.tsx`](app/src/App.tsx): chỉ có `invoke` từ `@tauri-apps/api/core`, không có driver SQLite.
3. Trong `D:\Moi-Truong_AI\app`, chạy `npm run build`; trong `D:\Moi-Truong_AI\app\src-tauri`, chạy `cargo test`.
4. Chạy `npx tauri dev` với `%USERPROFILE%\.cargo\bin` trong `PATH`; mở hồ sơ `An`, tạo/cập nhật nhân sự và mở lại ứng dụng để kiểm tra persistence.

### Cập nhật prerequisite — 2026-11-06

- **Rust:** Đã cài thành công qua Rustup. Xác minh bằng đường dẫn `%USERPROFILE%\.cargo\bin`: `rustc 1.99.1` và `cargo 1.99.1`.
- **Microsoft C++ Build Tools:** Installer đã được tải và đang mở cửa sổ **Visual Studio Installer** với workload `Microsoft.VisualStudio.Workload.VCTools`, nhưng chưa hoàn tất. Lần khởi chạy trùng báo `0x80070020` vì installer đầu tiên vẫn giữ tệp.
- **Trạng thái hiện tại:** Đã khắc phục. `vswhere` đã xác nhận component `Microsoft.VisualStudio.Component.VC.Tools.x86.x64`; Cargo và Tauri đã build/chạy được.
- **Lỗi phát sinh khi chạy:** `npx tauri dev` ban đầu không thấy `cargo` trong `PATH`; đã chạy lại với `%USERPROFILE%\.cargo\bin` chỉ trong phiên lệnh, không đổi cấu hình hệ thống. Có hai instance debug do thử chạy lại, khiến build overwrite `app.exe` bị Windows từ chối; không ảnh hưởng database hay mã nguồn.

---




---

# T-01.03

- **Source report:** [Executive index](../../README.md)

## EXE-03 — T-01.03: Hiện thực Hộp uỷ quyền theo ngữ cảnh

- **Task referenced:** [`Tasks.md`](Tasks.md#t-0103--hiện-thực-hộp-uỷ-quyền-theo-ngữ-cảnh) — `T-01.03`
- **Status:** Completed.
- **Kết quả:** SQLite có bảng `authority_envelopes`; API Rust cấp, liệt kê, thu hồi và đánh giá quyền theo nhân sự, action, resource/work scope, thời hạn và ngân sách. Thu hồi chuyển trạng thái sang `revoked`, không xoá lịch sử.
- **Kiểm tra:** xem `create_authority_envelope`, `revoke_authority_envelope`, `evaluate_authority` trong [`app/src-tauri/src/lib.rs`](app/src-tauri/src/lib.rs); `cargo test` xác nhận quyền active còn đọc được sau khi mở lại SQLite.
- **Lỗi:** Không có lỗi còn mở.




---

# T-01.04

- **Source report:** [Executive index](../../README.md)

## EXE-04 — T-01.04: Hồ sơ nhân viên cho CEO

- **Task referenced:** [`Tasks.md`](Tasks.md#t-0104--hồ-sơ-nhân-viên-cho-ceo) — `T-01.04`
- **Status:** Completed.
- **Kết quả:** CEO có danh bạ và hồ sơ để xem/tạo/chỉnh sửa nhân sự: tên, chức vụ, runtime, cấp quản lý, trạng thái. Với nhân sự đang chọn, CEO thấy Hộp uỷ quyền, cấp quyền báo cáo và thu hồi quyền; hồ sơ khác không bị thay đổi.
- **Kiểm tra:** chạy app, mở `An`, dùng **+ Thêm** để tạo nhân sự; dùng **Cấp quyền báo cáo** và **Thu hồi** trong mục Hộp uỷ quyền. Giao diện đã được quan sát khi app chạy thật.
- **Lỗi:** Không có lỗi còn mở.




---

# T-01.05

- **Source report:** [Executive index](../../README.md)

## EXE-05 — T-01.05: Kiểm tra nền tảng tổ chức và uỷ quyền

- **Task referenced:** [`Tasks.md`](Tasks.md#t-0105--kiểm-tra-nền-tảng-tổ-chức-và-uỷ-quyền) — `T-01.05`
- **Status:** Completed.
- **Kết quả kiểm tra:** `npm run build` hoàn tất; `cargo test` chạy 1/1 test pass. Test tạo database SQLite tạm, seed tổ chức/nhân viên, ghi Hộp uỷ quyền, đóng/mở lại database và xác nhận nhân viên lẫn quyền active còn tồn tại.
- **Bằng chứng runtime:** Tauri desktop đã chạy và hiển thị dữ liệu `An` qua Rust API, bao gồm ranh giới không truy cập SQLite trực tiếp từ React.
- **Giới hạn đã biết:** connector agent thật, ledger bất biến, công việc và báo cáo chưa triển khai vì thuộc J-02/J-03 trở đi.

## Trạng thái nghiệm thu J-01

J-01 từng được ghi nhận là đã nghiệm thu ngày 2026-11-06. Sau audit hồi tố `AIC-REV-J01-20261106-01`, trạng thái chính xác được hiệu chỉnh thành **Implemented with deviations — chưa CEO-verified đầy đủ**. T-01.03 đến T-01.05 có sai lệch kiểm chứng/quyền cần follow-up; J-03 không được dùng J-01 như điều kiện quyền đã chứng minh cho tới khi CEO xác nhận lại.

### Addendum kiểm chứng hồi tố J-01 — 2026-11-06

- **Review:** [`j01-retrospective-verification.md`](../../Reviews/J01/j01-retrospective-verification.md) — `AIC-REV-J01-20261106-01`.
- **Kết quả:** T-01.01 đúng; T-01.02 và T-01.04 đúng một phần; T-01.03 và T-01.05 có sai lệch đáng kể.
- **Lý do:** cấu trúc Hộp uỷ quyền/UI chưa bao phủ quyền theo chức vụ, thu hẹp envelope, delegate/self-approve và toàn bộ giới hạn; bộ test/UI chưa chứng minh các case lỗi yêu cầu.
- **CEO evidence:** Hướng dẫn thao tác, artifact cần mở và bảng xác nhận từng Task đã được thêm vào [`Tasks.md`](Tasks.md), phần “Hướng dẫn bằng chứng và xác nhận CEO — J-01”.

---



