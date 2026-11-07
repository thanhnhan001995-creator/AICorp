# Product Brief — AI Corp

- **Document ID:** `AIC-BRF-20261105-01`
- **Created:** 2026-11-05 (Asia/Bangkok)
- **Last updated:** 2026-11-06 (Asia/Bangkok)

## 1. Tình trạng tài liệu

- **Tên sản phẩm tạm thời:** AI Corp
- **Trạng thái:** bản brief khởi đầu để duyệt
- **Phạm vi tài liệu này:** chỉ xác định vấn đề, người dùng, kết quả mong muốn, nguyên tắc và ranh giới sản phẩm.
- **Không thuộc phạm vi hiện tại:** ý tưởng tính năng, job, task, roadmap kỹ thuật, lựa chọn stack, thiết kế màn hình chi tiết hoặc kế hoạch triển khai.

Mọi đề xuất sau này phải giải quyết một nhu cầu hoặc ràng buộc đã ghi ở brief này. Nếu không, đề xuất đó phải được bổ sung và duyệt lại ở brief trước.

## 2. Tuyên bố sản phẩm

**AI Corp là một ứng dụng local-first giúp CEO tạo, tổ chức, giao việc, quan sát và duyệt kết quả của các tác nhân AI như đang điều hành một công ty đa chức năng.**

Ứng dụng biến nhiều phiên AI, công cụ AI hoặc quy trình tự động rời rạc thành một môi trường làm việc có:

- danh tính và trách nhiệm rõ cho từng tác nhân;
- công việc có chủ sở hữu, trạng thái và bối cảnh;
- kết quả có thể xem, duyệt, yêu cầu sửa và truy vết;
- quyền quyết định cuối cùng thuộc về CEO.

Đây không phải một chatbot tổng quát, cũng không phải lời hứa về “công ty tự vận hành”. Đây là một bàn điều hành minh bạch cho con người quản lý lực lượng AI.

## 3. Vấn đề cần giải quyết

Khi dùng nhiều AI agents hoặc công cụ AI, CEO thường gặp các vấn đề sau:

1. **Mất bối cảnh:** không còn biết agent nào đang làm gì, vì sao làm và thông tin nào đã được dùng.
2. **Không có trách nhiệm rõ:** một câu trả lời hoặc log chat không nói rõ ai chịu trách nhiệm cho phần việc nào.
3. **Không có trạng thái đáng tin:** “đã xong” có thể chỉ là agent tự nhận; người dùng không biết đã có kết quả để kiểm tra hay chưa.
4. **Khó kiểm soát hành động:** chạy lệnh, sửa tệp, gọi API hoặc gửi dữ liệu ra ngoài có thể vượt khỏi mức người điều hành muốn cho phép.
5. **Kết quả rời rạc:** artifact, nhận xét, yêu cầu sửa và lịch sử quyết định nằm ở nhiều cửa sổ/công cụ khác nhau.
6. **Khó mở rộng có kiểm soát:** thêm agent khiến việc phối hợp, chi phí, quyền hạn và trách nhiệm rối hơn thay vì hữu ích hơn.

## 4. Người dùng chính

### CEO / Người điều hành (primary user)

Một cá nhân hoặc nhóm rất nhỏ sử dụng nhiều AI tác nhân để làm nghiên cứu, nội dung, lập trình, kiểm thử hoặc xử lý tác vụ lặp lại.

CEO cần:

- nhìn được tình trạng hoạt động thay vì tự nhớ qua nhiều tab;
- chủ động giao việc và đổi ưu tiên;
- xem bằng chứng trước khi chấp nhận kết quả;
- biết lúc nào cần can thiệp;
- giữ dữ liệu và quyền kiểm soát trong máy của mình ở giai đoạn đầu.

### Tác nhân AI (secondary user / managed actor)

Một tác nhân có thể là model local, AI API, coding agent, script hay công cụ tương thích trong tương lai. Trong sản phẩm, tác nhân được mô hình hoá như một “nhân viên” có giới hạn, không phải một con người thật. Mức quyền của nhân viên không được suy ra từ việc nó chạy local hay qua API; nó do CEO cấp theo chức vụ, phạm vi công việc và chính sách đang hiệu lực.

## 5. Giá trị cốt lõi

