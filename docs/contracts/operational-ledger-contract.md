# Hợp đồng Sổ cái Điều hành — AI Corp

- **Document ID:** `AIC-CON-LED-20261106-01`
- **Created:** 2026-11-06 (Asia/Bangkok)
- **Task:** [`Tasks.md`](../../Tasks.md#t-0201--xác-định-sự-kiện-ledger-và-liên-kết-nguồn) — `T-02.01`
- **Inputs:** [`brief.md`](../../brief.md) mục 5.3, 11; [`ideas.md`](../../ideas.md) I-05, I-06; hợp đồng tổ chức [`organization-domain-contract.md`](organization-domain-contract.md).
- **Status:** Approved implementation contract for J-02.

## 1. Mục đích và ranh giới

Operational Ledger là nguồn lịch sử tổ chức **tự động ghi, append-only và truy nguyên được**. Nó trả lời: *chuyện gì đã xảy ra, do ai/nguồn nào, theo quyền nào, trên đối tượng nào, và bằng chứng nằm ở đâu?*

Ledger không phải stream log kỹ thuật, không phải trạng thái hiện thời của công việc, và không phải bộ nhớ kinh nghiệm nhân viên. Những dữ liệu đó có thể liên kết tới ledger, nhưng không được thay thế nó.

Không đường dùng nào — kể cả CEO, agent hay UI — được cập nhật hoặc xoá một event đã ghi. Sai sót chỉ được nêu bằng event `ledger.correction_recorded` mới liên kết event gốc.

## 2. ID và liên kết chuẩn

| Đối tượng | Prefix | Ví dụ | Quy tắc |
| --- | --- | --- | --- |
| Ledger event | `led_` | `led_01j9x8m1q4r7t2v5w6y3z0a9bc` | ID bền vững, chỉ sinh một lần khi append. |
| Artifact | `art_` | `art_01j9x8n2s5u8v3w6x7y4z1b0cd` | Đầu ra có thể mở/kiểm tra; metadata riêng, không nhúng nội dung bí mật vào event. |
| Report | `rpt_` | `rpt_01j9x8p3t6v9w4x7y8z5a2b1de` | Báo cáo điều hành sáu phần; có thể tham chiếu nhiều event/artifact. |
| Work item | `work_` | Theo hợp đồng J-03 | Chỉ liên kết nếu đã tồn tại; J-02 không tự tạo công việc. |
| Authority envelope | `auth_` | Theo J-01 | Bắt buộc khi agent thực hiện hành vi cần quyền. |

Mỗi tham chiếu là một cặp `{ kind, id }`. `kind` bắt buộc giúp tránh coi một chuỗi ID là bằng chứng không rõ loại. URI/path artifact phải nằm trong `evidenceRefs`, không nằm trong tiêu đề hay ghi chú tự do.

## 3. Cấu trúc event chuẩn

```json
{
  "id": "led_01j9x8m1q4r7t2v5w6y3z0a9bc",
  "occurredAt": "2026-11-06T11:30:00.000Z",
  "recordedAt": "2026-11-06T11:30:00.081Z",
  "eventType": "authority.granted",
  "actor": {
    "kind": "ceo",
    "employeeId": null,
    "displayNameAtEvent": "CEO"
  },
  "source": {
    "kind": "user_action",
    "channel": "ceo_profile",
    "runtimeBindingId": null
  },
  "subjectRefs": [
    { "kind": "employee", "id": "emp_ops_001" },
    { "kind": "authority_envelope", "id": "auth_01j9x8k0p3q6s1u4v5w2x9z8ab" }
  ],
  "context": {
    "organizationId": "org_aicorp_hq",
    "workItemId": null,
    "authorityEnvelopeId": "auth_01j9x8k0p3q6s1u4v5w2x9z8ab",
    "parentEventId": null,
    "correctionOfEventId": null
  },
  "summary": "CEO cấp quyền chuẩn bị báo cáo cho An.",
  "details": { "action": "prepare_report", "scope": "reports/*" },
  "evidenceRefs": [],
  "schemaVersion": 1
}
```

### 3.1 Bất biến theo field

Sau khi ghi thành công, mọi field trên là bất biến, gồm cả `summary`, `details`, `evidenceRefs`, `occurredAt` và các liên kết. Không thêm evidence bằng cách sửa event cũ; ghi event mới (`artifact.attached` hoặc `ledger.correction_recorded`) liên kết event liên quan.

`occurredAt` là thời điểm sự kiện được cho là diễn ra; `recordedAt` là thời điểm AI Corp append event. Nếu thời điểm diễn ra chỉ là ước tính, `details.timeConfidence` phải nêu `estimated` cùng lý do.

### 3.2 Tác nhân và nguồn

| Actor kind | Ý nghĩa | Ví dụ source kind hợp lệ |
| --- | --- | --- |
| `ceo` | CEO đưa quyết định/thao tác. | `user_action` |
| `employee` | Nhân viên AI hành động trong phạm vi uỷ quyền. | `agent_runtime`, `system` |
| `system` | AI Corp tự ghi nhận một chuyển trạng thái/quy tắc. | `system` |
| `external` | Hệ thống ngoài gửi tín hiệu đã xác thực. | `connector_callback` |

`actor.kind = employee` bắt buộc có `employeeId`; hành vi cần quyền phải có `context.authorityEnvelopeId`. Local/API/provider không tự biến thành actor hay quyền hợp lệ.

## 4. Nhóm event khởi đầu

| Nhóm | Event type | Tác nhân chính | Subject tối thiểu | Mục đích |
| --- | --- | --- | --- | --- |
| Quyền | `authority.granted`, `authority.narrowed`, `authority.revoked` | CEO | employee, authority_envelope | Kiểm toán cấp/giảm/thu quyền. |
| Nhân sự | `employee.created`, `employee.updated`, `runtime.bound`, `runtime.unbound` | CEO/system | employee; runtime nếu có | Giữ lịch sử hồ sơ và runtime. |
| Công việc (J-03) | `work.created`, `work.assigned`, `work.paused`, `work.cancelled` | CEO/employee | work, employee | Truy nguyên điều hành. |
| Runtime (J-03) | `execution.dispatched`, `execution.started`, `execution.responded`, `execution.failed` | system/employee | work, employee | Tách rõ gửi lệnh/chạy/phản hồi/lỗi. |
| Báo cáo | `report.created`, `report.submitted`, `report.approved`, `report.revision_requested` | employee/CEO | report, work nếu có | Không đồng nhất report với CEO duyệt. |
| Artifact | `artifact.registered`, `artifact.attached` | employee/system | artifact, report/work | Liên kết bằng chứng kiểm tra được. |
| Delegation (J-04) | `delegation.proposed`, `delegation.approved`, `delegation.created`, `delegation.blocked` | employee/CEO/system | work cha/con, employee | Giải trình việc phụ. |
| Đính chính | `ledger.correction_recorded` | CEO/employee/system | event gốc và event đính chính | Sửa nghĩa bằng event mới, không sửa record cũ. |

Các nhóm tương lai không được dùng event type mơ hồ như `updated`, `done` hoặc `message`. Chúng phải thêm type có ngữ nghĩa, version schema và subject/evidence tối thiểu.

## 5. Liên kết báo cáo và artifact

Một report có ID `rpt_` phải liên kết ít nhất một `led_` chứng minh lúc tạo/nộp report. Report chứa sáu phần đã duyệt:

1. nhiệm vụ/việc đã làm;
2. kết quả hiện tại;
3. bằng chứng/artifact;
4. điều chưa chắc, blocker hoặc điều cần CEO;
5. công việc phụ cùng quyền liên quan;
6. đề xuất bước tiếp theo, tách khỏi phần đã hoàn thành.

`artifact.registered` yêu cầu `evidenceRefs` có metadata tối thiểu: `artifactId`, `kind`, `location`, `integrityHint` (ví dụ hash hoặc revision), `openability` và `createdBy`. Nếu không có artifact, report phải nói rõ lý do thay vì khẳng định kết quả bên ngoài hệ thống.

## 6. Đính chính

Ví dụ: source của event `led_...abc` ghi nhầm runtime.

```json
{
  "eventType": "ledger.correction_recorded",
  "actor": { "kind": "ceo", "employeeId": null, "displayNameAtEvent": "CEO" },
  "context": { "organizationId": "org_aicorp_hq", "correctionOfEventId": "led_01j9x8m1q4r7t2v5w6y3z0a9bc" },
  "summary": "Đính chính nguồn runtime của event led_…abc.",
  "details": {
    "reason": "Connector ghi nhầm local preview thay vì runtime đã cấu hình.",
    "correctedFields": { "source.runtimeBindingId": "rtb_correct" }
  }
}
```

Điều kiện: event gốc phải tồn tại; correction không được mang cùng ID event gốc; lý do và actor là bắt buộc. Màn hình đọc lịch sử phải hiển thị cả hai, có liên kết hai chiều, và không làm biến mất nội dung gốc.

## 7. Trường hợp bị từ chối

| Thao tác | Lý do từ chối |
| --- | --- |
| `UPDATE ledger_events SET summary = ...` | Vi phạm append-only. |
| `DELETE FROM ledger_events ...` | Vi phạm lịch sử, kể cả CEO. |
| Agent ghi `report.approved` thay CEO khi không có quyền tự duyệt cụ thể. | Sai actor/quyền. |
| `artifact.registered` chỉ có câu “đã xong”, không có metadata mở/kiểm tra. | Không đủ bằng chứng. |
| Correction không có `correctionOfEventId` hoặc lý do. | Không truy nguyên được sai sót. |
| Event runtime tự đặt `work completed`. | Runtime phản hồi không đồng nghĩa CEO duyệt. |

## 8. Tiêu chí bàn giao sang T-02.02

T-02.02 phải hiện thực đúng hợp đồng này: Rust là điểm append duy nhất; UI không có API update/delete ledger; event có actor/source/context/reference; correction là event mới. Test phải chứng minh event cũ không đổi sau mọi đường thao tác và mở lại SQLite vẫn giữ lịch sử.
