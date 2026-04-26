use axum::response::Html;

pub async fn index() -> Html<&'static str> {
    Html(INDEX_HTML)
}

const INDEX_HTML: &str = r#"<!doctype html>
<html lang="vi">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <meta name="color-scheme" content="dark" />
  <title>Embedding API Server Studio</title>
  <style>
    :root {
      color-scheme: dark;
      --bg: #000000;
      --panel: #121212;
      --panel-2: #161616;
      --panel-3: #0f0f0f;
      --line: #262626;
      --line-strong: #f0f0f0;
      --text: #f6f6f6;
      --muted: #9b9b9b;
      --muted-2: #707070;
      --accent: #ffffff;
      --accent-soft: #5b4dff;
      --success: #0b8f5f;
      --warning: #445166;
      --danger: #7f3340;
      --shadow: 0 18px 48px rgba(0, 0, 0, 0.45);
      --radius: 14px;
    }

    * { box-sizing: border-box; }

    html, body { min-height: 100%; }

    body {
      margin: 0;
      background: var(--bg);
      color: var(--text);
      font: 14px/1.45 Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    }

    button, input, textarea, select {
      font: inherit;
      color: inherit;
    }

    button {
      border: 1px solid var(--line);
      background: var(--panel-2);
      border-radius: 10px;
      cursor: pointer;
      transition: transform .15s ease, border-color .15s ease, background .15s ease;
    }

    button:hover { transform: translateY(-1px); border-color: #3a3a3a; }

    .app {
      display: grid;
      grid-template-columns: 250px minmax(0, 1fr);
      min-height: 100vh;
    }

    .sidebar {
      padding: 18px 14px;
      border-right: 1px solid var(--line);
      background: linear-gradient(180deg, #0a0a0a 0%, #0e0e0e 100%);
      display: flex;
      flex-direction: column;
      gap: 24px;
    }

    .brand {
      display: flex;
      align-items: center;
      gap: 12px;
      padding: 0 2px;
    }

    .brand-title {
      display: flex;
      align-items: center;
      gap: 10px;
      font-size: 22px;
      font-weight: 800;
      letter-spacing: -0.03em;
    }

    .version {
      padding: 2px 8px;
      border-radius: 999px;
      border: 1px solid var(--line);
      background: #202020;
      color: #eaeaea;
      font-size: 11px;
      font-weight: 700;
    }

    .nav {
      display: grid;
      gap: 10px;
      margin-top: 8px;
    }

    .nav button {
      width: 100%;
      text-align: left;
      padding: 12px 14px;
      background: transparent;
      border-color: transparent;
      color: var(--muted);
      border-radius: 9px;
      font-weight: 500;
    }

    .nav button.active {
      color: var(--text);
      border: 2px solid var(--line-strong);
      background: rgba(255, 255, 255, 0.03);
      font-weight: 700;
    }

    .sidebar-footer {
      margin-top: auto;
      display: grid;
      gap: 10px;
      color: var(--muted);
      font-size: 12px;
    }

    .chip {
      display: inline-flex;
      align-items: center;
      gap: 8px;
      width: fit-content;
      padding: 8px 10px;
      border-radius: 999px;
      border: 1px solid var(--line);
      background: #101010;
      color: #d9d9d9;
      font-size: 12px;
    }

    .content {
      padding: 56px 32px 40px;
      display: flex;
      justify-content: center;
    }

    .content-inner {
      width: min(1120px, 100%);
      display: grid;
      gap: 26px;
      align-content: start;
    }

    .page { display: none; }
    .page.active { display: grid; gap: 22px; }

    .page-head {
      display: grid;
      gap: 10px;
    }

    .page-head h1 {
      margin: 0;
      font-size: 22px;
      line-height: 1.15;
      letter-spacing: -0.03em;
    }

    .page-head p {
      margin: 0;
      color: var(--muted);
      font-size: 14px;
    }

    .card {
      border: 1px solid var(--line);
      border-radius: 10px;
      background: linear-gradient(180deg, var(--panel) 0%, #151515 100%);
      box-shadow: var(--shadow);
    }

    .tester {
      width: min(790px, 100%);
      padding: 28px 24px 24px;
      display: grid;
      gap: 18px;
    }

    .field-grid {
      display: grid;
      gap: 16px;
    }

    .field-grid.hidden,
    .hidden {
      display: none;
    }

    .inline-row {
      display: grid;
      gap: 10px;
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }

    .helper {
      color: var(--muted-2);
      font-size: 12px;
      line-height: 1.4;
    }

    .tab-row {
      display: inline-flex;
      gap: 8px;
      flex-wrap: wrap;
    }

    .tab-row button {
      padding: 8px 12px;
      color: var(--muted);
      background: #141414;
    }

    .tab-row button.active {
      color: var(--text);
      border-color: var(--line-strong);
      background: #1d1d1d;
    }

    .field { display: grid; gap: 8px; }

    .field label, .label {
      color: var(--muted);
      font-size: 13px;
    }

    input, textarea, select {
      width: 100%;
      border: 1px solid var(--line);
      background: #171717;
      border-radius: 9px;
      padding: 12px 14px;
      outline: none;
    }

    textarea { min-height: 120px; resize: vertical; }

    input:focus, textarea:focus, select:focus {
      border-color: #4f4f4f;
      box-shadow: 0 0 0 2px rgba(255,255,255,.05);
    }

    .actions {
      display: flex;
      gap: 12px;
      flex-wrap: wrap;
      align-items: center;
    }

    .primary {
      width: auto;
      padding: 12px 18px;
      border-color: #f0f0f0;
      background: #f7f7f7;
      color: #111;
      font-weight: 700;
    }

    .primary:hover { background: #fff; }

    .subtle {
      color: var(--muted);
      font-size: 12px;
    }

    .models-wrap {
      display: grid;
      grid-template-columns: repeat(2, minmax(0, 1fr));
      gap: 20px;
      align-items: start;
    }

    .models-head {
      display: grid;
      grid-template-columns: minmax(0, 1fr) auto;
      gap: 18px;
      align-items: end;
    }

    .models-summary {
      display: grid;
      grid-template-columns: repeat(4, minmax(0, 1fr));
      gap: 12px;
    }

    .summary-card {
      min-width: 130px;
      padding: 14px 16px;
      border: 1px solid var(--line);
      border-radius: 14px;
      background: linear-gradient(180deg, #171717 0%, #101010 100%);
      box-shadow: var(--shadow);
      display: grid;
      gap: 6px;
    }

    .summary-card .label {
      color: var(--muted);
      font-size: 12px;
      letter-spacing: 0.02em;
      text-transform: uppercase;
    }

    .summary-card .value {
      font-size: 24px;
      line-height: 1;
      font-weight: 800;
      letter-spacing: -0.04em;
    }

    .summary-card .hint {
      color: var(--muted-2);
      font-size: 12px;
    }

    .models-grid {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
      gap: 18px;
    }

    .model-card {
      padding: 0;
      display: grid;
      gap: 0;
      min-height: 100%;
      overflow: hidden;
      position: relative;
      border-radius: 16px;
      background: linear-gradient(180deg, #171717 0%, #111111 100%);
    }

    .model-card::before {
      content: '';
      position: absolute;
      inset: 0 auto 0 0;
      width: 4px;
      background: linear-gradient(180deg, #fff 0%, #5b4dff 100%);
      opacity: 0.9;
    }

    .model-card:hover {
      transform: translateY(-2px);
      border-color: #444;
    }

    .model-card-body {
      display: grid;
      gap: 18px;
      padding: 18px 18px 16px 22px;
    }

    .model-card-head {
      display: grid;
      gap: 10px;
    }

    .model-card h3 {
      margin: 0;
      font-size: 18px;
      line-height: 1.2;
      letter-spacing: -0.02em;
      word-break: break-word;
    }

    .model-card-id {
      color: var(--muted);
      font-size: 12px;
      font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
      word-break: break-all;
    }

    .model-card-badges {
      display: flex;
      gap: 8px;
      flex-wrap: wrap;
    }

    .meta-row {
      display: grid;
      grid-template-columns: 1fr auto;
      gap: 10px 18px;
      align-items: center;
      color: var(--muted);
    }

    .meta-row .value {
      color: var(--text);
      font-weight: 700;
      text-align: right;
    }

    .model-capabilities {
      display: flex;
      gap: 8px;
      flex-wrap: wrap;
    }

    .capability {
      padding: 5px 10px;
      border-radius: 999px;
      border: 1px solid var(--line);
      background: rgba(255, 255, 255, 0.04);
      color: #e8e8e8;
      font-size: 12px;
      font-weight: 600;
    }

    .model-footer {
      padding: 14px 18px 18px 22px;
      border-top: 1px solid var(--line);
      background: rgba(255, 255, 255, 0.02);
      display: grid;
      gap: 8px;
    }

    .model-footer .subtle {
      color: var(--muted);
    }

    .model-empty {
      grid-column: 1 / -1;
      padding: 28px 22px;
      border: 1px dashed var(--line);
      border-radius: 16px;
      background: rgba(255, 255, 255, 0.02);
      color: var(--muted);
      text-align: center;
    }

    .endpoint-schema {
      padding: 16px 18px;
      display: grid;
      gap: 8px;
      background: linear-gradient(180deg, #171717 0%, #101010 100%);
    }

    .endpoint-form {
      display: grid;
      gap: 18px;
      padding: 18px;
      border: 1px solid var(--line);
      border-radius: 16px;
      background: linear-gradient(180deg, #171717 0%, #111111 100%);
      box-shadow: var(--shadow);
    }

    .endpoint-form-head {
      display: grid;
      gap: 6px;
    }

    .endpoint-form-head h3 {
      margin: 0;
      font-size: 18px;
      letter-spacing: -0.03em;
    }

    .endpoint-form-body {
      display: grid;
      gap: 16px;
    }

    .endpoint-form-actions {
      display: flex;
      flex-wrap: wrap;
      gap: 10px;
      align-items: center;
    }

    .schema-title {
      font-size: 18px;
      font-weight: 800;
      letter-spacing: -0.03em;
    }

    .schema-fields {
      display: flex;
      gap: 8px;
      flex-wrap: wrap;
    }

    .schema-pill {
      padding: 5px 10px;
      border-radius: 999px;
      border: 1px solid var(--line);
      background: rgba(255, 255, 255, 0.04);
      color: #e8e8e8;
      font-size: 12px;
      font-weight: 600;
    }

    .tag {
      width: fit-content;
      padding: 4px 10px;
      border-radius: 999px;
      font-size: 12px;
      font-weight: 500;
      color: #d6d6d6;
      background: #3a465a;
    }

    .tag.loaded { background: var(--success); }
    .tag.idle { background: var(--warning); }
    .tag.error { background: var(--danger); }

    .model-actions {
      display: flex;
      gap: 10px;
      flex-wrap: wrap;
    }

    .model-actions button {
      width: auto;
      padding: 10px 14px;
    }

    .docs {
      display: grid;
      grid-template-columns: repeat(2, minmax(0, 1fr));
      gap: 18px;
      width: min(820px, 100%);
    }

    .doc-card {
      padding: 18px;
      display: grid;
      gap: 12px;
    }

    .doc-card code {
      display: inline-flex;
      width: fit-content;
      padding: 5px 8px;
      border-radius: 6px;
      background: #1d1d1d;
      border: 1px solid var(--line);
      color: #f0f0f0;
      font-size: 12px;
    }

    .response {
      width: min(790px, 100%);
      padding: 18px;
      display: grid;
      gap: 12px;
    }

    .response-meta {
      display: flex;
      gap: 12px;
      flex-wrap: wrap;
      color: var(--muted);
      font-size: 12px;
    }

    .response-meta span {
      padding: 4px 8px;
      border: 1px solid var(--line);
      border-radius: 999px;
      background: #121212;
    }

    pre {
      margin: 0;
      min-height: 220px;
      max-height: 420px;
      overflow: auto;
      padding: 16px;
      border-radius: 10px;
      border: 1px solid var(--line);
      background: #090909;
      white-space: pre-wrap;
      word-break: break-word;
    }

    .status-line {
      display: flex;
      align-items: center;
      gap: 10px;
      color: var(--muted);
      font-size: 12px;
    }

    .dot {
      width: 9px;
      height: 9px;
      border-radius: 999px;
      background: #7b7b7b;
      box-shadow: 0 0 0 3px rgba(255,255,255,.04);
      flex: none;
    }

    .dot.ok { background: #4ad18f; }
    .dot.err { background: #ff7f8f; }

    .empty {
      color: var(--muted);
      text-align: center;
      padding: 28px 18px;
    }

    @media (max-width: 1100px) {
      .models-wrap, .models-grid, .docs { grid-template-columns: 1fr; }
      .models-summary { grid-template-columns: repeat(2, minmax(0, 1fr)); }
      .content { padding-inline: 20px; }
    }

    @media (max-width: 860px) {
      .app { grid-template-columns: 1fr; }
      .sidebar { border-right: 0; border-bottom: 1px solid var(--line); }
      .content { padding: 28px 16px 28px; }
      .tester, .response, .page-head { width: 100%; }
      .models-head { grid-template-columns: 1fr; }
      .models-summary { grid-template-columns: 1fr; }
    }
  </style>
</head>
<body>
  <div class="app">
    <aside class="sidebar">
      <div class="brand">
        <div class="brand-title">PyEmbedding <span class="version">v1.0.0</span></div>
      </div>

      <nav class="nav" aria-label="Primary">
        <button class="active" data-view="embeddings">Embeddings</button>
        <button data-view="models">Models</button>
        <button data-view="docs">API Docs</button>
      </nav>

      <div class="sidebar-footer">
        <div class="chip"><span class="dot" id="healthDot"></span><span id="healthText">Health: checking...</span></div>
        <div class="chip">Port <strong>34749</strong></div>
        <div class="chip">Models <strong id="sidebarModelCount">0</strong></div>
      </div>
    </aside>

    <main class="content">
      <div class="content-inner">
        <section class="page active" id="page-embeddings">
          <div class="page-head">
            <h1>API Tester</h1>
            <p>Thử nghiệm các endpoint embedding trực tiếp từ trình duyệt.</p>
          </div>

          <div class="card tester">
            <div class="field">
              <label for="endpointSelect">Endpoint</label>
              <select id="endpointSelect"></select>
              <div class="helper" id="endpointHelp"></div>
            </div>

            <div class="tab-row" role="tablist" aria-label="Request mode">
              <button type="button" class="active" data-mode="form">Form mode</button>
              <button type="button" data-mode="json">Raw JSON</button>
            </div>

            <div class="endpoint-schema card" id="endpointSchema">
              <div class="subtle" id="endpointSchemaLabel">Schema</div>
              <div class="schema-title" id="endpointSchemaTitle">-</div>
              <div class="schema-fields" id="endpointSchemaFields"></div>
            </div>

            <div class="field-grid" id="formMode"></div>

            <div class="field-grid hidden" id="jsonMode">
              <div class="field">
                <label for="rawJsonInput">Raw JSON</label>
                <textarea id="rawJsonInput" placeholder="{\n  \"model\": \"...\"\n}"></textarea>
              </div>
            </div>

            <div class="actions">
              <button class="primary" id="sendRequest">Gửi request</button>
              <button id="refreshModels">Refresh models</button>
              <button id="checkHealth">Health</button>
            </div>

            <div class="subtle" id="requestMeta">Sẵn sàng</div>
          </div>

          <div class="card response">
            <div class="status-line"><span class="dot" id="responseDot"></span><span id="responseMode">JSON response</span></div>
            <div class="response-meta">
              <span id="responseEndpoint">Endpoint: -</span>
              <span id="responseStatus">Status: -</span>
              <span id="responseTime">Time: -</span>
            </div>
            <pre id="output">Chưa có dữ liệu phản hồi.</pre>
            <div class="actions">
              <button id="copyResponse">Copy</button>
              <button id="clearResponse">Clear</button>
            </div>
          </div>
        </section>

        <section class="page" id="page-models">
          <div class="page-head models-head">
            <div>
              <h1>Models List</h1>
              <p>Danh sách model được nạp từ registry, kèm trạng thái và khả năng hỗ trợ.</p>
            </div>
            <div class="models-summary">
              <div class="summary-card">
                <span class="label">Tổng model</span>
                <strong class="value" id="modelsTotal">0</strong>
                <span class="hint">Trong thư mục `AI_Models/`</span>
              </div>
              <div class="summary-card">
                <span class="label">Đang tải</span>
                <strong class="value" id="modelsLoaded">0</strong>
                <span class="hint">Sẵn sàng nhận request</span>
              </div>
              <div class="summary-card">
                <span class="label">Capabilities</span>
                <strong class="value" id="modelsCapabilities">0</strong>
                <span class="hint">Tổng loại tính năng</span>
              </div>
              <div class="summary-card">
                <span class="label">Dim lớn nhất</span>
                <strong class="value" id="modelsMaxDim">-</strong>
                <span class="hint">Kích thước embedding cao nhất</span>
              </div>
            </div>
          </div>

          <div class="models-grid" id="modelGrid"></div>
        </section>

        <section class="page" id="page-docs">
          <div class="page-head">
            <h1>API Docs</h1>
            <p>Tổng hợp nhanh các endpoint đang nhúng trong server.</p>
          </div>

          <div class="docs">
            <div class="card doc-card">
              <h3>/v1/embeddings</h3>
              <code>POST</code>
              <div class="subtle">OpenAI-compatible embeddings.</div>
            </div>
            <div class="card doc-card">
              <h3>/api/embeddings</h3>
              <code>POST</code>
              <div class="subtle">Ollama-style embedding payload.</div>
            </div>
            <div class="card doc-card">
              <h3>/api/rerank</h3>
              <code>POST</code>
              <div class="subtle">Rerank danh sách documents.</div>
            </div>
            <div class="card doc-card">
              <h3>/api/embeddings/sparse</h3>
              <code>POST</code>
              <div class="subtle">Sparse embeddings cho text ngắn.</div>
            </div>
            <div class="card doc-card">
              <h3>/api/embeddings/colbert</h3>
              <code>POST</code>
              <div class="subtle">ColBERT embeddings theo token.</div>
            </div>
            <div class="card doc-card">
              <h3>/v1/models</h3>
              <code>GET</code>
              <div class="subtle">Đọc metadata từ AI_Models/*/metadata.json.</div>
            </div>
            <div class="card doc-card">
              <h3>/api/tokens/count</h3>
              <code>POST</code>
              <div class="subtle">Đếm token cho một đoạn text.</div>
            </div>
          </div>
        </section>
      </div>
    </main>
  </div>

  <script>
    const els = {
      endpointSelect: document.getElementById('endpointSelect'),
      endpointHelp: document.getElementById('endpointHelp'),
      rawJsonInput: document.getElementById('rawJsonInput'),
      formMode: document.getElementById('formMode'),
      jsonMode: document.getElementById('jsonMode'),
      sendRequest: document.getElementById('sendRequest'),
      refreshModels: document.getElementById('refreshModels'),
      checkHealth: document.getElementById('checkHealth'),
      requestMeta: document.getElementById('requestMeta'),
      output: document.getElementById('output'),
      responseMode: document.getElementById('responseMode'),
      responseDot: document.getElementById('responseDot'),
      responseEndpoint: document.getElementById('responseEndpoint'),
      responseStatus: document.getElementById('responseStatus'),
      responseTime: document.getElementById('responseTime'),
      copyResponse: document.getElementById('copyResponse'),
      clearResponse: document.getElementById('clearResponse'),
      modelGrid: document.getElementById('modelGrid'),
      modelsTotal: document.getElementById('modelsTotal'),
      modelsLoaded: document.getElementById('modelsLoaded'),
      modelsCapabilities: document.getElementById('modelsCapabilities'),
      modelsMaxDim: document.getElementById('modelsMaxDim'),
      endpointSchemaLabel: document.getElementById('endpointSchemaLabel'),
      endpointSchemaTitle: document.getElementById('endpointSchemaTitle'),
      endpointSchemaFields: document.getElementById('endpointSchemaFields'),
      healthDot: document.getElementById('healthDot'),
      healthText: document.getElementById('healthText'),
      sidebarModelCount: document.getElementById('sidebarModelCount'),
    };

    const endpoints = [
      {
        id: 'openai_embeddings',
        label: '/v1/embeddings',
        method: 'POST',
        path: '/v1/embeddings',
        help: 'OpenAI-compatible embeddings. Body: model, input.',
        fields: ['model', 'input'],
        jsonTemplate: () => ({ model: '', input: '' }),
        defaultModel: true,
      },
      {
        id: 'ollama_embeddings',
        label: '/api/embeddings',
        method: 'POST',
        path: '/api/embeddings',
        help: 'Ollama-style embedding payload. Body: model, prompt, options, keep_alive.',
        fields: ['model', 'prompt', 'options', 'keep_alive'],
        jsonTemplate: () => ({ model: '', prompt: '', options: {}, keep_alive: '' }),
        defaultModel: true,
      },
      {
        id: 'sparse_embeddings',
        label: '/api/embeddings/sparse',
        method: 'POST',
        path: '/api/embeddings/sparse',
        help: 'Sparse embeddings. Body: model, input.',
        fields: ['model', 'input'],
        jsonTemplate: () => ({ model: '', input: '' }),
        defaultModel: true,
      },
      {
        id: 'colbert_embeddings',
        label: '/api/embeddings/colbert',
        method: 'POST',
        path: '/api/embeddings/colbert',
        help: 'ColBERT embeddings. Body: model, input.',
        fields: ['model', 'input'],
        jsonTemplate: () => ({ model: '', input: '' }),
        defaultModel: true,
      },
      {
        id: 'rerank',
        label: '/api/rerank',
        method: 'POST',
        path: '/api/rerank',
        help: 'Rerank documents. Body: model, query, documents, top_n, return_documents.',
        fields: ['model', 'query', 'documents', 'rerank_options'],
        jsonTemplate: () => ({
          model: '',
          query: '',
          documents: ['Document 1', 'Document 2'],
          top_n: 2,
          return_documents: true,
        }),
        defaultModel: true,
      },
      {
        id: 'token_count',
        label: '/api/tokens/count',
        method: 'POST',
        path: '/api/tokens/count',
        help: 'Count tokens. Body: text.',
        fields: ['text'],
        jsonTemplate: () => ({ text: '' }),
        defaultModel: false,
      },
    ];

    const state = {
      models: [],
      activeView: 'embeddings',
      endpointId: 'openai_embeddings',
      requestMode: 'form',
    };

    function escapeHtml(value) {
      return String(value)
        .replaceAll('&', '&amp;')
        .replaceAll('<', '&lt;')
        .replaceAll('>', '&gt;')
        .replaceAll('"', '&quot;')
        .replaceAll("'", '&#39;');
    }

    function prettyValue(value, fallback = '-') {
      if (value === null || value === undefined || value === '') return fallback;
      return String(value);
    }

    function listCapabilities(model) {
      const values = Array.isArray(model.supported_types || model.capabilities)
        ? (model.supported_types || model.capabilities)
        : [];
      const filtered = values.map((value) => String(value).trim()).filter(Boolean);
      return filtered.length ? filtered : ['dense'];
    }

    function renderModelOptions(selectedValue = '') {
      if (!state.models.length) {
        return '<option value="">Chưa có model</option>';
      }
      return state.models
        .map((model) => {
          const id = modelId(model);
          const selected = id === selectedValue ? ' selected' : '';
          return `<option value="${escapeHtml(id)}"${selected}>${escapeHtml(modelName(model))}</option>`;
        })
        .join('');
    }

    function formShell(title, body, subtitle) {
      return `
        <div class="endpoint-form">
          <div class="endpoint-form-head">
            <h3>${escapeHtml(title)}</h3>
            <div class="subtle">${subtitle}</div>
          </div>
          <div class="endpoint-form-body">
            ${body}
          </div>
        </div>`;
    }

    function modelSelectField(selectedValue) {
      return `
        <div class="field">
          <label for="modelSelect">Chọn Model</label>
          <select id="modelSelect">${renderModelOptions(selectedValue)}</select>
        </div>`;
    }

    function renderEmbeddingForm(title) {
      const selectedValue = selectedModel() || (state.models[0] ? modelId(state.models[0]) : '');
      els.formMode.innerHTML = formShell(
        title,
        `${modelSelectField(selectedValue)}
         <div class="field">
           <label for="inputText">Văn bản cần Embed</label>
           <textarea id="inputText" placeholder="Nhập nội dung cần xử lý..."></textarea>
         </div>`,
        'Body: <code>model</code>, <code>input</code>'
      );
    }

    function renderOllamaForm() {
      const selectedValue = selectedModel() || (state.models[0] ? modelId(state.models[0]) : '');
      els.formMode.innerHTML = formShell(
        'Ollama Embeddings',
        `${modelSelectField(selectedValue)}
         <div class="field">
           <label for="promptInput">Prompt</label>
           <textarea id="promptInput" placeholder="Nhập prompt..."></textarea>
         </div>
         <div class="inline-row">
           <div class="field">
             <label for="keepAliveInput">keep_alive</label>
             <input id="keepAliveInput" type="text" placeholder="5m, 1h, ..." />
           </div>
           <div class="field">
             <label for="optionsInput">options JSON</label>
             <input id="optionsInput" type="text" placeholder='{"temperature":0.1}' />
           </div>
         </div>`,
        'Body: <code>model</code>, <code>prompt</code>, <code>options</code>, <code>keep_alive</code>'
      );
    }

    function renderRerankForm() {
      const selectedValue = selectedModel() || (state.models[0] ? modelId(state.models[0]) : '');
      els.formMode.innerHTML = formShell(
        'Rerank',
        `${modelSelectField(selectedValue)}
         <div class="field">
           <label for="queryInput">Query</label>
           <input id="queryInput" type="text" placeholder="Câu truy vấn..." />
         </div>
         <div class="field">
           <label for="documentsInput">Documents</label>
           <textarea id="documentsInput" placeholder="Mỗi dòng là một document"></textarea>
         </div>
         <div class="inline-row">
           <div class="field">
             <label for="topNInput">top_n</label>
             <input id="topNInput" type="number" min="1" placeholder="Tối đa kết quả" />
           </div>
           <div class="field">
             <label for="returnDocumentsInput">return_documents</label>
             <select id="returnDocumentsInput">
               <option value="true">true</option>
               <option value="false">false</option>
             </select>
           </div>
         </div>`,
        'Body: <code>model</code>, <code>query</code>, <code>documents</code>, <code>top_n</code>, <code>return_documents</code>'
      );
    }

    function renderTokenCountForm() {
      els.formMode.innerHTML = formShell(
        'Token Count',
        `<div class="field">
           <label for="textInput">Text</label>
           <textarea id="textInput" placeholder="Nội dung cần đếm token"></textarea>
         </div>`,
        'Body: <code>text</code>'
      );
    }

    function renderEndpointForm(endpoint) {
      if (endpoint.id === 'ollama_embeddings') return renderOllamaForm();
      if (endpoint.id === 'rerank') return renderRerankForm();
      if (endpoint.id === 'token_count') return renderTokenCountForm();
      if (endpoint.id === 'sparse_embeddings') return renderEmbeddingForm('Sparse Embeddings');
      if (endpoint.id === 'colbert_embeddings') return renderEmbeddingForm('ColBERT Embeddings');
      return renderEmbeddingForm('OpenAI Embeddings');
    }

    function buildEndpointJson(endpoint) {
      const template = endpoint.jsonTemplate ? endpoint.jsonTemplate() : {};
      if (endpoint.defaultModel) {
        template.model = selectedModel() || '';
      }
      return JSON.stringify(template, null, 2);
    }

    function endpointSchemaLabel(endpoint) {
      if (endpoint.id === 'ollama_embeddings') return 'Ollama schema';
      if (endpoint.id === 'rerank') return 'Rerank schema';
      if (endpoint.id === 'token_count') return 'Token count schema';
      return 'Embedding schema';
    }

    function renderEndpointSchema(endpoint) {
      const fields = endpoint.fields || [];
      els.endpointSchemaLabel.textContent = endpointSchemaLabel(endpoint);
      els.endpointSchemaTitle.textContent = `${endpoint.method} ${endpoint.label}`;
      els.endpointSchemaFields.innerHTML = fields
        .map((field) => `<span class="schema-pill">${escapeHtml(field)}</span>`)
        .join('');
    }

    function syncRawJsonTemplate() {
      const endpoint = selectedEndpoint();
      els.rawJsonInput.value = buildEndpointJson(endpoint);
    }

    function seedFormValues() {
      const endpoint = selectedEndpoint();
      const form = els.formMode;
      const setValue = (selector, value) => {
        const node = form.querySelector(selector);
        if (node && !node.value) {
          node.value = value;
        }
      };

      if (endpoint.defaultModel) {
        syncModelSelection();
      }

      if (endpoint.id === 'openai_embeddings' || endpoint.id === 'sparse_embeddings' || endpoint.id === 'colbert_embeddings') {
        setValue('#inputText', 'Xin chào, tôi cần test embedding.');
      } else if (endpoint.id === 'ollama_embeddings') {
        setValue('#promptInput', 'Xin chào, tôi cần test embedding.');
        setValue('#keepAliveInput', '');
        setValue('#optionsInput', '{"temperature":0.1}');
      } else if (endpoint.id === 'rerank') {
        setValue('#queryInput', 'embedding');
        setValue('#documentsInput', 'Document 1\nDocument 2');
        setValue('#topNInput', '2');
        const returnDocuments = form.querySelector('#returnDocumentsInput');
        if (returnDocuments && !returnDocuments.value) {
          returnDocuments.value = 'true';
        }
      } else if (endpoint.id === 'token_count') {
        setValue('#textInput', 'Xin chào, tôi cần đếm token.');
      }
    }

    function modelName(model) {
      return model.display_name || model.name || model.id || model.model || 'Unknown model';
    }

    function modelId(model) {
      return model.id || model.name || model.model || modelName(model);
    }

    function modelStatus(model) {
      if (model.load_error) return { label: 'Lỗi tải', cls: 'error' };
      if (model.is_loaded) return { label: 'Đã tải', cls: 'loaded' };
      return { label: 'Chưa tải', cls: 'idle' };
    }

    function setStatus(message, kind = 'neutral') {
      els.healthText.textContent = message;
      els.healthDot.className = 'dot' + (kind === 'ok' ? ' ok' : kind === 'err' ? ' err' : '');
    }

    function showJson(value) {
      els.responseMode.textContent = 'JSON response';
      els.responseDot.className = 'dot ok';
      els.output.textContent = JSON.stringify(value, null, 2);
    }

    function showText(value) {
      els.responseMode.textContent = 'TEXT response';
      els.responseDot.className = 'dot';
      els.output.textContent = value;
    }

    function selectedModel() {
      return els.formMode.querySelector('#modelSelect')?.value || '';
    }

    function selectedEndpoint() {
      return endpoints.find((endpoint) => endpoint.id === state.endpointId) || endpoints[0];
    }

    function setHidden(node, hidden) {
      node.classList.toggle('hidden', hidden);
    }

    function parseJsonInput(value) {
      const text = String(value || '').trim();
      if (!text) return null;
      return JSON.parse(text);
    }

    function renderEndpointPicker() {
      els.endpointSelect.innerHTML = '';
      for (const endpoint of endpoints) {
        const option = document.createElement('option');
        option.value = endpoint.id;
        option.textContent = `${endpoint.method} ${endpoint.label}`;
        els.endpointSelect.appendChild(option);
      }
      els.endpointSelect.value = state.endpointId;
      updateEndpointView();
    }

    function updateEndpointView() {
      const endpoint = selectedEndpoint();
      renderEndpointForm(endpoint);
      renderEndpointSchema(endpoint);
      els.endpointHelp.textContent = endpoint.help;
      setHidden(els.formMode, state.requestMode !== 'form');
      setHidden(els.jsonMode, state.requestMode !== 'json');
      seedFormValues();
      syncRawJsonTemplate();

      document.querySelectorAll('.tab-row button').forEach((button) => {
        button.classList.toggle('active', button.dataset.mode === state.requestMode);
      });
    }

    function syncModelSelection() {
      const modelSelect = els.formMode.querySelector('#modelSelect');
      if (modelSelect && state.models.length && !modelSelect.value) {
        modelSelect.value = modelId(state.models[0]);
      }
    }

    function buildRequestBody() {
      if (state.requestMode === 'json') {
        return parseJsonInput(els.rawJsonInput.value);
      }

      const endpoint = selectedEndpoint();
      const body = {};

      if (endpoint.defaultModel) {
        const model = selectedModel();
        if (!model) {
          throw new Error('Không có model nào để test');
        }
        body.model = model;
      }

      if (endpoint.id === 'openai_embeddings' || endpoint.id === 'sparse_embeddings' || endpoint.id === 'colbert_embeddings') {
        body.input = els.formMode.querySelector('#inputText')?.value || '';
      } else if (endpoint.id === 'ollama_embeddings') {
        body.prompt = els.formMode.querySelector('#promptInput')?.value || '';
        const options = parseJsonInput(els.formMode.querySelector('#optionsInput')?.value || '');
        if (options !== null) body.options = options;
        const keepAlive = els.formMode.querySelector('#keepAliveInput')?.value || '';
        if (keepAlive) body.keep_alive = keepAlive;
      } else if (endpoint.id === 'rerank') {
        body.query = els.formMode.querySelector('#queryInput')?.value || '';
        body.documents = (els.formMode.querySelector('#documentsInput')?.value || '')
          .split(/\r?\n/)
          .map((value) => value.trim())
          .filter(Boolean);
        const topN = Number(els.formMode.querySelector('#topNInput')?.value || '');
        if (Number.isFinite(topN) && topN > 0) body.top_n = topN;
        body.return_documents = (els.formMode.querySelector('#returnDocumentsInput')?.value || 'true') === 'true';
      } else if (endpoint.id === 'token_count') {
        body.text = els.formMode.querySelector('#textInput')?.value || '';
      }

      return body;
    }

    async function callEndpoint() {
      const endpoint = selectedEndpoint();
      const payload = buildRequestBody();
      if (state.requestMode === 'json' && payload === null) {
        throw new Error('Raw JSON không hợp lệ');
      }

      const started = performance.now();
      els.requestMeta.textContent = `Đang gọi ${endpoint.path}`;
      els.responseEndpoint.textContent = `Endpoint: ${endpoint.path}`;
      els.responseStatus.textContent = 'Status: pending';
      els.responseTime.textContent = 'Time: ...';

      const response = await fetch(endpoint.path, {
        method: endpoint.method,
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
      });

      const elapsed = Math.round(performance.now() - started);
      const text = await response.text();
      let data = text;
      try { data = text ? JSON.parse(text) : null; } catch {}

      els.responseStatus.textContent = `Status: ${response.status}`;
      els.responseTime.textContent = `Time: ${elapsed}ms`;

      if (!response.ok) {
        throw new Error(typeof data === 'string' ? data : JSON.stringify(data));
      }

      return data;
    }

    function renderModels() {
      els.sidebarModelCount.textContent = String(state.models.length);
      const loadedCount = state.models.filter((model) => model.is_loaded).length;
      const capabilitySet = new Set();
      let maxDim = 0;

      for (const model of state.models) {
        for (const capability of listCapabilities(model)) {
          capabilitySet.add(capability);
        }
        const dim = Number(model.dimensions ?? model.dimension);
        if (Number.isFinite(dim)) {
          maxDim = Math.max(maxDim, dim);
        }
      }

      els.modelsTotal.textContent = String(state.models.length);
      els.modelsLoaded.textContent = String(loadedCount);
      els.modelsCapabilities.textContent = String(capabilitySet.size);
      els.modelsMaxDim.textContent = maxDim > 0 ? String(maxDim) : '-';

      els.modelGrid.innerHTML = '';

      if (!state.models.length) {
        els.modelGrid.innerHTML = '<div class="model-empty">Chưa có model nào. Hãy thêm `metadata.json` vào từng thư mục model rồi bấm Refresh models.</div>';
        syncSelectedModelCard();
        return;
      }

      for (const model of state.models) {
        const id = modelId(model);
        const status = modelStatus(model);
        const capabilities = listCapabilities(model);
        const dims = prettyValue(model.dimensions ?? model.dimension);
        const maxTokens = prettyValue(model.max_tokens ?? model.maxTokens);
        const card = document.createElement('button');
        card.type = 'button';
        card.className = 'card model-card';
        card.dataset.model = id;
        card.innerHTML = `
          <div class="model-card-body">
            <div class="model-card-head">
              <div>
                <h3>${escapeHtml(modelName(model))}</h3>
                <div class="model-card-id">${escapeHtml(id)}</div>
              </div>
              <div class="model-card-badges">
                <span class="tag ${status.cls}">${escapeHtml(status.label)}</span>
              </div>
            </div>
            <div class="meta-row"><span>Kích thước (Dim)</span><span class="value">${escapeHtml(dims)}</span></div>
            <div class="meta-row"><span>Max Tokens</span><span class="value">${escapeHtml(maxTokens)}</span></div>
          </div>
          <div class="model-footer">
            <div class="subtle">Capabilities</div>
            <div class="model-capabilities">
              ${capabilities.map((item) => `<span class="capability">${escapeHtml(item)}</span>`).join('')}
            </div>
          </div>
        `;
        card.addEventListener('click', () => {
          const modelSelect = els.formMode.querySelector('#modelSelect');
          if (modelSelect) {
            modelSelect.value = id;
          }
          syncSelectedModelCard();
          syncRawJsonTemplate();
          setView('embeddings');
        });
        els.modelGrid.appendChild(card);
      }

      syncSelectedModelCard();
    }

    function syncSelectedModelCard() {
      const selected = selectedModel();
      document.querySelectorAll('.model-card').forEach((item) => {
        item.style.borderColor = item.dataset.model === selected ? '#f0f0f0' : 'var(--line)';
      });
    }

    function setView(view) {
      state.activeView = view;
      document.querySelectorAll('.nav button').forEach((button) => {
        button.classList.toggle('active', button.dataset.view === view);
      });
      document.querySelectorAll('.page').forEach((page) => page.classList.remove('active'));
      document.getElementById(`page-${view}`).classList.add('active');
    }

    async function refreshModels() {
      setStatus('Đang tải models...');
      const response = await fetch('/health');
      const text = await response.text();
      let data = text;
      try { data = text ? JSON.parse(text) : null; } catch {}

      if (!response.ok) {
        throw new Error(typeof data === 'string' ? data : JSON.stringify(data));
      }

      state.models = data.models || data.data || [];
      renderModels();
      setStatus(`Đã tải ${state.models.length} model`, 'ok');
      showJson(data);
      renderEndpointForm(selectedEndpoint());
      seedFormValues();
      syncRawJsonTemplate();
    }

    async function checkHealth() {
      const response = await fetch('/health');
      const data = await response.json();
      setStatus(`Health: ${data.status || 'ok'}`, data.status === 'ok' ? 'ok' : 'neutral');
      showJson(data);
    }

    function handleError(error) {
      const message = error && error.message ? error.message : String(error);
      setStatus(message, 'err');
      els.requestMeta.textContent = 'Có lỗi xảy ra';
      els.responseStatus.textContent = 'Status: error';
      showText(error && error.stack ? error.stack : message);
    }

    document.querySelectorAll('.nav button').forEach((button) => {
      button.addEventListener('click', () => setView(button.dataset.view));
    });

    els.endpointSelect.addEventListener('change', () => {
      state.endpointId = els.endpointSelect.value;
      updateEndpointView();
    });

    document.querySelectorAll('.tab-row button').forEach((button) => {
      button.addEventListener('click', () => {
        state.requestMode = button.dataset.mode;
        updateEndpointView();
      });
    });

    els.formMode.addEventListener('change', (event) => {
      if (event.target && event.target.id === 'modelSelect') {
        syncSelectedModelCard();
        syncRawJsonTemplate();
      }
    });
    els.refreshModels.addEventListener('click', () => refreshModels().catch(handleError));
    els.checkHealth.addEventListener('click', () => checkHealth().catch(handleError));
    els.copyResponse.addEventListener('click', async () => {
      try {
        await navigator.clipboard.writeText(els.output.textContent);
        setStatus('Đã copy response', 'ok');
      } catch (error) {
        handleError(error);
      }
    });
    els.clearResponse.addEventListener('click', () => {
      showText('Chưa có dữ liệu phản hồi.');
      setStatus('Response cleared');
      els.responseEndpoint.textContent = 'Endpoint: -';
      els.responseStatus.textContent = 'Status: -';
      els.responseTime.textContent = 'Time: -';
    });

    els.sendRequest.addEventListener('click', async () => {
      try {
        const result = await callEndpoint();
        showJson(result);
        setStatus('Request thành công', 'ok');
      } catch (error) {
        handleError(error);
      }
    });

    renderEndpointPicker();
    updateEndpointView();
    syncRawJsonTemplate();
    refreshModels()
      .then(checkHealth)
      .catch(handleError);
  </script>
</body>
</html>"#;
