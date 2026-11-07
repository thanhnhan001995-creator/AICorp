# Quy trình phát triển và nghiệm thu AI Corp

- **Mã tài liệu:** WP-01
- **Ngày tạo:** 07/11/2026
- **Cập nhật gần nhất:** 07/11/2026 — bổ sung quy trình AcceptanceReport final và chốt trạng thái Job.
- **Phạm vi:** Quy trình chuyển yêu cầu của CEO thành kết quả có thể kiểm tra, sửa sai lệch và nghiệm thu theo từng Job.

## 1. Nguyên tắc vận hành

1. Làm theo thứ tự, không nhảy thẳng từ yêu cầu sang triển khai khi chưa có tài liệu định hướng phù hợp.
2. Mỗi kết quả phải truy vết được ngược về Brief, Idea, Job và Task đã phê duyệt.
3. Thành công khi build hoặc test tự động **không** tự thay thế kiểm tra runtime/UI thực tế của CEO.
4. Khi phát hiện thiếu sót, không sửa đè lịch sử Task, Execute, Evidence, Review hay AcceptanceReport đã lập. Tạo một đợt Fix mới kế thừa đúng điểm chưa chuẩn.
5. CEO là người quyết định phạm vi, phê duyệt Task trước khi làm và nghiệm thu kết quả tương tác thực tế.

## 2. Chuỗi tài liệu và vai trò

```text
Brief → Ideas → Jobs → Tasks → Thực hiện → Execute + Evidence
                                      ↓
                         Review hồi tố với Brief/Ideas
                                      ↓
                Chuẩn kỹ thuật? ── Không ──→ Fix → Thực hiện lại
                       │                                  │
                       Có ─────────────────────────────────┘
                       ↓
            CEO kiểm tra runtime/UI → AcceptanceReport lượt n
                       ↓
            Rejected: tạo Fix mới | Accepted: lập Final
                       ↓
        AcceptanceReport Final → Job hoàn tất chính thức
```

| Tài liệu | Mục đích | Người tạo/cập nhật | Không dùng để |
| --- | --- | --- | --- |
| `brief.md` | Xác định sản phẩm, người dùng, giá trị, ranh giới và tiêu chí định hướng. | CEO duyệt; nhóm phát triển ghi nhận. | Chia nhỏ thao tác triển khai. |
| `ideas.md` | Chuyển Brief thành các ý tưởng sản phẩm có mã Idea và tham chiếu nguồn khi cần. | Nhóm phát triển; CEO duyệt. | Cam kết kỹ thuật cuối cùng. |
| `Jobs.md` | Gom các Idea thành đầu việc lớn, có đầu vào, đầu ra kiểm tra được và quan hệ kế thừa. | Nhóm phát triển; CEO duyệt. | Liệt kê thao tác nhỏ để lập trình. |
| `docs/Tasks/<Job>/` | Chia Job thành Task thực hiện được; mỗi Task nêu đầu vào, việc chính, đầu ra, phụ thuộc và cách CEO kiểm tra. | Nhóm phát triển; CEO phê duyệt trước thực hiện. | Ghi kết quả đã làm. |
| `docs/Executive/<Job>/` | Nhật ký thuần túy của lần triển khai: kết quả, lỗi, nguyên nhân, file/lệnh/cách kiểm tra kỹ thuật. | Nhóm phát triển. | Xin hoặc ghi nhận phê duyệt CEO. |
| `docs/evidence/<Job>/` | Bằng chứng kỹ thuật có thể tái kiểm: test, build, migration, API, ảnh/log được dẫn chiếu. | Nhóm phát triển. | Thay thế kiểm tra UI/runtime của CEO. |
| `docs/reviews/<Job>/` | Kiểm tra hồi tố: so khớp trạng thái mới nhất với Brief, Ideas, Job và Fix; chỉ ra thiếu sót còn lại. | Nhóm phát triển. | Tuyên bố CEO đã nghiệm thu. |
| `docs/AcceptanceReport/<Job>-AccRep/` | Biên bản CEO kiểm tra thực tế theo từng lượt và biên bản `Final` để chốt Job. | CEO ghi kết quả; nhóm phát triển chuẩn bị hướng dẫn và tổng hợp final. | Nhật ký triển khai kỹ thuật. |

## 3. Quy trình từ định hướng đến Task