### 5.1. Văn phòng trực quan

CEO có thể hiểu tình hình của đội ngũ trong vài giây: ai rảnh, ai đang làm, ai chờ duyệt và phần việc nào bị chặn. Bảng điều hành với nhân viên có danh tính là bề mặt điều hành chính. Văn phòng ảo trực quan là bề mặt bổ trợ, giúp việc điều hành sinh động và bớt nhàm chán, nhưng không được che khuất dữ liệu vận hành hoặc trở thành nơi duy nhất để thao tác.

### 5.2. Quyền điều hành trực tiếp

CEO có thể giao việc, điều chỉnh mức ưu tiên, tạm dừng, huỷ, duyệt, yêu cầu sửa và thay đổi quyền theo ngữ cảnh. Quyền hạn được cấp theo cấp chức vụ của nhân viên AI, giới hạn công việc (thời gian, ngân sách, quyền thao tác...) và chỉ định riêng của CEO. Không có hành động nhạy cảm nào bị coi là mặc định an toàn chỉ vì agent đề xuất nó.

### 5.3. Công việc phải có bằng chứng

Báo cáo là artifact trung tâm cho tình huống dùng đầu tiên: CEO phải có thể nhận, đọc, yêu cầu sửa và duyệt một báo cáo ngay cả khi sản phẩm cuối chưa xuất hiện. Khi công việc khẳng định đã tạo tác động ngoài hệ thống hoặc đã có sản phẩm cuối, báo cáo cần trỏ tới bằng chứng kiểm tra được tương ứng: tệp, liên kết, dữ liệu, thay đổi mã nguồn hoặc bản ghi thực thi. Nội dung hội thoại đơn thuần không đủ để xác nhận hoàn thành.

### 5.4. Minh bạch hơn tự động hoá

Hệ thống phải ưu tiên cho người dùng hiểu được: ai đã làm gì, dùng nguồn/công cụ nào, trạng thái vì sao đổi, và quyết định nào do AI hay do người đưa ra.

### 5.5. Local-first, mở đường cho nhiều nguồn AI

Dữ liệu điều hành cơ bản, lịch sử công việc và artifact metadata ưu tiên nằm tại máy local. Sản phẩm không được khóa chặt vào một model hay một nhà cung cấp AI.

## 6. Kết quả mong muốn

Khi sản phẩm hoạt động đúng, CEO có thể:

- tạo và duy trì một đội ngũ tác nhân với tên, vai trò, khả năng và quyền hạn rõ ràng;
- chuyển một mục tiêu thành công việc có trách nhiệm cụ thể;
- theo dõi tiến trình mà không cần mở từng phiên AI riêng lẻ;
- nhận biết phần việc cần mình quyết định;
- xem artifact và lịch sử hành động trước khi duyệt;
- yêu cầu chỉnh sửa với bối cảnh không bị thất lạc;
- dừng một agent hoặc một công việc trước khi nó tạo thêm ảnh hưởng;
- truy nguyên từ kết quả về công việc, tác nhân, dữ liệu đầu vào và các quyết định liên quan.

## 7. Mô hình khái niệm bắt buộc

Các khái niệm dưới đây cần được phân biệt trong toàn bộ sản phẩm. Đây là ngôn ngữ chung, chưa phải mô hình dữ liệu hay thiết kế kỹ thuật.

| Khái niệm | Ý nghĩa sản phẩm |
| --- | --- |
| **Tổ chức** | Không gian điều hành bao gồm CEO, đội ngũ, công việc và lịch sử chung. |
| **Nhân viên AI** | Danh tính hoạt động lâu dài của một tác nhân: vai trò, năng lực, giới hạn và lịch sử. |
| **Vai trò** | Trách nhiệm kỳ vọng của nhân viên, không đồng nghĩa với model hoặc công cụ cụ thể. |
| **Công việc** | Một cam kết có mục tiêu, người chịu trách nhiệm, đầu vào, trạng thái và tiêu chí kết quả. |
| **Phiên thực hiện** | Một lần nhân viên thực sự bắt đầu xử lý công việc; có log và kết quả riêng. |
| **Artifact** | Đầu ra có thể mở/kiểm tra/duyệt: tệp, báo cáo, dữ liệu, thay đổi mã, liên kết hoặc kết quả có cấu trúc. |
| **Quyết định** | Sự chấp thuận, từ chối, yêu cầu sửa, tạm dừng hoặc thay đổi hướng do CEO đưa ra, hoặc do nhân viên được CEO uỷ quyền trong phạm vi rõ ràng. |
| **Quyền hạn** | Những hành động một nhân viên được phép đề xuất, thực hiện, giao việc phụ hoặc tự duyệt trong một ngữ cảnh cụ thể; do CEO cấp theo chức vụ, công việc và chính sách. |
| **Trạng thái** | Sự thật vận hành hiện thời của nhân viên, công việc hay phiên thực hiện; không là nhãn trang trí. |

