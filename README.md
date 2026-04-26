# Embedding API Server

Embedding API Server là dịch vụ API cho embedding model ONNX, được viết bằng Rust để tăng hiệu năng, giảm độ trễ khởi động và dễ triển khai.

## Mục tiêu

- Chạy inference ONNX với hiệu năng cao.
- Hỗ trợ nhiều định dạng API phổ biến: OpenAI và Ollama.
- Hỗ trợ nhiều kiểu đầu ra: dense embedding, sparse embedding, ColBERT embedding và đếm token.
- Quản lý model theo thư mục local, phù hợp cho triển khai on-premise.

## Tech Stack

- `Rust`: ngôn ngữ triển khai chính.
- `Axum`: framework HTTP API.
- `Tokio`: async runtime.
- `Serde`: serialize/deserialize JSON.
- `ONNX Runtime`: suy luận model ONNX.
- `Tokenizers`: đếm token và xử lý tokenizer.
- `Tower` / `tower-http`: middleware, CORS, timeout, compression.
- `Tracing`: logging và observability.
- `Docker`: đóng gói và triển khai.

## Phương án triển khai

- Tổ chức model theo registry nội bộ, load lazy theo nhu cầu.
- Tách capability theo từng nhóm model: embedding, sparse, ColBERT, rerank.
- Chuẩn hóa contract API bằng OpenAPI trong `Docs/ApiDocument.md`.
- Ưu tiên binary đơn, chạy ổn định trong container, mount trực tiếp `AI_Models/`.

## API

### OpenAI compatible

- `POST /v1/embeddings`
- `GET /v1/models`

### Internal / Ollama compatible

- `POST /api/embeddings`
- `POST /api/embeddings/sparse`
- `POST /api/embeddings/colbert`
- `POST /api/rerank`
- `POST /api/tokens/count`

### Health

- `GET /health`

## Request / Response

Tài liệu chi tiết về schema và response format nằm trong `Docs/ApiDocument.md`.

Một số kiểu dữ liệu chính:

- OpenAI embedding request/response
- OpenAI model list response
- Ollama embedding request/response
- Token count request/response
- Sparse embedding request/response
- ColBERT embedding request/response
- Rerank request/response
- Health response

## Cấu trúc model

Model được lưu cục bộ trong thư mục `AI_Models/`.

Ví dụ:

```text
AI_Models/
├── BGE-M3/
├── dangvantuan-vietnamese-document-embedding/
├── qwen3-embedding-0.6B/
└── bge-reranker-v2-m3-ONNX/
```

## Cấu hình

Các cấu hình runtime được đọc từ biến môi trường hoặc file `.env`.

- `MODELS_DIR`: thư mục chứa các model.
- `DEFAULT_MODEL`: model mặc định khi request không chỉ định model.

## Chạy dự án

Khi mã nguồn Rust đã sẵn sàng, có thể chạy theo workflow chuẩn của Cargo:

```bash
cargo build --release
cargo run --release
```

## Chạy bằng Docker

```bash
docker compose up --build
```

Mặc định container sẽ mount `./AI_Models` vào `/app/AI_Models` và mở cổng `8000`.
Host port mặc định của Docker Compose là `34749`.

## Ví dụ gọi API

```bash
curl -X POST "http://localhost:34749/v1/embeddings" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "qwen3-embedding-0.6B",
    "input": "Xin chào, tôi là trợ lý AI."
  }'
```

## Tài liệu API

Xem đầy đủ trong `Docs/ApiDocument.md`.
