# Product Ideas — AI Corp

- **Document ID:** `AIC-IDE-20261106-01`
- **Created:** 2026-11-06 (Asia/Bangkok)
- **Last updated:** 2026-11-06 (Asia/Bangkok)
- **Brief referenced:** [`brief.md`](brief.md) — `AIC-BRF-20261105-01`

## 1. Tình trạng tài liệu

- **Nguồn định hướng:** `brief.md` đã được CEO duyệt.
- **Trạng thái:** các idea I-01 đến I-10 và ba quyết định tại mục 7 đã được CEO duyệt; các bổ sung tiếp theo được ghi nhận trong bản này.
- **Mục đích:** chuyển các yêu cầu và ranh giới của brief thành những ý tưởng sản phẩm có thể đánh giá.
- **Không phải:** danh sách job, task, roadmap, thiết kế kỹ thuật chi tiết hay cam kết triển khai.
- **Quy tắc:** một idea chỉ đáng giữ khi nó làm rõ vòng điều hành của CEO, tăng bằng chứng, hoặc làm quyền hạn dễ kiểm soát hơn.

Các idea dưới đây là đề xuất độc lập cho AI Corp. Dự án tham khảo chỉ cung cấp bài học hoặc mô thức; chúng không phải blueprint để sao chép.

## 2. Luận điểm sản phẩm

**AI Corp nên là một “bàn điều hành có nhân viên” trước, và một “văn phòng ảo” sau.**

CEO cần biết ngay ba điều:

1. Đội ngũ đang làm gì?
2. Điều gì cần CEO quyết định ngay?
3. Điều gì đã được làm, dựa trên quyền nào và có bằng chứng gì?

Vì vậy, mọi lớp trực quan hoặc tự động hoá đều phải quay về chuỗi sau:

> Mục tiêu → công việc → người chịu trách nhiệm → quyền đang có → phiên thực hiện → báo cáo/artifact → quyết định của CEO.

Đây là khác biệt quan trọng so với một dashboard chỉ đếm token, một bảng Kanban thông thường, hoặc một game mô phỏng công sở.

## 3. Các mô thức tham khảo đã đối chiếu

