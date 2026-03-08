# Contributing to NEXUS

> Internal development guide for the NEXUS AI Workstation Builder.

## Quick Start

```bash
# Clone
git clone https://github.com/AiImpDevelopment/nexus-workstation.git
cd nexus-workstation/Nexus

# Install dependencies
pnpm install

# Development mode (hot reload)
cargo tauri dev

# Run Rust tests
cd src-tauri && cargo test

# Build release
cargo tauri build --release
```

## Architecture

### Rust Backend (`src-tauri/src/`)

23 modules organized by domain:

| Module | Purpose | Key Commands |
|--------|---------|-------------|
| `router` | Intelligent model routing (Ollama/OpenRouter) | `route_message`, `route_message_stream` |
| `orchestrator` | Standalone AI brain (MAPE-K, Hebbian trust) | Background tasks, scheduling |
| `chat` | Tauri Channel-based streaming | `chat_stream` |
| `docker` | Docker container management | `list_containers`, `container_action` |
| `github` | GitHub API integration | `get_repos`, `get_issues`, `get_pull_requests` |
| `agents` | Custom AI agent CRUD | `create_agent`, `list_agents` |
| `inference` | HuggingFace Hub + GGUF loading | `cmd_download_model`, `cmd_load_gguf` |
| `ide` | Filesystem + search + command execution | `ide_read_file`, `ide_search_files` |
| `browser` | Internal browser management | `open_internal_browser` |
| `cdp_engine` | Chrome DevTools Protocol (chromiumoxide) | `cdp_navigate`, `cdp_screenshot` |
| `cdp_network` | HTTP waterfall capture | `cdp_network_entries` |
| `cdp_devtools` | Console, performance, cookies | `cdp_console_entries`, `cdp_perf_metrics` |
| `browser_agent` | AI-powered web automation | `browser_agent_run` |
| `browser_import` | Bookmarks/history import | `browser_detect_profiles` |
| `web_scraper` | Built-in HTML scraper | `web_scrape`, `web_extract_metadata` |
| `theme_engine` | ElvUI-inspired UI customization | `theme_save`, `theme_validate_contrast` |
| `widget_registry` | Modular dashboard components | `widget_list`, `widget_categories` |
| `style_engine` | BenikUI sub-component styling | `style_get_widget`, `style_save_graph` |
| `evaluation` | Agent-as-a-Judge quality scoring | `eval_agent_output` |
| `settings` | App configuration persistence | `cmd_get_settings`, `cmd_set_setting` |
| `monitoring_quick` | Lightweight sysfs-based system stats | `cmd_get_quick_stats` |
| `system_agent` | Offline health checks | `system_scan` |
| `neuralswarm` | NeuralSwarm status bridge | `neuralswarm_status` |
| `error` | Structured error handling (NexusError) | 7 categories, panic hook |

### Svelte Frontend (`src/`)

| Route | Page |
|-------|------|
| `/` | Dashboard (redirect to chat) |
| `/chat` | AI Chat with model routing |
| `/docker` | Docker container manager |
| `/github` | GitHub repos, issues, PRs |
| `/n8n` | Automation workflows |
| `/news` | Tech news aggregator |

### Key Stores (`src/lib/stores/`)

| Store | Purpose |
|-------|---------|
| `chat.svelte.ts` | Chat state, message history, streaming |
| `errors.svelte.ts` | Centralized error handling + toast queue |

## Testing

```bash
# All Rust tests
cd src-tauri && cargo test

# Specific module
cargo test --lib browser_agent
cargo test --lib web_scraper
cargo test --lib theme_engine

# Current count: 144 tests
```

## Code Style

- **Rust**: `cargo fmt` + `cargo clippy`
- **TypeScript/Svelte**: Prettier + ESLint
- **Commits**: Conventional Commits (`feat:`, `fix:`, `chore:`)
- **Branching**: `main` (releases), `develop` (active dev)

## Key Design Decisions

1. **Standalone**: No external service dependencies (PostgreSQL, Redis, systemd)
2. **Offline-first**: Local Ollama models preferred, cloud as fallback
3. **MIT/Apache-2.0 only**: All dependencies must be commercially safe
4. **Cross-platform**: Linux (primary), Windows, macOS
5. **SQLite bundled**: WAL mode, rusqlite, no external DB
6. **Structured errors**: NexusError with categories, suggestions, and panic recovery

## Release Process

1. Update version in `Cargo.toml` and `tauri.conf.json`
2. Run full test suite: `cargo test`
3. Build: `cargo tauri build --release`
4. Tag: `git tag v0.5.1`
5. Push: `git push origin main --tags`
6. GitHub Actions builds cross-platform binaries

## License

MIT — see [LICENSE](LICENSE)