1. **Lập và duyệt Brief.** Chỉ khi CEO đồng ý tiêu chí ở Brief mới chuyển sang Ideas.
2. **Lập và duyệt Ideas.** Mỗi ý tưởng phải phản ánh Brief; nguồn tham khảo chỉ là tham khảo, không tự mở rộng phạm vi.
3. **Lập và duyệt Jobs.** Mỗi Job nêu rõ Ideas được hiện thực hóa, đầu vào/kế thừa, một việc chính lớn và đầu ra CEO có thể tự kiểm tra.
4. **Lập và duyệt Tasks của Job.** Một Job có từ 1 đến 12 Task. Nếu phức tạp hơn, được tối đa 3 Task lớn, mỗi Task lớn tối đa 9 Task nhỏ. Task phải đủ nhỏ để triển khai và tìm đúng vị trí sai, nhưng không biến thành nhật ký thao tác.
5. **Lập ma trận acceptance trước triển khai.** Với mỗi đầu ra CEO sẽ kiểm tra, ghi thao tác, dữ liệu đầu vào, kết quả mong đợi, bằng chứng cần giữ và điều kiện fail. Ma trận có thể nằm trong Task bundle hoặc AcceptanceReport nháp, nhưng phải được duyệt trước khi code phần có rủi ro.
6. **Chốt contract và state transition trước UI.** Với dữ liệu có trạng thái hoặc quyền hạn, Task phải nêu source of truth, subject, trạng thái hợp lệ, transition hợp lệ/không hợp lệ, thứ tự ưu tiên và điều kiện transaction/rollback. UI chỉ gọi contract đã chốt, không tự suy diễn luật nghiệp vụ.
7. **Chạy smoke test bằng đúng môi trường CEO trước AcceptanceReport.** Trước khi gửi CEO nghiệm thu, nhóm phát triển phải dùng đúng shell/command/path CEO sẽ dùng (ví dụ Command Prompt), xác minh app mở được và ghi command đã pass vào Execute/Evidence. Smoke test không thay thế kiểm tra runtime/UI của CEO.
8. **Chờ CEO cho phép thực hiện.** Không tự động triển khai Task hay Job kế tiếp khi chưa có phạm vi/phê duyệt tương ứng của CEO.

## 4. Quy trình thực hiện một Job

1. Thực hiện các Task của Job theo thứ tự phụ thuộc đã định.
2. Sau từng Task, ghi kết quả vào Execute tương ứng: phần đã làm, kết quả kiểm tra kỹ thuật, lỗi (nếu có), nguyên nhân, file/thư mục/lệnh để tái kiểm.
3. Lưu hoặc cập nhật Evidence tương ứng khi có bằng chứng kỹ thuật đáng kể. Evidence phải nói rõ nó chứng minh điều gì và không chứng minh điều gì.
4. Hoàn thành nhóm Task ban đầu thì dừng tại mốc CEO kiểm tra, nếu kế hoạch yêu cầu mốc này.
5. Thực hiện Review hồi tố so với Brief, Ideas, Job và toàn bộ Fix hiện có.
6. Không tạo Execute thay cho CEO approval: Execute chỉ được cập nhật sau hoạt động triển khai; AcceptanceReport mới là nơi ghi Pass/Fail/Accepted/Rejected của CEO.

## 5. Xử lý sai lệch bằng Fix

Khi Review hoặc CEO kiểm tra phát hiện kết quả chưa đúng:

1. Giữ nguyên Task, Execute, Evidence, Review và AcceptanceReport cũ để bảo toàn lịch sử.
2. Tạo file Task Fix mới trong cùng thư mục Job. Quy ước hiện hành: `T-01-Fix01.md`, `T-01-Fix02.md`…; một Fix có thể chứa các Task con.
3. Fix phải ghi rõ: nguồn phát hiện (Review/AcceptanceReport), Task gốc liên quan, yêu cầu Brief/Idea/Job bị lệch, đầu vào kế thừa và đầu ra kiểm tra được.
4. CEO duyệt Fix trước khi thực hiện.
5. Ghi triển khai vào Execute cùng tên, ví dụ `Exe-T-01-Fix01.md`, và Evidence tương ứng `envidence-T-01-Fix01.md` nếu cần. Giữa các cụm thực hiện khác nhau trong Execute dùng dải `====` để dễ phân biệt.
6. Sau Fix, lập Review hồi tố mới trên trạng thái mới nhất. Lặp lại cho đến khi không còn sai lệch kỹ thuật trong phạm vi Job.

