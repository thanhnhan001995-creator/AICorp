# Hợp đồng Runtime Binding J-01

- **Contract ID:** `AIC-CON-RUNTIME-20261107-01`
- **Applies to:** J-01 Fix05.

## Ranh giới

1. Runtime binding ở J-01 là **seed-only và immutable**: UI không có API create/update/delete runtime binding.
2. Mọi seed/insert nội bộ phải gọi `insert_seed_runtime_binding`, validate `provider_kind` và `display_name` trước persistence.
3. Giá trị có marker secret-like (`api_key`, `secret`, `token`, `password`, `Bearer`, `sk-`, PEM header) bị từ chối trong trường hiển thị.
4. `connection_reference` là technical reference nội bộ: không có trong `RuntimeBinding` DTO, query `list_runtime_bindings` hoặc React state.
5. Khi J-03 bổ sung quản trị connector/runtime thật, mọi write API phải thay thế seed-only boundary này và giữ nguyên luật 2–4; không được truyền credential về UI.
