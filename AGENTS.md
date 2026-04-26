# EmbeddingServer

- If GitNexus says the index is stale, run `npx gitnexus analyze` before relying on it.
- Before editing any Rust function, class, or method, run `gitnexus_impact` on that symbol and stop if the risk is HIGH or CRITICAL.
- Run `gitnexus_detect_changes()` before every commit.
- Use `gitnexus_query` and `gitnexus_context` for code navigation instead of guessing call paths.

## Repo shape

- Single Rust crate; there is no `package.json` or workspace config.
- `src/main.rs` loads `.env`, initializes tracing, discovers models, and starts the server.
- `src/routes.rs` owns route wiring; `src/registry.rs` owns model discovery, lazy loading, and TTL unloading.
- `src/frontend.rs` serves the embedded `/` page.
- `tests/integration.rs` is the main integration suite.
- `Docs/ApiDocument.md` is the API contract source of truth.

## Commands

- Build: `cargo build --release`
- Run: `cargo run --release`
- Test all: `cargo test`
- Run one integration test: `cargo test --test integration <test_name>`
- Docker: `docker compose up --build`
- On Windows, Cargo uses `rust-lld` from `.cargo/config.toml`; do not change the linker unless you mean to.

## Runtime config

- Startup always loads `.env`.
- Relevant env vars: `MODELS_DIR`, `DEFAULT_MODEL`, `MODEL_TTL_SECONDS`, `BIND_ADDR`, `USE_GPU`, `GPU_AVAILABLE`.
- Docker Compose binds host port `34749` to container port `8000` by default and mounts `./AI_Models` read-only at `/app/AI_Models`.
- Model discovery only accepts subdirectories with `metadata.json`; the file named by `default_model_file` must exist or the model is marked unavailable.
- `USE_GPU=true` is only a preference; the backend falls back to CPU unless `GPU_AVAILABLE=true`.

## Change hygiene

- If you change routes or payloads, update `Docs/ApiDocument.md` with the code change.
- Prefer the executable sources (`Cargo.toml`, `docker-compose.yml`, `.cargo/config.toml`, `src/*`) over README text when they disagree.

<!-- gitnexus:start -->
# GitNexus — Code Intelligence

This project is indexed by GitNexus as **EmbeddingServer** (279 symbols, 575 relationships, 24 execution flows). Use the GitNexus MCP tools to understand code, assess impact, and navigate safely.

> If any GitNexus tool warns the index is stale, run `npx gitnexus analyze` in terminal first.

## Always Do

- **MUST run impact analysis before editing any symbol.** Before modifying a function, class, or method, run `gitnexus_impact({target: "symbolName", direction: "upstream"})` and report the blast radius (direct callers, affected processes, risk level) to the user.
- **MUST run `gitnexus_detect_changes()` before committing** to verify your changes only affect expected symbols and execution flows.
- **MUST warn the user** if impact analysis returns HIGH or CRITICAL risk before proceeding with edits.
- When exploring unfamiliar code, use `gitnexus_query({query: "concept"})` to find execution flows instead of grepping. It returns process-grouped results ranked by relevance.
- When you need full context on a specific symbol — callers, callees, which execution flows it participates in — use `gitnexus_context({name: "symbolName"})`.

## Never Do

- NEVER edit a function, class, or method without first running `gitnexus_impact` on it.
- NEVER ignore HIGH or CRITICAL risk warnings from impact analysis.
- NEVER rename symbols with find-and-replace — use `gitnexus_rename` which understands the call graph.
- NEVER commit changes without running `gitnexus_detect_changes()` to check affected scope.

## Resources

| Resource | Use for |
|----------|---------|
| `gitnexus://repo/EmbeddingServer/context` | Codebase overview, check index freshness |
| `gitnexus://repo/EmbeddingServer/clusters` | All functional areas |
| `gitnexus://repo/EmbeddingServer/processes` | All execution flows |
| `gitnexus://repo/EmbeddingServer/process/{name}` | Step-by-step execution trace |

## CLI

| Task | Read this skill file |
|------|---------------------|
| Understand architecture / "How does X work?" | `.claude/skills/gitnexus/gitnexus-exploring/SKILL.md` |
| Blast radius / "What breaks if I change X?" | `.claude/skills/gitnexus/gitnexus-impact-analysis/SKILL.md` |
| Trace bugs / "Why is X failing?" | `.claude/skills/gitnexus/gitnexus-debugging/SKILL.md` |
| Rename / extract / split / refactor | `.claude/skills/gitnexus/gitnexus-refactoring/SKILL.md` |
| Tools, resources, schema reference | `.claude/skills/gitnexus/gitnexus-guide/SKILL.md` |
| Index, status, clean, wiki CLI commands | `.claude/skills/gitnexus/gitnexus-cli/SKILL.md` |

<!-- gitnexus:end -->
