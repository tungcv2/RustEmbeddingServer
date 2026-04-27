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

    .metrics-summary {
      display: grid;
      grid-template-columns: repeat(4, minmax(0, 1fr));
      gap: 12px;
    }

    .metrics-panel {
      padding: 20px;
      display: grid;
      gap: 16px;
    }

    .metrics-panel-head {
      display: flex;
      justify-content: space-between;
      align-items: end;
      gap: 12px;
      flex-wrap: wrap;
    }

    .metrics-panel-head h3 {
      margin: 0;
      font-size: 18px;
      letter-spacing: -0.03em;
    }

    .metrics-table-wrap {
      overflow-x: auto;
      border: 1px solid var(--line);
      border-radius: 12px;
      background: #111;
    }

    .metrics-table {
      width: 100%;
      min-width: 780px;
      border-collapse: collapse;
    }

    .metrics-table th,
    .metrics-table td {
      padding: 12px 14px;
      border-bottom: 1px solid var(--line);
      text-align: left;
      vertical-align: top;
    }

    .metrics-table th {
      color: var(--muted);
      font-size: 12px;
      text-transform: uppercase;
      letter-spacing: 0.04em;
    }

    .metrics-table tbody tr:last-child td {
      border-bottom: 0;
    }

    .metric-route {
      font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
    }

    .metric-meta {
      color: var(--muted-2);
      font-size: 12px;
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

    .docs-browser {
      width: min(980px, 100%);
      padding: 20px;
      display: grid;
      gap: 18px;
    }

    .docs-toolbar {
      display: grid;
      grid-template-columns: minmax(0, 1fr) auto;
      gap: 14px;
      align-items: end;
    }

    .docs-toolbar .field {
      gap: 6px;
    }

    .docs-toolbar button {
      align-self: end;
      white-space: nowrap;
    }

    .docs-panel {
      display: grid;
      gap: 18px;
      padding-top: 2px;
    }

    .docs-header {
      display: grid;
      gap: 10px;
    }

    .docs-header-top {
      display: flex;
      gap: 10px;
      flex-wrap: wrap;
      align-items: center;
    }

    .docs-title {
      margin: 0;
      font-size: 20px;
      line-height: 1.2;
      letter-spacing: -0.03em;
    }

    .docs-url {
      width: fit-content;
      padding: 8px 12px;
      border-radius: 10px;
      border: 1px solid var(--line);
      background: #111111;
      color: #f1f1f1;
      font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
      font-size: 12px;
      word-break: break-all;
    }

    .docs-grid {
      display: grid;
      gap: 14px;
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }

    .docs-section {
      padding: 16px;
      border: 1px solid var(--line);
      border-radius: 14px;
      background: rgba(255, 255, 255, 0.02);
      display: grid;
      gap: 12px;
      align-content: start;
    }

    .docs-section h4 {
      margin: 0;
      font-size: 14px;
      letter-spacing: 0.01em;
      text-transform: uppercase;
      color: var(--muted);
    }

    .schema-list {
      display: grid;
      gap: 10px;
    }

    .schema-row {
      padding: 12px 14px;
      border: 1px solid var(--line);
      border-radius: 12px;
      background: #121212;
      display: grid;
      gap: 6px;
    }

    .schema-row-top {
      display: flex;
      gap: 10px;
      flex-wrap: wrap;
      align-items: center;
    }

    .schema-row-name {
      font-weight: 700;
    }

    .schema-row-type {
      padding: 3px 8px;
      border-radius: 999px;
      border: 1px solid var(--line);
      background: rgba(255, 255, 255, 0.04);
      color: #ececec;
      font-size: 12px;
    }

    .schema-row-note {
      color: var(--muted);
      font-size: 12px;
    }

    .docs-example {
      margin: 0;
      min-height: 160px;
      max-height: 300px;
      overflow: auto;
      background: #0b0b0b;
    }

    .docs-full {
      grid-column: 1 / -1;
    }

    .docs-note {
      color: var(--muted-2);
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
      .models-wrap, .models-grid, .docs-grid, .docs-toolbar { grid-template-columns: 1fr; }
      .models-summary { grid-template-columns: repeat(2, minmax(0, 1fr)); }
      .metrics-summary { grid-template-columns: repeat(2, minmax(0, 1fr)); }
      .content { padding-inline: 20px; }
    }

    @media (max-width: 860px) {
      .app { grid-template-columns: 1fr; }
      .sidebar { border-right: 0; border-bottom: 1px solid var(--line); }
      .content { padding: 28px 16px 28px; }
      .tester, .response, .page-head { width: 100%; }
      .models-head { grid-template-columns: 1fr; }
      .models-summary { grid-template-columns: 1fr; }
      .metrics-summary { grid-template-columns: 1fr; }
      .metrics-panel-head { display: grid; }
      .metrics-table { min-width: 640px; }
    }
  </style>
</head>
<body>
  <div class="app">
    <aside class="sidebar">
      <div class="brand">
        <div class="brand-title">Embedding<span class="version">v1.0.0</span></div>
      </div>

      <nav class="nav" aria-label="Primary">
        <button class="active" data-view="embeddings">Embeddings</button>
        <button data-view="models">Models</button>
        <button data-view="metrics">Metrics</button>
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

        <section class="page" id="page-metrics">
          <div class="page-head">
            <h1>API Metrics</h1>
            <p>Theo dõi số lần gọi, latency trung bình và 100 request gần nhất, chỉ lưu kết quả success/fail.</p>
          </div>

          <div class="metrics-summary">
            <div class="summary-card">
              <span class="label">Tổng request</span>
              <strong class="value" id="metricsTotalCalls">0</strong>
              <span class="hint">Tất cả API đang được track</span>
            </div>
            <div class="summary-card">
              <span class="label">Latency TB</span>
              <strong class="value" id="metricsAvgMs">0.0</strong>
              <span class="hint">Milliseconds trung bình</span>
            </div>
            <div class="summary-card">
              <span class="label">Tỉ lệ thành công</span>
              <strong class="value" id="metricsSuccessRate">0%</strong>
              <span class="hint">Success / tổng request</span>
            </div>
            <div class="summary-card">
              <span class="label">100 gần nhất</span>
              <strong class="value" id="metricsRecentCount">0</strong>
              <span class="hint" id="metricsUpdatedAt">Chưa tải dữ liệu</span>
            </div>
          </div>

          <div class="card metrics-panel">
            <div class="metrics-panel-head">
              <div>
                <div class="label">By API</div>
                <h3>Thống kê theo endpoint</h3>
              </div>
              <div class="subtle">Tổng số route: <strong id="metricsRouteCount">0</strong></div>
            </div>
            <div class="metrics-table-wrap">
              <table class="metrics-table">
                <thead>
                  <tr>
                    <th>API</th>
                    <th>Calls</th>
                    <th>Avg ms</th>
                    <th>Success</th>
                    <th>Fail</th>
                    <th>Last</th>
                  </tr>
                </thead>
                <tbody id="metricsRoutesBody"></tbody>
              </table>
            </div>
          </div>

          <div class="card metrics-panel">
            <div class="metrics-panel-head">
              <div>
                <div class="label">Recent 100</div>
                <h3>Kết quả gần nhất</h3>
              </div>
              <div class="subtle">Chỉ lưu status và thời gian xử lý</div>
            </div>
            <div class="metrics-table-wrap">
              <table class="metrics-table">
                <thead>
                  <tr>
                    <th>API</th>
                    <th>Result</th>
                    <th>Duration</th>
                    <th>Time</th>
                  </tr>
                </thead>
                <tbody id="metricsRecentBody"></tbody>
              </table>
            </div>
          </div>
        </section>

        <section class="page" id="page-docs">
          <div class="page-head">
            <h1>API Docs</h1>
            <p>Chọn API từ dropdown để xem hướng dẫn chi tiết và copy toàn bộ nội dung.</p>
          </div>

          <div class="card docs-browser">
            <div class="docs-toolbar">
              <div class="field">
                <label for="docsSelect">API</label>
                <select id="docsSelect"></select>
                <div class="helper">Mỗi mục bao gồm URL, input schema, output schema và ví dụ dùng nhanh.</div>
              </div>
              <button type="button" class="primary" id="copyDocGuide">Copy hướng dẫn</button>
            </div>

            <div class="docs-panel" id="docsPanel">
              <div class="docs-header">
                <div class="docs-header-top">
                  <span class="tag" id="docsMethod">POST</span>
                  <span class="subtle" id="docsSummary">-</span>
                </div>
                <h3 class="docs-title" id="docsTitle">-</h3>
                <div class="docs-url" id="docsUrl">-</div>
                <div class="docs-note">Bấm nút copy để sao chép toàn bộ hướng dẫn đang chọn.</div>
              </div>

              <div class="docs-grid">
                <section class="docs-section">
                  <h4>Input schema</h4>
                  <div class="schema-list" id="docsInputSchema"></div>
                </section>
                <section class="docs-section">
                  <h4>Output schema</h4>
                  <div class="schema-list" id="docsOutputSchema"></div>
                </section>
                <section class="docs-section docs-full">
                  <h4>Ví dụ request</h4>
                  <pre class="docs-example" id="docsRequestExample"></pre>
                </section>
                <section class="docs-section docs-full">
                  <h4>Ví dụ response</h4>
                  <pre class="docs-example" id="docsResponseExample"></pre>
                </section>
              </div>
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
      docsSelect: document.getElementById('docsSelect'),
      copyDocGuide: document.getElementById('copyDocGuide'),
      docsPanel: document.getElementById('docsPanel'),
      docsMethod: document.getElementById('docsMethod'),
      docsSummary: document.getElementById('docsSummary'),
      docsTitle: document.getElementById('docsTitle'),
      docsUrl: document.getElementById('docsUrl'),
      docsInputSchema: document.getElementById('docsInputSchema'),
      docsOutputSchema: document.getElementById('docsOutputSchema'),
      docsRequestExample: document.getElementById('docsRequestExample'),
      docsResponseExample: document.getElementById('docsResponseExample'),
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
      metricsTotalCalls: document.getElementById('metricsTotalCalls'),
      metricsAvgMs: document.getElementById('metricsAvgMs'),
      metricsSuccessRate: document.getElementById('metricsSuccessRate'),
      metricsRecentCount: document.getElementById('metricsRecentCount'),
      metricsUpdatedAt: document.getElementById('metricsUpdatedAt'),
      metricsRouteCount: document.getElementById('metricsRouteCount'),
      metricsRoutesBody: document.getElementById('metricsRoutesBody'),
      metricsRecentBody: document.getElementById('metricsRecentBody'),
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

    const docsEndpoints = [
      {
        id: 'openai_embeddings',
        label: '/v1/embeddings',
        method: 'POST',
        summary: 'OpenAI-compatible embeddings.',
        inputSchema: [
          { name: 'model', type: 'string', required: true, note: 'Model đã nạp từ registry.' },
          { name: 'input', type: 'string | string[]', required: true, note: 'Một câu hoặc nhiều câu.' },
          { name: 'user', type: 'string | null', required: false, note: 'Tùy chọn.' },
        ],
        outputSchema: [
          { name: 'object', type: 'string', required: true, note: 'Giá trị mặc định: list' },
          { name: 'data', type: 'OpenAIEmbeddingData[]', required: true, note: 'Danh sách vector embedding.' },
          { name: 'model', type: 'string', required: true, note: 'Tên model trả về.' },
          { name: 'usage', type: 'OpenAIUsage', required: true, note: 'Thông tin token.' },
        ],
        requestExample: `{
  "model": "text-embedding-3-small",
  "input": "Xin chào"
}`,
        responseExample: `{
  "object": "list",
  "data": [
    {
      "object": "embedding",
      "index": 0,
      "embedding": [0.01, 0.02]
    }
  ],
  "model": "text-embedding-3-small",
  "usage": {
    "prompt_tokens": 2,
    "total_tokens": 2
  }
}`,
      },
      {
        id: 'ollama_embeddings',
        label: '/api/embeddings',
        method: 'POST',
        summary: 'Ollama-style embedding payload.',
        inputSchema: [
          { name: 'model', type: 'string', required: true, note: 'Model cần nạp.' },
          { name: 'prompt', type: 'string', required: true, note: 'Chuỗi đầu vào.' },
          { name: 'options', type: 'object | null', required: false, note: 'Tùy chọn cấu hình.' },
          { name: 'keep_alive', type: 'string | null', required: false, note: 'Giữ model trong bộ nhớ.' },
        ],
        outputSchema: [
          { name: 'embedding', type: 'number[]', required: true, note: 'Vector embedding.' },
        ],
        requestExample: `{
  "model": "my-model",
  "prompt": "Xin chào",
  "options": { "temperature": 0.1 }
}`,
        responseExample: `{
  "embedding": [0.01, 0.02]
}`,
      },
      {
        id: 'sparse_embeddings',
        label: '/api/embeddings/sparse',
        method: 'POST',
        summary: 'Sparse embeddings cho text ngắn.',
        inputSchema: [
          { name: 'model', type: 'string', required: true, note: 'Model sparse embedding.' },
          { name: 'input', type: 'string | string[]', required: true, note: 'Text đầu vào.' },
        ],
        outputSchema: [
          { name: 'model', type: 'string', required: true, note: 'Tên model.' },
          { name: 'data', type: 'SparseEmbeddingData[]', required: true, note: 'Sparse vector theo index/value.' },
        ],
        requestExample: `{
  "model": "sparse-model",
  "input": "Xin chào"
}`,
        responseExample: `{
  "model": "sparse-model",
  "data": [
    {
      "index": 0,
      "indices": [1, 5, 8],
      "values": [0.2, 0.1, 0.05]
    }
  ]
}`,
      },
      {
        id: 'colbert_embeddings',
        label: '/api/embeddings/colbert',
        method: 'POST',
        summary: 'ColBERT embeddings theo token.',
        inputSchema: [
          { name: 'model', type: 'string', required: true, note: 'Model ColBERT.' },
          { name: 'input', type: 'string | string[]', required: true, note: 'Chuỗi hoặc mảng chuỗi.' },
        ],
        outputSchema: [
          { name: 'model', type: 'string', required: true, note: 'Tên model.' },
          { name: 'data', type: 'ColBERTEmbeddingData[]', required: true, note: 'Embedding theo token.' },
          { name: 'tokens', type: 'string[][] | null', required: false, note: 'Token gốc nếu có.' },
        ],
        requestExample: `{
  "model": "colbert-model",
  "input": ["Xin chào", "Embedding test"]
}`,
        responseExample: `{
  "model": "colbert-model",
  "data": [
    {
      "index": 0,
      "embeddings": [[0.1, 0.2]]
    }
  ],
  "tokens": [["Xin", "chào"]]
}`,
      },
      {
        id: 'rerank',
        label: '/api/rerank',
        method: 'POST',
        summary: 'Rerank danh sách documents.',
        inputSchema: [
          { name: 'model', type: 'string', required: true, note: 'Model rerank.' },
          { name: 'query', type: 'string', required: true, note: 'Truy vấn cần xếp hạng.' },
          { name: 'documents', type: 'string[]', required: true, note: 'Danh sách tài liệu.' },
          { name: 'top_n', type: 'integer | null', required: false, note: 'Số kết quả trả về.' },
          { name: 'return_documents', type: 'boolean | null', required: false, note: 'Trả lại nội dung tài liệu.' },
        ],
        outputSchema: [
          { name: 'model', type: 'string', required: true, note: 'Tên model.' },
          { name: 'results', type: 'RerankResult[]', required: true, note: 'Danh sách kết quả đã sắp xếp.' },
        ],
        requestExample: `{
  "model": "rerank-model",
  "query": "embedding",
  "documents": ["Document 1", "Document 2"],
  "top_n": 2,
  "return_documents": true
}`,
        responseExample: `{
  "model": "rerank-model",
  "results": [
    {
      "index": 0,
      "document": "Document 1",
      "score": 0.98
    }
  ]
}`,
      },
      {
        id: 'token_count',
        label: '/api/tokens/count',
        method: 'POST',
        summary: 'Đếm token cho một đoạn text.',
        inputSchema: [
          { name: 'text', type: 'string', required: true, note: 'Nội dung cần đếm token.' },
        ],
        outputSchema: [
          { name: 'count', type: 'integer', required: true, note: 'Số token.' },
          { name: 'model', type: 'string', required: true, note: 'Model được dùng để đếm.' },
        ],
        requestExample: `{
  "text": "Xin chào, tôi cần đếm token."
}`,
        responseExample: `{
  "count": 7,
  "model": "token-model"
}`,
      },
      {
        id: 'openai_models',
        label: '/v1/models',
        method: 'GET',
        summary: 'Liệt kê model theo kiểu OpenAI.',
        inputSchema: [],
        outputSchema: [
          { name: 'object', type: 'string', required: true, note: 'Giá trị mặc định: list' },
          { name: 'data', type: 'OpenAIModelData[]', required: true, note: 'Danh sách model.' },
        ],
        requestExample: `GET /v1/models`,
        responseExample: `{
  "object": "list",
  "data": [
    {
      "id": "text-embedding-3-small",
      "object": "model",
      "created": 1677610602,
      "owned_by": "openai"
    }
  ]
}`,
      },
      {
        id: 'health',
        label: '/health',
        method: 'GET',
        summary: 'Kiểm tra trạng thái server và model registry.',
        inputSchema: [],
        outputSchema: [
          { name: 'status', type: 'string', required: true, note: 'Trạng thái tổng quát.' },
          { name: 'models', type: 'ModelInfo[]', required: true, note: 'Danh sách model hiện có.' },
        ],
        requestExample: `GET /health`,
        responseExample: `{
  "status": "ok",
  "models": []
}`,
      },
      {
        id: 'api_metrics',
        label: '/api/metrics',
        method: 'GET',
        summary: 'Thống kê realtime về số lần gọi, latency trung bình và 100 request gần nhất.',
        inputSchema: [],
        outputSchema: [
          { name: 'totals', type: 'MetricsTotals', required: true, note: 'Tổng request, success, fail và latency trung bình.' },
          { name: 'routes', type: 'MetricsRoute[]', required: true, note: 'Thống kê theo endpoint.' },
          { name: 'recent', type: 'MetricsRecent[]', required: true, note: '100 request gần nhất.' },
        ],
        requestExample: `GET /api/metrics`,
        responseExample: `{
  "totals": {
    "calls": 42,
    "success": 40,
    "failure": 2,
    "average_ms": 12.4,
    "tracked_routes": 6
  },
  "routes": [],
  "recent": []
}`,
      },
    ];

    const state = {
      models: [],
      activeView: 'embeddings',
      endpointId: 'openai_embeddings',
      docsEndpointId: 'openai_embeddings',
      requestMode: 'form',
      metrics: { totals: {}, routes: [], recent: [] },
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

    function endpointCapability(endpoint) {
      if (endpoint.id === 'openai_embeddings' || endpoint.id === 'ollama_embeddings') return 'embedding';
      if (endpoint.id === 'sparse_embeddings') return 'sparse_embedding';
      if (endpoint.id === 'colbert_embeddings') return 'colbert_embedding';
      if (endpoint.id === 'rerank') return 'rerank';
      if (endpoint.id === 'token_count') return 'token_count';
      return null;
    }

    function modelsForEndpoint(endpoint = selectedEndpoint()) {
      const capability = endpointCapability(endpoint);
      if (!capability) return state.models;
      return state.models.filter((model) => listCapabilities(model).includes(capability));
    }

    function selectedModelForEndpoint(endpoint = selectedEndpoint()) {
      const selected = selectedModel();
      const models = modelsForEndpoint(endpoint);
      if (selected && models.some((model) => modelId(model) === selected)) {
        return selected;
      }
      return models[0] ? modelId(models[0]) : '';
    }

    function renderModelOptions(selectedValue = '', endpoint = selectedEndpoint()) {
      const models = modelsForEndpoint(endpoint);
      if (!state.models.length) {
        return '<option value="">Chưa có model</option>';
      }
      if (!models.length) {
        return '<option value="">Không có model phù hợp</option>';
      }
      return models
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

    function modelSelectField(selectedValue, endpoint) {
      return `
        <div class="field">
          <label for="modelSelect">Chọn Model</label>
          <select id="modelSelect">${renderModelOptions(selectedValue, endpoint)}</select>
        </div>`;
    }

    function renderEmbeddingForm(title) {
      const endpoint = selectedEndpoint();
      const selectedValue = selectedModelForEndpoint(endpoint);
      els.formMode.innerHTML = formShell(
        title,
        `${modelSelectField(selectedValue, endpoint)}
         <div class="field">
           <label for="inputText">Văn bản cần Embed</label>
           <textarea id="inputText" placeholder="Nhập nội dung cần xử lý..."></textarea>
         </div>`,
        'Body: <code>model</code>, <code>input</code>'
      );
    }

    function renderOllamaForm() {
      const endpoint = selectedEndpoint();
      const selectedValue = selectedModelForEndpoint(endpoint);
      els.formMode.innerHTML = formShell(
        'Ollama Embeddings',
        `${modelSelectField(selectedValue, endpoint)}
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
      const endpoint = selectedEndpoint();
      const selectedValue = selectedModelForEndpoint(endpoint);
      els.formMode.innerHTML = formShell(
        'Rerank',
        `${modelSelectField(selectedValue, endpoint)}
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
        template.model = selectedModelForEndpoint(endpoint);
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

    function formatMs(value) {
      return `${Number(value || 0).toFixed(1)} ms`;
    }

    function formatTimestamp(ms) {
      if (!ms) return '-';
      return new Date(ms).toLocaleString();
    }

    function renderMetrics() {
      const snapshot = state.metrics || {};
      const totals = snapshot.totals || {};
      const routes = Array.isArray(snapshot.routes) ? snapshot.routes : [];
      const recent = Array.isArray(snapshot.recent) ? snapshot.recent : [];
      const success = Number(totals.success || 0);
      const calls = Number(totals.calls || 0);
      const rate = calls ? Math.round((success / calls) * 1000) / 10 : 0;

      els.metricsTotalCalls.textContent = String(calls);
      els.metricsAvgMs.textContent = formatMs(totals.average_ms || 0);
      els.metricsSuccessRate.textContent = `${rate.toFixed(1)}%`;
      els.metricsRecentCount.textContent = String(recent.length);
      els.metricsRouteCount.textContent = String(routes.length);
      els.metricsUpdatedAt.textContent = `Cập nhật: ${new Date().toLocaleTimeString()}`;

      els.metricsRoutesBody.innerHTML = routes.length
        ? routes.map((item) => `
          <tr>
            <td class="metric-route">${escapeHtml(item.route)}</td>
            <td>${escapeHtml(item.calls)}</td>
            <td>${escapeHtml(formatMs(item.average_ms))}</td>
            <td>${escapeHtml(item.success)}</td>
            <td>${escapeHtml(item.failure)}</td>
            <td><span class="tag ${Number(item.last_status || 0) < 400 ? 'loaded' : 'error'}">${escapeHtml(item.last_status || '-')}</span></td>
          </tr>`).join('')
        : '<tr><td colspan="6" class="subtle">Chưa có dữ liệu.</td></tr>';

      els.metricsRecentBody.innerHTML = recent.length
        ? recent.map((item) => `
          <tr>
            <td class="metric-route">${escapeHtml(item.route)}</td>
            <td><span class="tag ${item.ok ? 'loaded' : 'error'}">${item.ok ? 'success' : 'fail'}</span></td>
            <td>${escapeHtml(formatMs(item.duration_ms))}</td>
            <td class="metric-meta">${escapeHtml(formatTimestamp(item.timestamp_ms))}</td>
          </tr>`).join('')
        : '<tr><td colspan="4" class="subtle">Chưa có request nào.</td></tr>';
    }

    async function loadMetrics() {
      const response = await fetch('/api/metrics');
      const data = await response.json();

      if (!response.ok) {
        throw new Error(JSON.stringify(data));
      }

      state.metrics = data;
      renderMetrics();
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

    function selectedDocsEndpoint() {
      return docsEndpoints.find((endpoint) => endpoint.id === state.docsEndpointId) || docsEndpoints[0];
    }

    function renderSchemaRows(items) {
      if (!items.length) {
        return '<div class="subtle">Không có schema.</div>';
      }

      return items.map((item) => `
        <div class="schema-row">
          <div class="schema-row-top">
            <span class="schema-row-name">${escapeHtml(item.name)}</span>
            <span class="schema-row-type">${escapeHtml(item.type)}</span>
            ${item.required ? '<span class="tag loaded">required</span>' : '<span class="tag idle">optional</span>'}
          </div>
          ${item.note ? `<div class="schema-row-note">${escapeHtml(item.note)}</div>` : ''}
        </div>
      `).join('');
    }

    function renderDocsPicker() {
      els.docsSelect.innerHTML = '';
      for (const endpoint of docsEndpoints) {
        const option = document.createElement('option');
        option.value = endpoint.id;
        option.textContent = `${endpoint.method} ${endpoint.label}`;
        els.docsSelect.appendChild(option);
      }
      els.docsSelect.value = state.docsEndpointId;
      updateDocsView();
    }

    function formatDocsGuide(endpoint) {
      const inputSchema = endpoint.inputSchema.length
        ? endpoint.inputSchema.map((item) => `- ${item.name}: ${item.type}${item.required ? ' (required)' : ''}${item.note ? ` - ${item.note}` : ''}`).join('\n')
        : '- Không có input body.';
      const outputSchema = endpoint.outputSchema.length
        ? endpoint.outputSchema.map((item) => `- ${item.name}: ${item.type}${item.required ? ' (required)' : ''}${item.note ? ` - ${item.note}` : ''}`).join('\n')
        : '- Không có output schema.';

      return [
        `${endpoint.method} ${endpoint.label}`,
        '',
        endpoint.summary,
        '',
        'Input schema:',
        inputSchema,
        '',
        'Output schema:',
        outputSchema,
        '',
        'Ví dụ request:',
        endpoint.requestExample,
        '',
        'Ví dụ response:',
        endpoint.responseExample,
      ].join('\n');
    }

    function updateDocsView() {
      const endpoint = selectedDocsEndpoint();
      els.docsMethod.textContent = endpoint.method;
      els.docsSummary.textContent = endpoint.summary;
      els.docsTitle.textContent = endpoint.label;
      els.docsUrl.textContent = `${endpoint.method} ${endpoint.label}`;
      els.docsInputSchema.innerHTML = renderSchemaRows(endpoint.inputSchema);
      els.docsOutputSchema.innerHTML = renderSchemaRows(endpoint.outputSchema);
      els.docsRequestExample.textContent = endpoint.requestExample;
      els.docsResponseExample.textContent = endpoint.responseExample;
    }

    async function copyDocsGuide() {
      const endpoint = selectedDocsEndpoint();
      await navigator.clipboard.writeText(formatDocsGuide(endpoint));
      setStatus(`Đã copy hướng dẫn ${endpoint.label}`, 'ok');
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
      if (!modelSelect || !state.models.length) {
        return;
      }

      const endpoint = selectedEndpoint();
      const models = modelsForEndpoint(endpoint);
      if (!models.length) {
        modelSelect.value = '';
        return;
      }

      if (modelSelect.value && models.some((model) => modelId(model) === modelSelect.value)) {
        return;
      }

      modelSelect.value = selectedModelForEndpoint(endpoint);
    }

    function buildRequestBody() {
      if (state.requestMode === 'json') {
        return parseJsonInput(els.rawJsonInput.value);
      }

      const endpoint = selectedEndpoint();
      const body = {};

      if (endpoint.defaultModel) {
        const model = selectedModelForEndpoint(endpoint);
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
            syncModelSelection();
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
      if (view === 'metrics') {
        loadMetrics().catch(handleError);
      }
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

    els.docsSelect.addEventListener('change', () => {
      state.docsEndpointId = els.docsSelect.value;
      updateDocsView();
    });

    els.copyDocGuide.addEventListener('click', () => copyDocsGuide().catch(handleError));

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
    renderDocsPicker();
    updateEndpointView();
    syncRawJsonTemplate();
    renderMetrics();
    refreshModels()
      .then(checkHealth)
      .catch(handleError);
  </script>
</body>
</html>"#;
