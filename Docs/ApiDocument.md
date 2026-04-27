openapi: 3.1.0
info:
  title: Embedding API Server
  version: 0.2.0
paths:
  /v1/embeddings:
    post:
      summary: Openai Embeddings
      operationId: openai_embeddings_v1_embeddings_post
      requestBody:
        content:
          application/json:
            schema:
              $ref: "#/components/schemas/OpenAIEmbeddingRequest"
        required: true
      responses:
        "200":
          description: Successful Response
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/OpenAIEmbeddingResponse"
        "422":
          description: Validation Error
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/HTTPValidationError"
  /v1/models:
    get:
      summary: List Models
      operationId: list_models_v1_models_get
      responses:
        "200":
          description: Successful Response
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/OpenAIListModelResponse"
  /api/embeddings:
    post:
      summary: Ollama Embeddings
      operationId: ollama_embeddings_api_embeddings_post
      requestBody:
        content:
          application/json:
            schema:
              $ref: "#/components/schemas/OllamaEmbeddingRequest"
        required: true
      responses:
        "200":
          description: Successful Response
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/OllamaEmbeddingResponse"
        "422":
          description: Validation Error
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/HTTPValidationError"
  /api/tokens/count:
    post:
      summary: Count Tokens
      operationId: count_tokens_api_tokens_count_post
      requestBody:
        content:
          application/json:
            schema:
              $ref: "#/components/schemas/TokenCountRequest"
        required: true
      responses:
        "200":
          description: Successful Response
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/TokenCountResponse"
        "422":
          description: Validation Error
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/HTTPValidationError"
  /api/metrics:
    get:
      summary: Api Metrics
      operationId: api_metrics_api_metrics_get
      responses:
        "200":
          description: Successful Response
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/MetricsSnapshot"
  /api/embeddings/sparse:
    post:
      summary: Sparse Embeddings
      operationId: sparse_embeddings_api_embeddings_sparse_post
      requestBody:
        content:
          application/json:
            schema:
              $ref: "#/components/schemas/SparseEmbeddingRequest"
        required: true
      responses:
        "200":
          description: Successful Response
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/SparseEmbeddingResponse"
        "422":
          description: Validation Error
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/HTTPValidationError"
  /api/embeddings/colbert:
    post:
      summary: Colbert Embeddings
      operationId: colbert_embeddings_api_embeddings_colbert_post
      requestBody:
        content:
          application/json:
            schema:
              $ref: "#/components/schemas/ColBERTEmbeddingRequest"
        required: true
      responses:
        "200":
          description: Successful Response
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ColBERTEmbeddingResponse"
        "422":
          description: Validation Error
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/HTTPValidationError"
  /api/rerank:
    post:
      summary: Rerank Documents
      operationId: rerank_api_rerank_post
      requestBody:
        content:
          application/json:
            schema:
              $ref: "#/components/schemas/RerankRequest"
        required: true
      responses:
        "200":
          description: Successful Response
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/RerankResponse"
        "422":
          description: Validation Error
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/HTTPValidationError"
  /health:
    get:
      summary: Health Check
      operationId: health_check_health_get
      responses:
        "200":
          description: Successful Response
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/HealthResponse"
components:
  schemas:
    ColBERTEmbeddingData:
      properties:
        index:
          type: integer
          title: Index
        embeddings:
          items:
            items:
              type: number
            type: array
          type: array
          title: Embeddings
      type: object
      required:
        - index
        - embeddings
      title: ColBERTEmbeddingData
    ColBERTEmbeddingRequest:
      properties:
        model:
          type: string
          title: Model
        input:
          anyOf:
            - type: string
            - items:
                type: string
              type: array
          title: Input
      type: object
      required:
        - model
        - input
      title: ColBERTEmbeddingRequest
    ColBERTEmbeddingResponse:
      properties:
        model:
          type: string
          title: Model
        data:
          items:
            $ref: "#/components/schemas/ColBERTEmbeddingData"
          type: array
          title: Data
        tokens:
          anyOf:
            - items:
                items:
                  type: string
                type: array
              type: array
            - type: "null"
          title: Tokens
      type: object
      required:
        - model
        - data
      title: ColBERTEmbeddingResponse
    RerankRequest:
      properties:
        model:
          type: string
          title: Model
        query:
          type: string
          title: Query
        documents:
          items:
            type: string
          type: array
          title: Documents
        top_n:
          anyOf:
            - type: integer
            - type: "null"
          title: Top N
        return_documents:
          anyOf:
            - type: boolean
            - type: "null"
          title: Return Documents
      type: object
      required:
        - model
        - query
        - documents
      title: RerankRequest
    RerankResult:
      properties:
        index:
          type: integer
          title: Index
        document:
          type: string
          title: Document
        score:
          type: number
          title: Score
      type: object
      required:
        - index
        - score
      title: RerankResult
    RerankResponse:
      properties:
        model:
          type: string
          title: Model
        results:
          items:
            $ref: "#/components/schemas/RerankResult"
          type: array
          title: Results
      type: object
      required:
        - model
        - results
      title: RerankResponse
    HTTPValidationError:
      properties:
        detail:
          items:
            $ref: "#/components/schemas/ValidationError"
          type: array
          title: Detail
      type: object
      title: HTTPValidationError
    HealthResponse:
      properties:
        status:
          type: string
          title: Status
        models:
          items:
            $ref: "#/components/schemas/ModelInfo"
          type: array
          title: Models
      type: object
      required:
        - status
        - models
      title: HealthResponse
    MetricsRecent:
      properties:
        route:
          type: string
          title: Route
        method:
          type: string
          title: Method
        status:
          type: integer
          title: Status
        ok:
          type: boolean
          title: Ok
        duration_ms:
          type: integer
          title: Duration Ms
        timestamp_ms:
          type: integer
          title: Timestamp Ms
      type: object
      required:
        - route
        - method
        - status
        - ok
        - duration_ms
        - timestamp_ms
      title: MetricsRecent
    MetricsRoute:
      properties:
        route:
          type: string
          title: Route
        calls:
          type: integer
          title: Calls
        success:
          type: integer
          title: Success
        failure:
          type: integer
          title: Failure
        average_ms:
          type: number
          title: Average Ms
        last_status:
          type: integer
          title: Last Status
        last_seen_ms:
          type: integer
          title: Last Seen Ms
      type: object
      required:
        - route
        - calls
        - success
        - failure
        - average_ms
        - last_status
        - last_seen_ms
      title: MetricsRoute
    MetricsSnapshot:
      properties:
        totals:
          $ref: "#/components/schemas/MetricsTotals"
        routes:
          items:
            $ref: "#/components/schemas/MetricsRoute"
          type: array
          title: Routes
        recent:
          items:
            $ref: "#/components/schemas/MetricsRecent"
          type: array
          title: Recent
      type: object
      required:
        - totals
        - routes
        - recent
      title: MetricsSnapshot
    MetricsTotals:
      properties:
        calls:
          type: integer
          title: Calls
        success:
          type: integer
          title: Success
        failure:
          type: integer
          title: Failure
        average_ms:
          type: number
          title: Average Ms
        tracked_routes:
          type: integer
          title: Tracked Routes
      type: object
      required:
        - calls
        - success
        - failure
        - average_ms
        - tracked_routes
      title: MetricsTotals
    ModelInfo:
      properties:
        name:
          type: string
          title: Name
        dimensions:
          type: integer
          title: Dimensions
        max_tokens:
          type: integer
          title: Max Tokens
        is_loaded:
          type: boolean
          title: Is Loaded
        supported_types:
          items:
            type: string
          type: array
          title: Supported Types
      type: object
      required:
        - name
        - dimensions
        - max_tokens
        - is_loaded
        - supported_types
      title: ModelInfo
    OllamaEmbeddingRequest:
      properties:
        model:
          type: string
          title: Model
        prompt:
          type: string
          title: Prompt
        options:
          anyOf:
            - additionalProperties: true
              type: object
            - type: "null"
          title: Options
        keep_alive:
          anyOf:
            - type: string
            - type: "null"
          title: Keep Alive
      type: object
      required:
        - model
        - prompt
      title: OllamaEmbeddingRequest
    OllamaEmbeddingResponse:
      properties:
        embedding:
          items:
            type: number
          type: array
          title: Embedding
      type: object
      required:
        - embedding
      title: OllamaEmbeddingResponse
    OpenAIEmbeddingData:
      properties:
        object:
          type: string
          title: Object
          default: embedding
        index:
          type: integer
          title: Index
        embedding:
          items:
            type: number
          type: array
          title: Embedding
      type: object
      required:
        - index
        - embedding
      title: OpenAIEmbeddingData
    OpenAIEmbeddingRequest:
      properties:
        model:
          type: string
          title: Model
        input:
          anyOf:
            - type: string
            - items:
                type: string
              type: array
          title: Input
        user:
          anyOf:
            - type: string
            - type: "null"
          title: User
      type: object
      required:
        - model
        - input
      title: OpenAIEmbeddingRequest
    OpenAIEmbeddingResponse:
      properties:
        object:
          type: string
          title: Object
          default: list
        data:
          items:
            $ref: "#/components/schemas/OpenAIEmbeddingData"
          type: array
          title: Data
        model:
          type: string
          title: Model
        usage:
          $ref: "#/components/schemas/OpenAIUsage"
      type: object
      required:
        - data
        - model
        - usage
      title: OpenAIEmbeddingResponse
    OpenAIListModelResponse:
      properties:
        object:
          type: string
          title: Object
          default: list
        data:
          items:
            $ref: "#/components/schemas/OpenAIModelData"
          type: array
          title: Data
      type: object
      required:
        - data
      title: OpenAIListModelResponse
    OpenAIModelData:
      properties:
        id:
          type: string
          title: Id
        object:
          type: string
          title: Object
          default: model
        created:
          type: integer
          title: Created
          default: 1677610602
        owned_by:
          type: string
          title: Owned By
          default: openai
      type: object
      required:
        - id
      title: OpenAIModelData
    OpenAIUsage:
      properties:
        prompt_tokens:
          type: integer
          title: Prompt Tokens
        total_tokens:
          type: integer
          title: Total Tokens
      type: object
      required:
        - prompt_tokens
        - total_tokens
      title: OpenAIUsage
    SparseEmbeddingData:
      properties:
        index:
          type: integer
          title: Index
        indices:
          items:
            type: integer
          type: array
          title: Indices
        values:
          items:
            type: number
          type: array
          title: Values
      type: object
      required:
        - index
        - indices
        - values
      title: SparseEmbeddingData
    SparseEmbeddingRequest:
      properties:
        model:
          type: string
          title: Model
        input:
          anyOf:
            - type: string
            - items:
                type: string
              type: array
          title: Input
      type: object
      required:
        - model
        - input
      title: SparseEmbeddingRequest
    SparseEmbeddingResponse:
      properties:
        model:
          type: string
          title: Model
        data:
          items:
            $ref: "#/components/schemas/SparseEmbeddingData"
          type: array
          title: Data
      type: object
      required:
        - model
        - data
      title: SparseEmbeddingResponse
    TokenCountRequest:
      properties:
        text:
          type: string
          title: Text
      type: object
      required:
        - text
      title: TokenCountRequest
    TokenCountResponse:
      properties:
        count:
          type: integer
          title: Count
        model:
          type: string
          title: Model
      type: object
      required:
        - count
        - model
      title: TokenCountResponse
    ValidationError:
      properties:
        loc:
          items:
            anyOf:
              - type: string
              - type: integer
          type: array
          title: Location
        msg:
          type: string
          title: Message
        type:
          type: string
          title: Error Type
      type: object
      required:
        - loc
        - msg
        - type
      title: ValidationError
