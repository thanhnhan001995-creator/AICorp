# Kiểm tra hồi tố J-01 — Lần 7

- **Review ID:** `AIC-REV-J01-20261107-07`
- **Ngày:** 07/11/2026 (Asia/Bangkok)
- **Phạm vi:** J-01 sau Fix08.
- **Nguồn:** [`brief.md`](../../brief.md), [`ideas.md`](../../ideas.md), [`Jobs.md`](../../Jobs.md), [Task Fix08](../../Tasks/J01/T-01-Fix08.md), [Execute Fix08](../../Executive/J01/Exe-T-01-Fix08.md), [Evidence Fix08](../../evidence/J01/envidence-T-01-Fix08.md), [AcceptanceReport lượt 3](../../AcceptanceReport/J01-AccRep/AcceptanceReport-J01-3rd.md).

## Đối chiếu hồi tố

| Hạng mục | Yêu cầu | Trạng thái mới nhất | Kết luận |
| --- | --- | --- | --- |
| Thu hẹp quyền | Giới hạn policy, giữ chuỗi truy nguyên. | Successor, `supersedesId`, narrow/grant history đã có; regression pass. | Pass kỹ thuật. |
| Thu hồi quyền | Thu hồi policy hợp lệ có lý do, không sửa đè lịch sử. | Successor `active` chuyển `revoked`, event cùng transaction; rollback event được kiểm. | Pass kỹ thuật. |
| Lỗi điều hành có thể hành động | CEO thấy được ranh giới quyền. | Lỗi trả policy ID/status và hướng chọn successor active. | Pass kỹ thuật. |
| Runtime/UI thật | Build/test không thay CEO kiểm tra. | Lượt 3 bị `Rejected`; Fix08 chưa có kiểm tra UI mới. | Chưa kiểm tra. |

## Kết quả kiểm tra kỹ thuật

- Rust regression: 9/9 pass.
- TypeScript type-check và production build: pass.
- `cargo fmt --check` fail do định dạng lịch sử toàn dự án; không thuộc regression chức năng và không được sửa trong Fix08.
- Không có policy ID/raw error từ lượt 3, do đó không tuyên bố đã tái lập chính xác dữ liệu runtime cũ.

## Kết luận

**J-01 sẵn sàng kỹ thuật cho CEO kiểm tra lại runtime/UI ở lượt 4; chưa được nghiệm thu.**

Khi kiểm tra, CEO cần cấp policy, thu hẹp, nhập lý do rồi thu hồi **policy kế nhiệm có status `active`**. Nếu còn fail, ghi policy ID/status và nguyên văn thông báo để Fix tiếp theo có thể định vị chính xác.
