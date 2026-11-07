# Hợp đồng precedence Hộp uỷ quyền

- **Contract ID:** `AIC-CON-AUTH-20261107-01`
- **Applies to:** J-01 Fix03 and Fix04.

## Luật

1. Policy theo **nhân viên** là lớp override giới hạn đối với cùng `action`.
2. Nếu có ít nhất một employee policy active cùng action, chỉ lớp này được xét. Không fallback sang role policy khi request bị giới hạn bởi scope, quota, expiry, delegate hoặc self-approve.
3. Nếu không có employee policy active cùng action, evaluator xét role policy của role hiện tại.
4. Narrow policy phải giữ nguyên subject kind và ID; successor chỉ là tập con giới hạn cũ.
5. Decision luôn có policy/source/limits hoặc denial code có thể đọc máy.
6. Mỗi policy có đúng một subject: **employee XOR role**. Policy theo role chỉ áp dụng nhân viên hiện đang mang role đó; policy theo employee không được lan sang người cùng role.
7. `authority_policy_versions` là đường quyết định quyền duy nhất. `authority_envelopes` chỉ là dữ liệu legacy để migration một lần, không có command/evaluator công khai.
8. Grant, narrow và revoke phải có event lịch sử append-only mang thời điểm, actor/nguồn, lý do và liên kết policy/version liên quan.

| Case | Kết quả |
| --- | --- |
| Role policy hợp lệ, không có employee override | Allow từ role. |
| Employee policy hợp lệ | Allow từ employee. |
| Employee policy cùng action nhưng scope/quota không khớp, role rộng hơn | Deny `employee_override_denied`; không fallback. |
| Employee policy không có action đó, role hợp lệ | Allow từ role. |
| Request gửi cả employee và role | Reject trước khi ghi; không có policy mơ hồ. |
| Legacy envelope tồn tại trước Fix04 | Migrate một lần sang policy version; chỉ evaluator policy version được dùng. |
