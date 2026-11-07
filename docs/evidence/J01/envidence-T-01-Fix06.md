# Evidence kỹ thuật — T-01 Fix06

- **Evidence ID:** `AIC-EVD-FIX06-20261107-01`
- **Task referenced:** [T-01 Fix06](../../Tasks/J01/T-01-Fix06.md)
- **Boundary:** evidence cho đường khởi chạy nghiệm thu; không phải CEO runtime/UI acceptance.

| Kiểm tra | Kết quả |
| --- | --- |
| `npm exec tauri -- --version` tại `D:\Moi-Truong_AI\app` | Pass: `tauri-cli 2.12.4`. |
| `package.json` scripts | Không có script `tauri`; lỗi `npm run tauri dev` trong AcceptanceReport-J01-1st là đúng và được giữ nguyên. |
| Lệnh replacement | `npx tauri dev` gọi Tauri CLI từ dependency dự án; đây là command thực cho Command Prompt. |

## Handoff CEO

CEO chạy lệnh trong [AcceptanceReport-J01-2nd](../../AcceptanceReport/J01-AccRep/AcceptanceReport-J01-2nd.md). Chỉ CEO runtime/UI interaction mới quyết định Accepted/Rejected.