| Nguồn | Mô thức đáng học | Không nên lấy nguyên xi |
| --- | --- | --- |
| [Paperclip](https://github.com/xRaudra/paperclip) | Liên kết mục tiêu, cấu trúc tổ chức, ngân sách, governance và audit trail trong cùng một không gian điều hành. | Giả định vận hành tự trị liên tục hoặc quản lý nhiều công ty từ đầu. |
| [Agent HQ](https://github.com/tools-for-agents/agent-hq) | Agent registry, Kanban có trạng thái blocked/dependency, memory bền vững và activity log có thể xem lại. | Chỉ dùng Kanban làm trải nghiệm trung tâm; AI Corp cần đặt báo cáo và quyết định CEO ngang hàng với công việc. |
| [TeamMCP](https://github.com/cookjohn/teammcp) | Tác nhân là thực thể bền vững, có giao tiếp, inbox, task và audit trail; tương thích nhiều agent qua MCP. | Agent được tự do phối hợp không cần một mô hình uỷ quyền rõ ràng theo CEO. |
| [Agency](https://github.com/christag/agency) | Agent có danh tính, vai trò, lịch sử; pipeline quan sát → đề xuất → quyết định → thực thi để biến “nhiễu agent” thành việc CEO có thể xử lý. | Mô hình quản lý thiên về agent coding; AI Corp phải phục vụ công việc hỗn hợp. |

Các nguồn đều xác nhận giá trị của agent identity, công việc có trạng thái, lịch sử và sự giám sát của con người. Brief của AI Corp bổ sung điểm nhấn riêng: **uỷ quyền có giới hạn theo chức vụ, báo cáo là artifact đầu tiên, và văn phòng ảo là bản đồ trực quan phụ trợ cho bàn điều hành.**

## 4. Các idea nền tảng

### I-01 — CEO Command Desk

**Ý tưởng:** Một màn điều hành duy nhất trả lời ba câu hỏi cốt lõi của CEO, thay vì buộc CEO đọc log hoặc chuyển tab.

- **Hiển thị trọng tâm:** công việc đang chạy, việc bị chặn, báo cáo đang chờ xem, yêu cầu cấp quyền/duyệt, và nhân viên cần chú ý.
- **Quyết định CEO có thể bắt đầu từ đây:** giao việc, đổi ưu tiên, tạm dừng, yêu cầu báo cáo, duyệt, yêu cầu sửa, hoặc xem lý do bị chặn.
- **Ràng buộc:** không biến thành trang KPI trang trí; mọi tín hiệu phải mở được dữ liệu và lịch sử bên dưới.
- **Vì sao phù hợp brief:** trực tiếp phục vụ quyền điều hành, khả năng can thiệp và tiêu chí “biết cần xử lý gì tiếp theo”.

### I-02 — Hồ sơ Nhân viên AI bền vững

**Ý tưởng:** Mỗi agent là một nhân viên có danh tính dài hạn, không phải một lần gọi model vô danh.

- **Bao gồm ở cấp ý tưởng:** tên, chức vụ, năng lực được khai báo, nguồn chạy hiện tại, cấp quản lý/báo cáo, tình trạng, lịch sử công việc và phạm vi quyền.
- **Điểm quan trọng:** vai trò tách khỏi runtime. Một “Nhà nghiên cứu” có thể chạy bằng local model hôm nay và provider khác ngày mai mà không mất trách nhiệm/lịch sử.
- **Không suy diễn năng lực:** hồ sơ nêu rõ năng lực được CEO cấp hoặc quan sát được, không gán cho agent tính cách hay trách nhiệm như con người thật.
- **Vì sao phù hợp brief:** tạo trách nhiệm rõ ràng và cho phép công ty đa chức năng mà không bị khóa vào một provider.

### I-03 — Hộp uỷ quyền (Delegation Envelope)

**Ý tưởng:** Mỗi quyền hoạt động của nhân viên được diễn đạt như một “hộp uỷ quyền” có thể đọc được, thay vì một danh sách quyền kỹ thuật mơ hồ.

Hộp uỷ quyền trả lời: nhân viên được làm **việc gì**, trên **phạm vi nào**, trong **bao lâu**, với **ngân sách/quota nào**, được **giao việc phụ** hay không, có được **tự duyệt** hay không, và khi nào phải báo CEO.

- **Theo ngữ cảnh:** quyền chức vụ là nền; công việc cụ thể có thể siết hoặc mở trong giới hạn CEO chỉ định.
- **Không gắn với local/API:** nguồn model chỉ là cách thực thi; uỷ quyền là quyết định quản trị.
- **CEO override:** CEO có thể xem, thu hồi, thu hẹp hoặc buộc duyệt trước ngay cả khi agent đang được uỷ quyền rộng.
- **Vì sao phù hợp brief:** đây là cách hiện thực hoá chính sách ở mục 13 mà không đánh đồng “agent tự trị” với “agent không kiểm soát”.

### I-04 — Công việc như một hợp đồng có thể kiểm tra

**Ý tưởng:** Một công việc không chỉ là prompt. Nó là một hợp đồng vận hành giữa CEO và người chịu trách nhiệm.

Mỗi công việc cần có ở cấp ý tưởng: mục tiêu, bối cảnh/đầu vào, người chịu trách nhiệm, giới hạn uỷ quyền đang áp dụng, điều kiện hoàn thành, yêu cầu báo cáo và liên kết tới các công việc phụ hoặc phụ thuộc.

- **Trạng thái phải trả lời được lý do:** chờ ai, bị chặn bởi gì, hoặc đang chờ CEO xem nội dung nào.
- **Tách “đã chạy xong” với “đã được chấp nhận”:** agent có thể dừng xử lý nhưng báo cáo vẫn chờ CEO duyệt hoặc cần sửa.
- **Vì sao phù hợp brief:** ngăn trạng thái “done” tự nhận và cho phép truy nguyên kết quả về mục tiêu ban đầu.

### I-05 — Báo cáo điều hành là artifact mặc định

**Ý tưởng:** Mọi lần bàn giao đầu tiên đều xoay quanh một báo cáo có cấu trúc, kể cả khi đầu ra cuối là mã nguồn, nội dung, dữ liệu hay nghiên cứu.

Một báo cáo tốt cho CEO cần trả lời:

- nhân viên đã nhận nhiệm vụ gì và đã làm gì;
- kết luận/kết quả hiện tại là gì;
- bằng chứng hoặc artifact liên quan ở đâu;
- phần nào chưa chắc chắn, chưa làm, bị chặn hoặc cần CEO quyết định;
- công việc phụ nào được giao, dựa trên uỷ quyền nào, và kết quả của chúng;
- đề xuất bước tiếp theo, nếu có, tách biệt với việc đã hoàn thành.

**Nguyên tắc:** báo cáo là bằng chứng giao tiếp và đánh giá; nó không tự chứng minh thay đổi ngoài hệ thống. Nếu agent nói đã tạo/sửa/gửi thứ gì, báo cáo phải trỏ tới bằng chứng kiểm tra được.

### I-06 — Sổ cái điều hành (Operational Ledger)

**Ý tưởng:** Một lịch sử thống nhất, có thể truy vết, cho các sự kiện quan trọng: CEO giao việc, cấp/thu quyền, agent bắt đầu/kết thúc, sinh công việc phụ, báo cáo, duyệt, yêu cầu sửa, pause/cancel và lỗi.

- **Mục đích:** trả lời “chuyện gì đã xảy ra và tại sao?” chứ không chỉ lưu stream log kỹ thuật.
- **Phân biệt nguồn quyết định:** mỗi sự kiện cho biết do CEO, do agent theo uỷ quyền nào, hay do hệ thống ghi nhận.
- **Không cần choáng ngợp:** Command Desk chỉ bật ra các sự kiện CEO cần chú ý; ledger là chỗ đào sâu khi cần.
- **Bất biến và tự động ghi:** các sự kiện được hệ thống tự động ghi thành bản ghi chỉ-ghi-thêm (append-only). Không ai, kể cả CEO, được sửa hoặc xoá nội dung lịch sử đã ghi.
- **Đính chính thay vì sửa đè:** nếu một thông tin cũ sai hoặc cần làm rõ, hệ thống tạo một bản ghi đính chính mới liên kết tới bản ghi gốc, nêu người/nguồn đính chính, lý do và nội dung đúng. Lịch sử ban đầu vẫn còn nguyên để kiểm tra.
- **Vì sao phù hợp brief:** đây là điều kiện để CEO có thể kiểm toán và vẫn cho phép tự giao việc phụ không cần liên tục hiện lên màn hình.

### I-07 — Cây công việc phụ có thể mở theo yêu cầu

**Ý tưởng:** Công việc CEO giao có thể sinh ra một cây delegation phía dưới; mặc định có thể gọn, nhưng luôn mở được theo yêu cầu của CEO.

- Mỗi nhánh nêu người giao, người nhận, lý do, giới hạn uỷ quyền, tiến độ, báo cáo và quan hệ phụ thuộc.
- Một người quản lý AI chịu trách nhiệm tổng hợp và giám sát các nhánh mình tạo, thay vì đẩy toàn bộ gánh nặng điều phối cho CEO.
- Nếu CEO bật chế độ “duyệt trước”, các nhánh mới chỉ là đề xuất cho đến khi CEO đồng ý.
- **Vì sao phù hợp brief:** bảo toàn quyền “không cần show nếu CEO không yêu cầu” nhưng không đánh đổi khả năng can thiệp hay giải trình.

### I-08 — Văn phòng ảo như bản đồ trạng thái sống

**Ý tưởng:** Văn phòng ảo không là một giao diện thứ hai tách rời; nó là cách nhìn không gian của cùng dữ liệu trên Command Desk.

- Mỗi nhân viên có chỗ/nhóm làm việc đại diện cho danh tính, bộ phận và tầng quản lý.
- **Tổng bộ:** CEO đứng đầu tổng bộ; đây là trung tâm điều hành trực quan của tổ chức.
- **Văn phòng đơn tầng:** lựa chọn phù hợp quy mô nhỏ, cho phép CEO nhìn trực tiếp toàn bộ nhân sự và luồng trạng thái trong cùng một không gian.
- **Văn phòng đa tầng:** lựa chọn phù hợp khi nhân sự tăng lên; mỗi tầng hoặc khu vực có thể phản chiếu bộ phận/cấp quản lý, nhưng vẫn dẫn về cùng dữ liệu điều hành chung.
- **Đa chi nhánh (mở rộng sau):** khi tổ chức đủ lớn, CEO quản lý các chi nhánh gián tiếp qua một **Branch Director**. Mỗi chi nhánh có thể có cấu trúc văn phòng riêng nhưng vẫn thuộc một sổ cái điều hành và cơ chế uỷ quyền thống nhất của AI Corp.
- Cấu trúc văn phòng không được cố định theo kỹ thuật: CEO chọn cấu trúc phù hợp quy mô nhân sự và cách muốn điều hành; dữ liệu tổ chức là nguồn thật, văn phòng là bản đồ diễn giải của dữ liệu đó.
- Trạng thái hoạt động, chờ duyệt, blocked hoặc tạm dừng được phản chiếu bằng nhãn, biểu tượng và mô tả — không chỉ bằng màu.
- Nhấn vào một nhân viên hoặc khu vực luôn mở đúng hồ sơ, công việc, báo cáo hay quyết định tương ứng ở bàn điều hành.
- Văn phòng có thể tạo cảm giác “đội ngũ đang làm việc”, nhưng không được dùng để giấu quyền, log hoặc khả năng thao tác.
- **Vì sao phù hợp brief:** giữ bản sắc AI Corp mà không hy sinh tính vận hành và khả năng kiểm tra.

### I-09 — Hàng đợi “CEO Attention” thay cho thông báo tràn lan

**Ý tưởng:** Biến mọi sự kiện cần quyết định thành một hàng đợi hữu hạn, có ngữ cảnh; tránh CEO bị ngập thông báo từ agents.

Một mục chỉ xuất hiện khi cần một trong các phản hồi: duyệt/từ chối, cấp/thu quyền, chọn hướng xử lý, xem báo cáo, giải quyết blocker, hoặc xử lý lỗi.

- Mỗi mục nêu rõ: ai yêu cầu, công việc liên quan, lý do, tác động nếu không phản hồi và hành động CEO có thể thực hiện.
- Tín hiệu không cần quyết định ở lại ledger/báo cáo thay vì cạnh tranh sự chú ý.
- **Vì sao phù hợp brief:** trực tiếp trả lời “Cái gì cần tôi?” và duy trì CEO trong vai trò điều hành, không phải người đọc log toàn thời gian.

#### Hướng phát triển sau — Thư ký AI cho CEO

Khi hàng đợi CEO Attention đã đáng tin cậy, AI Corp có thể thêm một nhân viên “Thư ký” riêng cho CEO. Thư ký dùng AI để phân loại mức khẩn cấp: mục nào cần CEO duyệt ngay, mục nào có thể chờ; đồng thời có một kênh liên lạc/báo cáo riêng để liên hệ CEO khi thực sự gấp.

Thư ký không thay thế hàng đợi và không được tự che giấu sự kiện: CEO luôn có thể xem hàng đợi gốc, lý do phân loại và thu hồi quyền lọc/chuyển tiếp. Đây là ý tưởng phát triển sau, không làm chệch trọng tâm hoàn thiện CEO Attention trước.

### I-10 — Lớp kết nối agent thật, thay thế được

**Ý tưởng:** AI Corp giao tiếp với agent qua một lớp kết nối chuẩn, cho phép một nhân viên chạy thật bằng local model, API, coding agent, script hoặc MCP-compatible agent mà vẫn báo về cùng một mô hình công việc/báo cáo.

- **Điểm cần giữ:** identity, quyền, công việc và artifact thuộc về AI Corp; provider chỉ thực hiện.
- **Tín hiệu thực tế:** màn hình cần phân biệt “đã gửi lệnh”, “runtime đang chạy”, “runtime phản hồi”, “đã tạo báo cáo” và “đã được CEO duyệt”.
- **Ràng buộc:** không tuyên bố hỗ trợ mọi provider từ đầu; mỗi connector chỉ được thêm khi có thể báo trạng thái/bằng chứng đáng tin.
- **Vì sao phù hợp brief:** bản đầu cần nối agent thật, nhưng không bị buộc vào một model hoặc một nhà cung cấp.

> **Ghi chú lựa chọn kiến trúc bổ sung (CEO duyệt ngày 2026-11-06):** AI Corp dùng **Tauri + React/TypeScript + Rust + SQLite**. React/TypeScript là lớp điều hành và trực quan; Rust/Tauri là lớp domain, kiểm tra Hộp uỷ quyền, connector runtime và truy cập local; SQLite là nguồn dữ liệu local-first. Đây là quyết định triển khai cho các idea hiện có, không tạo một idea hoặc phiên bản Ideas mới.

### I-11 — Kinh nghiệm cá nhân có nguồn của Nhân viên AI

> **Ghi chú bổ sung:** CEO bổ sung idea này sau vòng duyệt idea ban đầu. Đây là phần bị sót khi duyệt, được thêm vào cùng định hướng hiện tại; không tạo một phiên bản Ideas mới.

**Ý tưởng:** Mỗi nhân viên AI có một lịch sử kinh nghiệm riêng, bền vững qua các công việc. Đây là lớp trí nhớ vận hành cá nhân: điều từng làm, bài học rút ra, quy ước đã học, lỗi từng gặp, kỹ năng đã được chứng minh và bối cảnh cần nhớ cho công việc sau.

Kinh nghiệm cá nhân **không phải** Sổ cái điều hành:

- Ledger là lịch sử bất biến của những gì đã xảy ra trong tổ chức.
- Kinh nghiệm là bản diễn giải có phạm vi của một nhân viên về những bài học nên áp dụng; mỗi mục phải liên kết về báo cáo, artifact hoặc sự kiện ledger làm nguồn.
- Một kinh nghiệm có thể bị đính chính, thay thế hoặc vô hiệu hoá bằng bản ghi mới, nhưng không được sửa đè lịch sử nguồn trong ledger.

Mỗi kinh nghiệm cần có bài học, phạm vi áp dụng, nguồn bằng chứng, thời điểm, trạng thái hiệu lực và ai/điều gì đã tạo hoặc xác nhận nó. CEO có thể xem, yêu cầu đính chính hoặc vô hiệu hoá kinh nghiệm. Khi nhận một công việc mới, agent chỉ nhận các kinh nghiệm liên quan; hệ thống không xem toàn bộ lịch sử dài là ngữ cảnh mặc định.

**Giới hạn bản đầu:** chưa tự chấm điểm năng lực, chưa tự tăng quyền, chưa tạo bộ nhớ vô hạn và chưa để agent tự khẳng định một bài học là đúng nếu thiếu nguồn. Kinh nghiệm hỗ trợ quyết định và thực hiện, không thay thế Hộp uỷ quyền hoặc bằng chứng.

**Vì sao phù hợp brief:** nhân viên có thể tích luỹ giá trị lâu dài mà CEO vẫn kiểm tra được nguồn gốc, giữ quyền kiểm soát và tránh lẫn bài học chủ quan với sự thật vận hành.

## 5. Những idea chưa nên là trọng tâm

Các ý tưởng sau có thể hữu ích về sau, nhưng chưa phù hợp để dẫn dắt định hướng đầu tiên vì không trực tiếp hoàn tất vòng điều hành trong brief:

- CEO agent tự tạo chiến lược công ty hoặc tự tuyển agent.
- Heartbeat chạy liên tục, lịch định kỳ phức tạp và hoạt động không cần giám sát.
- Đánh giá hiệu suất bằng điểm số giả định, bảng KPI trang trí hoặc “tính cách” agent quá sâu.
- Chat tự do giữa mọi agent như một mạng xã hội nội bộ.
- Marketplace agent, thanh toán, ví, chi phí doanh thu hoặc nhiều công ty/tổ chức.
- Văn phòng ảo 3D/game hoá phức tạp trước khi dữ liệu điều hành có thật.

Chúng chỉ đáng xem lại sau khi AI Corp chứng minh được: một agent thật nhận việc, làm trong quyền được cấp, bàn giao báo cáo có bằng chứng, và CEO có thể duyệt hoặc yêu cầu sửa mà không mất ngữ cảnh.

## 6. Bộ idea định hướng được đề xuất

Để giữ sản phẩm có trọng tâm, AI Corp nên giữ ba tầng idea song hành:

| Tầng | Các idea | Vai trò |
| --- | --- | --- |
| **Điều hành** | I-01 Command Desk, I-04 Công việc như hợp đồng, I-09 CEO Attention | Cho CEO thấy và quyết định việc quan trọng. |
| **Tin cậy & trách nhiệm** | I-02 Hồ sơ nhân viên, I-03 Hộp uỷ quyền, I-05 Báo cáo, I-06 Ledger, I-07 Cây công việc phụ, I-11 Kinh nghiệm cá nhân | Biến hành động agent thành thứ có thể hiểu, kiểm tra và can thiệp. |
| **Bản sắc & kết nối** | I-08 Văn phòng ảo, I-10 Lớp kết nối agent thật | Tạo trải nghiệm AI Corp riêng và bảo đảm đây là hệ thống vận hành thực. |

Không tầng nào thay thế tầng khác: văn phòng đẹp không bù được thiếu báo cáo; kết nối agent thật không bù được thiếu uỷ quyền; ledger đầy đủ không bù được thiếu khả năng CEO ra quyết định nhanh.

## 7. Các quyết định CEO đã duyệt

CEO đã duyệt các định hướng sau để làm cơ sở cho giai đoạn xác định Job:

1. **Hình thức uỷ quyền:** Hộp uỷ quyền phản ánh đúng cách CEO muốn cấp quyền theo chức vụ, công việc, thời gian, ngân sách và quyền tự duyệt.
2. **Báo cáo đầu tiên:** Sáu phần của báo cáo điều hành là đủ cho định hướng hiện tại.
3. **Mức hiển thị việc phụ:** Mặc định thu gọn nhưng luôn truy nguyên được là đúng; CEO có thể yêu cầu hiển thị hoặc duyệt trước bất cứ lúc nào.
4. **Sổ cái điều hành:** tự động ghi, bất biến và append-only; mọi sai sót phải được hiệu chỉnh bằng đính chính mới, không sửa đè.
5. **Văn phòng ảo:** phải có ý tưởng nghiêm túc cho văn phòng đơn tầng và đa tầng; đa chi nhánh với Branch Director là hướng phát triển sau.
6. **CEO Attention:** tiếp tục là lựa chọn mặc định; Thư ký AI là lớp phát triển sau để phân loại khẩn cấp và liên hệ CEO qua kênh riêng, dưới quyền giám sát đầy đủ của CEO.
7. **Kinh nghiệm nhân viên:** mỗi nhân viên có lịch sử kinh nghiệm riêng, có nguồn, độc lập với ledger và được bổ sung vào cùng định hướng hiện tại sau vòng duyệt idea ban đầu.
8. **Kiến trúc triển khai:** Tauri + React/TypeScript + Rust + SQLite là lựa chọn CEO đã duyệt; lớp UI không tự chạy lệnh, truy cập bí mật hoặc ghi trực tiếp vào nguồn dữ liệu điều hành.

Các idea trong tài liệu hiện đã đủ định hướng để chuyển sang giai đoạn xác định Job. Khi đó, từng job phải dẫn về ít nhất một idea mã I-01 đến I-11 và một nhu cầu cụ thể trong `brief.md`.