## 8. Nguyên tắc trạng thái và hoàn thành

- Một nhân viên có trạng thái riêng với một công việc/phiên thực hiện. “Nhân viên đang bận” không tự động có nghĩa là công việc đã tiến triển.
- Một công việc phải cho biết đang chờ ai hoặc bị chặn bởi điều gì.
- “Hoàn tất” là trạng thái do hệ thống và CEO xác nhận theo bằng chứng, trừ khi CEO đã uỷ quyền quyền duyệt trong phạm vi rõ ràng; không chỉ dựa vào tuyên bố của agent.
- Kết quả cần phản hồi phải có trạng thái riêng với việc chạy: ví dụ agent hoàn thành phần làm nhưng đang chờ người duyệt.
- Mọi thay đổi quan trọng phải để lại lịch sử đọc được.

## 9. Kiểm soát và an toàn

Sản phẩm cần khiến CEO hiểu rõ phạm vi hoạt động và giữ quyền can thiệp:

- Phân biệt **xem/đề xuất** với **thực thi/thay đổi**.
- Phân biệt rõ hành động local với hành động sử dụng Internet, API, tài khoản hoặc dữ liệu bên ngoài.
- Hành động có ảnh hưởng khó đảo ngược phải có điểm duyệt của CEO, hoặc một chính sách/uỷ quyền mà CEO đã cấp rõ ràng.
- Có khả năng tạm dừng và hủy công việc; lịch sử vẫn được giữ lại để xem xét.
- CEO có thể yêu cầu bất kỳ nhân viên nào hiển thị công việc phụ, nêu lý do giao việc, hoặc chuyển sang chế độ cần CEO duyệt trước khi thực hiện. Nhân viên không được từ chối yêu cầu này.
- Không hiển thị bí mật, khoá API hay dữ liệu nhạy cảm trong log hoặc artifact một cách vô tình.
- Cần nói đúng mức độ chắc chắn: một log thành công không tự chứng minh một tác động ngoài hệ thống đã diễn ra như mong muốn.

## 10. Tiêu chí trải nghiệm

Sản phẩm cần tạo cảm giác CEO đang điều hành một văn phòng nhỏ, nhưng không được hi sinh khả năng kiểm tra:

- Mỗi thông tin trực quan phải có đường dẫn tới dữ liệu và lịch sử chi tiết.
- Trạng thái quan trọng không được chỉ biểu diễn bằng màu; phải có nhãn và ngữ cảnh.
- Người dùng không cần nhớ lệnh hay mở terminal để biết cần xử lý gì tiếp theo.
- Chi tiết chỉ xuất hiện khi cần; tổng quan luôn trả lời được: “Đội đang làm gì? Cái gì cần tôi? Kết quả nào đang chờ duyệt?”
- Ngôn ngữ nên hướng đến vận hành công việc, tránh nhân hoá quá mức khiến người dùng nhầm AI có năng lực hoặc trách nhiệm như con người.

## 11. Phạm vi ban đầu

Brief này xác định sản phẩm cần làm tốt một vòng điều hành khép kín:

> CEO xác định mục tiêu → giao một công việc rõ ràng → một nhân viên AI thật xử lý trong phạm vi được phép → tạo báo cáo/artifact và lịch sử → CEO kiểm tra, duyệt hoặc yêu cầu sửa.

Vòng này là tiêu chuẩn để đánh giá mọi ý tưởng ở các bước tiếp theo. Một tính năng chỉ nên được ưu tiên nếu nó làm vòng này rõ hơn, an toàn hơn hoặc hữu ích hơn.