## 6. Kiểm tra CEO và nghiệm thu

Khi Review kết luận đã sẵn sàng kỹ thuật:

1. Tạo AcceptanceReport trong `docs/AcceptanceReport/<Job>-AccRep/` theo thứ tự lần nghiệm thu, ví dụ `AcceptanceReport-J01-1st.md`.
2. Báo cáo phải nhắc lại Brief/Ideas/Job/Tasks liên quan, tóm tắt kết quả, và cung cấp hướng dẫn mở/chạy/thao tác bằng môi trường CEO đang dùng (hiện tại là Command Prompt khi phù hợp).
3. CEO thực hiện kiểm tra runtime/UI thực tế và tự ghi kết quả từng bước, bằng chứng và kết luận.
4. Nếu **Rejected**, giữ nguyên AcceptanceReport đó, tạo Fix mới dựa trên issue đã ghi, rồi trở lại bước thực hiện và Review hồi tố. Không đổi kết quả cũ thành pass.
5. Nếu lượt mới nhất là **Accepted**, lập `AcceptanceReport-<Job>-Final.md`. Final phải: dẫn về AcceptanceReport được Accepted; tổng hợp tất cả Task/Fix thuộc phạm vi đã được CEO nghiệm thu; nêu rõ các lượt Rejected vẫn được giữ lịch sử; dẫn tới Review/Execute/Evidence chốt; và ghi phạm vi Job hoàn thành.
6. Chỉ khi biên bản **Final** có kết luận `Accepted — Job completed` mới đánh dấu Job hoàn thành chính thức trong `Jobs.md` và chỉ mục `Tasks/README.md`. Job này từ đó trở thành đầu vào hợp lệ cho Job phụ thuộc.

## 7. Quy ước tổ chức thư mục hiện hành

```text
docs/
├── brief.md
├── ideas.md
├── Jobs.md
├── WorkflowProcess.md
├── Tasks/<Job>/
├── Executive/<Job>/
├── evidence/<Job>/
├── reviews/<Job>/
├── AcceptanceReport/<Job>-AccRep/
└── contracts/
```

Tên file Task và Execute tương ứng phải cùng định danh; Execute thêm tiền tố `Exe-`. Tài liệu mới phải trỏ tới tài liệu nguồn cần thiết và dùng đường dẫn tương đối còn hiệu lực sau khi tổ chức lại.

Với AcceptanceReport, dùng `AcceptanceReport-<Job>-1st.md`, `-2nd.md`… cho các lượt kiểm tra; dùng chính xác `AcceptanceReport-<Job>-Final.md` cho biên bản chốt cuối. Không xóa hay thay tên các lượt lịch sử chỉ để làm gọn thư mục.

## 8. Điểm dừng bắt buộc

- Dừng chờ CEO sau khi lập Brief, Ideas, Jobs, Task/Fix mới cần phê duyệt, hoặc khi cần quyền/ý kiến quyết định phạm vi.
- Dừng để CEO kiểm tra khi kết quả cần runtime/UI thật, dữ liệu ngoài phạm vi kỹ thuật, hoặc quyết định mang tính điều hành.
- Không gọi một Job là hoàn tất chỉ vì build/test pass hoặc vì một lượt AcceptanceReport pass; Job hoàn tất khi có **AcceptanceReport Final** kết luận `Accepted — Job completed`.

## 9. Sau khi Job đã hoàn thành

1. Không sửa đè Task, Execute, Evidence, Review hay AcceptanceReport Final của Job đã chốt.
2. Một lỗi phát hiện sau final được mở thành Fix mới, có nguồn là report/runtime evidence mới và phải trải qua lại Review → AcceptanceReport lượt mới → Final cập nhật hoặc final bổ sung.
3. Một yêu cầu mở rộng không phải sửa lỗi được lập thành Idea/Job mới; chỉ dùng Job hoàn thành làm đầu vào đã nghiệm thu, không tự mở rộng phạm vi của Job cũ.
4. Khi Job là phụ thuộc của Job khác, chỉ được truyền các đầu ra đã nêu trong Final; không coi dữ liệu mẫu, log agent hoặc build artifact đơn lẻ là đầu vào đã nghiệm thu.