## 12. Ngoài phạm vi ở giai đoạn brief ban đầu

Các điều sau chưa được giả định là yêu cầu sản phẩm:

- doanh nghiệp tự vận hành hoàn toàn hoặc không cần CEO kiểm soát;
- nhân viên AI tự tiêu tiền, gửi tin nhắn, triển khai sản phẩm hoặc truy cập hệ thống nhạy cảm;
- mô phỏng văn phòng/game hoá phức tạp;
- nhiều người dùng, phân quyền tổ chức lớn hoặc cộng tác cloud;
- marketplace agent, ví điện tử, thanh toán hoặc theo dõi lợi nhuận;
- tự động tạo/chia nhỏ công việc mà không có khả năng truy nguyên lý do, phạm vi uỷ quyền và giám sát;
- hỗ trợ tất cả provider và mọi loại tool từ ngày đầu.

Những mục này có thể được xem xét sau, nhưng cần chứng minh chúng phục vụ giá trị cốt lõi và không làm mất quyền điều hành trực tiếp.

## 13. Chính sách uỷ quyền và công việc phụ

Một nhân viên AI chỉ được tự giao công việc phụ khi CEO đã cấp quyền này. Khi được cấp, nhân viên có thể tự quyết định và tự duyệt trong giới hạn CEO thiết lập, gồm ít nhất phạm vi yêu cầu, thời gian, ngân sách, quyền thao tác và các chỉ định riêng.

- Mặc định của một quyền uỷ quyền không được suy đoán từ model, provider hay việc chạy local/API; nó phải được CEO quy định rõ cho nhân viên hoặc công việc.
- Nhân viên được uỷ quyền có thể không cần chủ động hiển thị từng công việc phụ nếu CEO không yêu cầu.
- Tuy vậy, hệ thống phải luôn lưu được lý do giao việc, người giao, giới hạn uỷ quyền, tiến độ và kết quả để nhân viên có thể báo cáo với CEO bất cứ lúc nào.
- Nhân viên giao việc phụ vẫn chịu trách nhiệm giám sát tiến độ của các công việc đó và tổng hợp chúng vào báo cáo cho CEO.
- CEO có quyền xem toàn bộ công việc phụ, yêu cầu giải trình, thu hồi uỷ quyền hoặc yêu cầu duyệt trước ở bất cứ thời điểm nào. Khi yêu cầu này có hiệu lực, nhân viên phải tuân thủ.

## 14. Câu hỏi còn mở của brief

Các quyết định dưới đây cố ý chưa được tự trả lời bằng thiết kế hay task:

1. **Chính sách dữ liệu local và provider:** dữ liệu nào buộc phải nằm hoàn toàn local; dữ liệu nào được gửi qua provider AI khi CEO cho phép; và CEO sẽ cấp/thu hồi sự cho phép đó ở cấp tổ chức, nhân viên hay từng công việc?
2. **Mức xác minh báo cáo:** đối với từng loại công việc hỗn hợp, đâu là mức bằng chứng tối thiểu để CEO duyệt báo cáo khi chưa có sản phẩm cuối?
3. **Mức tự trị lúc khởi đầu:** dù hệ thống kết nối agent thật ngay từ bản đầu, CEO sẽ muốn mức uỷ quyền mẫu ban đầu cho các chức vụ là thận trọng, cân bằng hay rộng rãi?

## 15. Tiêu chuẩn duyệt brief

Brief được coi là sẵn sàng để chuyển sang giai đoạn **tìm idea** khi CEO đồng ý rằng:

- tuyên bố sản phẩm phản ánh đúng thứ muốn xây;
- vấn đề và người dùng chính là đúng;
- các giá trị cốt lõi và ranh giới an toàn là chấp nhận được;
- vòng điều hành khép kín ở mục 11 là điểm xuất phát phù hợp;
- chính sách uỷ quyền ở mục 13 phản ánh đúng quyền điều hành CEO mong muốn;
- các câu hỏi còn mở ở mục 14 đã đủ rõ để dùng làm tiêu chí đánh giá idea.

Không bắt đầu lập job hoặc task trước khi brief được duyệt hoặc được chỉnh sửa rõ ràng.
