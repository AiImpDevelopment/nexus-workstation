# Rust + Svelte + Tauri: Production Best Practices Research (2025-2026)

**Date**: 2026-03-08
**Scope**: Best practices for building production-ready commercial desktop applications
**Focus**: Tauri 2.x, Svelte 5, Rust full-stack, Spec-driven development, Desktop UX, Rust frontend frameworks
**License Filter**: MIT / Apache-2.0 (commercial-friendly)

---

## Table of Contents

1. [Tauri 2.x Production Best Practices](#1-tauri-2x-production-best-practices)
2. [Svelte 5 / SvelteKit for Tauri Desktop Apps](#2-svelte-5--sveltekit-for-tauri-desktop-apps)
3. [Rust Full-Stack for Embedded Desktop Apps](#3-rust-full-stack-for-embedded-desktop-apps)
4. [Spec-Driven Development & Context Engineering](#4-spec-driven-development--context-engineering)
5. [Desktop App UX Patterns for AI Workstations](#5-desktop-app-ux-patterns-for-ai-workstations)
6. [Rust Frontend Frameworks: Yew vs Leptos vs Dioxus](#6-rust-frontend-frameworks-yew-vs-leptos-vs-dioxus)
7. [Cross-Platform Build Strategies](#7-cross-platform-build-strategies)
8. [Offline-First AI Architecture with Tauri](#8-offline-first-ai-architecture-with-tauri)
9. [Performance Optimization Playbook](#9-performance-optimization-playbook)
10. [Licensing Summary](#10-licensing-summary)
11. [Recommendations for Nexus](#11-recommendations-for-nexus)

---

## 1. Tauri 2.x Production Best Practices

### 1.1 Architecture Overview

Tauri 2.x (stable since October 2024, current: v2.4.2) provides a Rust backend + web frontend architecture using the OS-native WebView instead of bundling Chromium.

**Key architectural advantages:**
- Binary size: 2.5-3 MB vs Electron's 80-120 MB (97% reduction)
- RAM usage: 30-40 MB idle vs Electron's 100+ MB
- Native WebView: WebView2 (Windows), WebKit (macOS), WebKitGTK (Linux)
- Mobile support: iOS and Android from the same codebase (Tauri 2.0+)

### 1.2 IPC (Inter-Process Communication)

Tauri 2.0 rewrote its IPC layer with critical improvements:

**Two IPC primitives:**
- **Commands**: Request-response pattern via `#[tauri::command]` annotations
- **Events**: Fire-and-forget, one-way messages (emittable from both frontend and backend)

**Raw Payload support (NEW in v2):**
- Bypass JSON serialization for large data transfers
- Direct byte transfer or custom serialization (MessagePack, protobuf, BSON, Avro)
- Significant performance improvement for payloads > a few KB

**Channel type:**
- `tauri::ipc::Channel` + JS `Channel` for streaming data across IPC
- Ideal for LLM token streaming, progress updates, real-time data

```rust
// Tauri command with raw payload
#[tauri::command]
async fn process_data(data: Vec<u8>) -> Result<Vec<u8>, String> {
    // Process raw bytes without JSON overhead
    Ok(processed_data)
}
```

**Best practice**: Use MessagePack for large payloads, JSON for small structured data, and Channels for streaming.

### 1.3 Plugin System

Tauri 2.x provides a mature plugin system:

- Plugins can hook into the Tauri lifecycle
- Write platform-specific code in Swift (iOS/macOS) and Kotlin (Android)
- Official plugins workspace: https://github.com/tauri-apps/plugins-workspace
- Permission-based access control per plugin

**Key official plugins:**
- `tauri-plugin-updater` - Auto-updates with differential downloads
- `tauri-plugin-store` - Persistent key-value storage
- `tauri-plugin-shell` - Shell command execution
- `tauri-plugin-fs` - File system access
- `tauri-plugin-dialog` - Native dialogs
- `tauri-plugin-notification` - OS notifications
- `tauri-plugin-clipboard-manager` - Clipboard access
- `tauri-plugin-http` - HTTP client
- `tauri-plugin-process` - Process management

### 1.4 Security Model

Tauri 2.x implements a **default-deny, capabilities-based security model**:

**Permissions:**
- Explicit privilege descriptions for commands
- Map scopes to commands for fine-grained access control
- Defined in `src-tauri/capabilities/` as JSON or TOML files

**Capabilities:**
- Define which permissions are granted/denied per window/webview
- Multiple windows can share capabilities
- Runtime authority enforces all permissions

**Content Security Policy (CSP):**
- Tauri injects nonce and hash sources at compile time
- Only bundled scripts and styles are loadable
- XSS blast radius contained even if webview is compromised

**Production hardening checklist:**
1. Define minimal capabilities per window
2. Enable CSP with strict settings
3. Use Isolation Pattern for sensitive operations
4. Never expose raw system APIs to frontend
5. Audit all plugin permissions

### 1.5 Auto-Update & Distribution

**Update signing:**
- Tauri's updater requires a cryptographic signature (cannot be disabled)
- Generate keypair: `tauri signer generate`
- Public key in `tauri.conf.json`, private key in CI/CD secrets
- Separate from OS-level code signing

**Code signing (production):**
- macOS: Apple Developer certificate + notarization required
- Windows: EV code signing certificate recommended
- Linux: AppImage signing optional but recommended

**Distribution formats:**
- Windows: MSI, NSIS installer
- macOS: DMG, .app bundle
- Linux: AppImage, .deb, .tar.gz

**Update infrastructure:**
- Self-hosted: Static JSON endpoint
- CrabNebula Cloud: Official Tauri partner for dynamic update server
- GitHub Releases: Built-in support via tauri-action

### 1.6 Testing Strategy

**Unit/Integration testing:**
- Mock runtime without executing native WebView
- Standard Rust `#[test]` and `#[tokio::test]`

**End-to-end testing:**
- WebDriver protocol support (Linux + Windows)
- `tauri-driver` wraps platform WebDriver servers
- WebdriverIO and Selenium examples in official docs
- Playwright + TestDriver SDK for visual testing
- macOS: No native desktop WebDriver (use Playwright instead)

---

## 2. Svelte 5 / SvelteKit for Tauri Desktop Apps

### 2.1 Why Svelte 5 + Tauri

Svelte 5 (released October 2024) introduced Runes, a signal-based reactivity system that is particularly well-suited for desktop applications:

- **Smaller bundle**: Svelte compiles to vanilla JS (no runtime VDOM)
- **Fine-grained updates**: Only affected DOM nodes update
- **35% YoY Tauri adoption increase** after 2.0 release
- **Established boilerplate templates**: Tauri 2 + Svelte 5 + shadcn-svelte with CI/CD

### 2.2 Svelte 5 Runes System

**Core Runes for Desktop Apps:**

| Rune | Purpose | Desktop Use Case |
|------|---------|-----------------|
| `$state` | Declare reactive state | UI state, form data, settings |
| `$derived` | Computed values from state | Filtered lists, computed metrics |
| `$effect` | Side effects on state change | IPC calls, file watchers, notifications |
| `$props` | Component props | Reusable UI components |
| `$bindable` | Two-way bindable props | Form controls, settings panels |

**Best practices for desktop state management:**

```typescript
// stores/app.svelte.ts - Shared state file
export function createAppState() {
    let isOnline = $state(false);
    let models = $state<Model[]>([]);
    let activeModel = $derived(models.find(m => m.active));

    $effect(() => {
        // Sync with Rust backend via IPC
        invoke('check_connectivity').then(status => {
            isOnline = status;
        });
    });

    return {
        get isOnline() { return isOnline; },
        get models() { return models; },
        get activeModel() { return activeModel; },
    };
}
```

**Key patterns:**
- Encapsulate reactive logic in `.svelte.ts` files
- Use `$derived` to combine multiple `$state` sources (local UI + Rust data)
- Use `$effect` for IPC synchronization, not for derived values
- `$derived` is pure (no side effects); `$effect` is for side effects

### 2.3 SvelteKit Configuration for Tauri

Tauri does not support server-based solutions. Use these configurations:

**Option 1: SPA Mode (Recommended for Tauri)**
```javascript
// svelte.config.js
import adapter from '@sveltejs/adapter-static';

export default {
    kit: {
        adapter: adapter({
            fallback: 'index.html' // SPA fallback
        })
    }
};
```

```typescript
// src/routes/+layout.ts
export const ssr = false;    // Disable SSR
export const prerender = false; // SPA mode
```

**Option 2: Static Site Generation (SSG)**
```typescript
// src/routes/+layout.ts
export const prerender = true; // Prerender all pages at build time
export const ssr = false;
```

**Tauri configuration:**
```json
// tauri.conf.json
{
    "build": {
        "frontendDist": "../build"
    }
}
```

**Limitations in Tauri:**
- No `+server` files (no server endpoints)
- No SSR (no server-side rendering)
- No server-only `load` functions
- All data fetching via Tauri IPC commands

### 2.4 Component Libraries for Svelte 5 Desktop Apps

| Library | License | Description | Svelte 5 |
|---------|---------|-------------|----------|
| **shadcn-svelte** | MIT | Copy-paste components, full control | Yes |
| **Bits UI** | MIT | Headless primitives, accessibility-first | Yes |
| **Skeleton** | MIT | Full UI toolkit + Tauri guide | Yes |
| **Flowbite Svelte** | MIT | Tailwind-based component library | Yes |
| **Melt UI** | MIT | Headless builder API | Yes |

**Recommended stack for Tauri desktop:**
- **shadcn-svelte** (built on Bits UI) for accessible components
- **Tailwind CSS v4** for styling
- **Lucide Icons** for consistent iconography

**Starter template:**
- `alysonhower/tauri2-svelte5-shadcn` - Clean template with CI/CD for Windows, Linux, macOS

---

## 3. Rust Full-Stack for Embedded Desktop Apps

### 3.1 Database Options

#### SQLite via Rusqlite

**Status**: rusqlite v0.38.0 (December 2025), SQLite 3.51.1 bundled

| Feature | Details |
|---------|---------|
| License | MIT |
| Embedded | Yes (bundled feature flag) |
| Async | No (synchronous only) |
| WASM size | ~25KB minimum |
| Best for | Desktop apps, CLI tools, offline storage |

**Best practices:**
```toml
# Cargo.toml
[dependencies]
rusqlite = { version = "0.38", features = ["bundled"] }
```

- `bundled` feature compiles SQLite into the binary (no system dependency)
- WAL mode for concurrent reads
- Use `with_capacity` for pre-allocated collections
- Synchronous API is fine for desktop apps (not a bottleneck)

**Rust ORM comparison (2026):**

| ORM | Async | Compile-time safety | Best for |
|-----|-------|-------------------|----------|
| **Diesel** | No | Full (schema macros) | Type-safe queries |
| **SQLx** | Yes | Full (compile-time verification) | Async web servers |
| **SeaORM** | Yes | Partial | ActiveRecord pattern |
| **Rusqlite** | No | Manual | Direct SQLite access |

**Recommendation for Tauri desktop:** Rusqlite for simplicity, or Diesel for complex schemas.

#### SurrealDB Embedded

**Status**: Actively developed, used by Volvo, Walmart

| Feature | Details |
|---------|---------|
| License | Apache-2.0 (BSL for cloud features) |
| Embedded | Yes (in-process, no server) |
| Engines | RocksDB (local), SpeeDB, IndxDB |
| Data models | Documents, graphs, vectors, time series |
| AI features | Built-in ML inference, vector embeddings |
| Lean API | 24 functions on `Surreal<T>` |

**Advantages for AI desktop apps:**
- Handles structured data, graph relationships, and vector embeddings in one DB
- Row-level access controls and end-user authentication
- Evolving schemas without migrations
- Runs efficiently on constrained hardware (laptops, edge devices)

**Disadvantages:**
- RocksDB compilation complexity (especially on Windows)
- Smaller ecosystem than SQLite
- BSL license for some cloud features (embedded is Apache-2.0)

**Architecture pattern (AWESOME-APP blueprint):**
```
Desktop (Tauri + SQLite/SurrealDB) ←→ Cloud (Axum + SQLx + Postgres + K8s)
```

### 3.2 Web Framework Integration

#### Axum (v0.8.8, January 2026)

Axum is now the de facto standard Rust web framework, developed by the Tokio team.

**Integration with Tauri:**
- `tauri-plugin-axum` - Embed Axum routes directly in Tauri backend
- `tauri-axum-htmx` - HTMX-based server-side rendering within Tauri
- Full-stack pattern: Rust + Yew/Svelte + Axum + Tauri + Tailwind

**Use cases in desktop apps:**
- Local API server for plugin communication
- Inter-process communication with sidecars
- WebSocket server for real-time updates

### 3.3 Rust Embedding Libraries

#### FastEmbed-rs

**Status**: v5.12.0, MIT license

| Feature | Details |
|---------|---------|
| Backend | pykeio/ort (ONNX Runtime) |
| Tokenizer | HuggingFace tokenizers |
| Models | BGE, BAAI, Snowflake, Jina, etc. |
| Quantization | Q variants available (e.g., BGESmallENV15Q) |
| Capabilities | Text embeddings, image embeddings, reranking |

**Integration with Tauri desktop:**
```rust
use fastembed::{TextEmbedding, InitOptions, EmbeddingModel};

let model = TextEmbedding::try_new(InitOptions {
    model_name: EmbeddingModel::BGESmallENV15,
    show_download_progress: true,
    ..Default::default()
})?;

let embeddings = model.embed(vec!["Hello world"], None)?;
```

- No external dependencies, perfect for offline-first
- ONNX Runtime handles hardware acceleration (CPU, CUDA, DirectML)
- Local model files supported via `try_new_from_user_defined()`

---

## 4. Spec-Driven Development & Context Engineering

### 4.1 Spec-Driven Development (SDD)

**Definition**: Using well-crafted software requirement specifications as prompts, aided by AI coding agents, to generate executable code.

**The problem it solves**: 25% of YC Winter 2025 cohort shipped 95% AI-generated codebases but drowned in technical debt, security holes, and implementations that compiled but did not solve the right problems.

**Core workflow (4 gated phases):**

| Phase | Input | Output | Human Checkpoint |
|-------|-------|--------|------------------|
| **Specify** | High-level goals | Detailed spec (user journeys, success metrics) | Validate intent capture |
| **Plan** | Spec + stack preferences | Architecture plan, constraints | Verify feasibility |
| **Tasks** | Spec + plan | Small, reviewable work items | Confirm scope |
| **Implement** | Tasks | Focused code changes | Review each change |

**Key principles:**
1. Specifications become living, executable artifacts that evolve with the project
2. Use domain-oriented ubiquitous language, not implementation details
3. Structure scenarios using Given/When/Then format
4. Aim for clarity and conciseness (critical path, not all cases)
5. Semi-structured inputs reduce AI hallucinations
6. Deterministic CI/CD practices to safeguard architectures

### 4.2 GitHub Spec Kit

**Status**: Open-source (MIT), 72.7k stars, 110 releases, supports 22+ AI agent platforms

**Installation:**
```bash
uvx --from git+https://github.com/github/spec-kit.git specify init <PROJECT_NAME>
```

**Three primary commands:**
- `/specify` - Generate full specification
- `/plan` - Create technical implementation plan
- `/tasks` - Break down into actionable work items

**Supported AI tools:**
- GitHub Copilot
- Claude Code
- Amazon Q Developer CLI
- Gemini CLI

**Ideal use cases:**
1. Greenfield projects (ensures AI builds intended solutions)
2. Feature work in existing systems (forces integration clarity)
3. Legacy modernization (captures business logic in modern specs)

### 4.3 Context Engineering

**Definition**: Curating what the model sees so you get better results.

**2026 convergence**: Spec-driven development and context engineering are merging:
- Specification languages with built-in context directives
- Context management systems that parse specs to determine what info to retrieve
- Integrated toolchains where specs trigger automatic context assembly

**CLAUDE.md / AGENTS.md best practices:**

| Aspect | Recommendation |
|--------|---------------|
| Scope | As few instructions as possible, universally applicable |
| Style | Examples beat abstractions - point to real files |
| Linting | Use deterministic tools (eslint, rustfmt), not LLMs |
| Limits | Frontier models follow ~150-200 instructions consistently |
| Structure | Separate instructions from guidance/rules/guardrails |
| Hierarchy | `CLAUDE.md` (root) > `.claude/rules/` > per-directory |

**Two categories of prompts:**
1. **Instructions**: Tell an agent to do something specific
2. **Guidance/rules/guardrails**: General conventions to follow

### 4.4 SDD + Claude Code Workflow

**Recommended approach (from Heeki Park, March 2026):**

```
1. spec-kit specify init project-name
2. /specify → Generate specification
3. Human review & iterate spec
4. /plan → Generate technical plan
5. Human review architecture decisions
6. /tasks → Break into implementable units
7. Claude Code implements task-by-task
8. Human reviews each change
9. CI/CD validates automatically
```

**cc-sdd (alternative):**
- Kiro-style commands for structured requirements-design-tasks workflow
- Supports Claude Code, Codex, Opencode, Cursor, Copilot, Gemini CLI, Windsurf

---

## 5. Desktop App UX Patterns for AI Workstations

### 5.1 Microsoft's Enterprise AI Design Principles (2025-2026)

**Three core attributes**: Fast. Intelligent. Beautiful.

**Design principles:**
1. **Human-Centered Simplicity** - Clean iconography with vibrant gradients suggesting intelligence
2. **Function Through Interoperability** - Fluid transitions between apps without losing context
3. **Unified Agent UX Strategy** - Consistent visual identity, shared patterns for autonomy/oversight/trust

### 5.2 Agentic AI Design Patterns

**Key patterns for AI workstation UX:**

| Pattern | Description | Application |
|---------|-------------|-------------|
| **Context Preservation** | Maintain state across multi-app workflows | Chat history, project context |
| **Progressive Disclosure** | Show complexity gradually | Model settings, advanced options |
| **Trust Indicators** | Visual cues for AI confidence | Certainty scores, source attribution |
| **Human Override** | Clear intervention points | Stop/pause AI actions |
| **Activity Timeline** | Transparent action logging | Agent activity feed |
| **Split Agency** | Clear who leads (human vs AI) | Mode indicators |

### 5.3 Enterprise Dashboard Intelligence

**Recommended patterns for AI workstation dashboards:**
- Natural language queries over structured data
- Narrative AI summaries alongside charts
- Real-time metric updates with anomaly highlighting
- Contextual actions from insights

### 5.4 Desktop-Specific UX Considerations

| Area | Pattern | Implementation |
|------|---------|---------------|
| **Window Management** | Multi-window with context sync | Tauri multi-window API |
| **System Tray** | Persistent background operation | `tauri-plugin-tray` |
| **Keyboard Shortcuts** | Power-user efficiency | Global + context-specific |
| **Drag & Drop** | File/content import | Native DnD events |
| **Notifications** | Non-intrusive alerts | OS-native notifications |
| **Theme** | System-aware dark/light | CSS media queries + store |
| **Offline Indicator** | Clear connectivity status | Status bar component |
| **Progress** | Long-operation feedback | Streaming progress bars |

---

## 6. Rust Frontend Frameworks: Yew vs Leptos vs Dioxus

### 6.1 Overview Comparison (2026)

| Feature | Yew (v0.21) | Leptos (v0.6+) | Dioxus (v0.7) |
|---------|-------------|----------------|---------------|
| **License** | MIT/Apache-2.0 | MIT | MIT/Apache-2.0 |
| **GitHub Stars** | ~30.5k | ~18.5k | ~24k+ |
| **Reactivity** | Virtual DOM (React-like) | Fine-grained signals (SolidJS-like) | Custom fiber-like VDOM |
| **Min WASM Size** | ~110KB | ~25KB | ~45KB |
| **With Hydration** | ~130KB | ~35KB | ~60KB |
| **SSR** | Yes | Yes (streaming) | Yes |
| **Desktop** | Via Tauri | Via Tauri | Native (built-in) |
| **Mobile** | No | No | Yes (experimental) |
| **TUI** | No | No | Yes |
| **Hot Reload** | Trunk | cargo-leptos | dx serve --hotpatch |
| **Maturity** | Most mature | Stabilizing toward 1.0 | Solid web, growing ecosystem |

### 6.2 When to Use Each

**Yew**: Best for teams wanting stability and a React-like mental model. Largest community, most established component libraries. Conservative organizations prioritizing API stability.

**Leptos**: Best for performance-critical applications. Smallest bundle sizes, surgical DOM updates, full-stack with type-safe server functions. Closest to native vanilla JS DOM performance.

**Dioxus**: Best for cross-platform (web + desktop + mobile + TUI). React-familiar syntax, sub-second hot-patch reloads in v0.7. Ideal for teams with JavaScript experience wanting to target multiple platforms.

### 6.3 Performance Reality

Leptos and Dioxus both significantly outperform React JS and Yew. Despite using WebAssembly, they achieve near-native vanilla JavaScript DOM performance. Yew's virtual DOM diffing adds overhead but prevents many common bugs.

### 6.4 Recommendation for Tauri Desktop Apps

For a **Tauri desktop app** specifically, the Svelte frontend remains the recommended choice because:
1. Smallest possible bundle (compiles away the framework)
2. No WASM overhead
3. Largest ecosystem of UI component libraries
4. Simpler developer experience for UI-heavy apps
5. Better tooling maturity for desktop patterns

Rust frontend frameworks (Leptos/Dioxus) make sense when:
- You want a 100% Rust stack (no JavaScript at all)
- Your team has deep Rust expertise but limited JS/TS experience
- You need cross-platform rendering targets beyond what Tauri offers (TUI, native desktop without WebView)

---

## 7. Cross-Platform Build Strategies

### 7.1 GitHub Actions with tauri-action

**Official tool**: `tauri-apps/tauri-action` - Builds native binaries for all platforms

**Matrix strategy workflow:**
```yaml
name: Release
on:
  push:
    tags: ['v*']

jobs:
  release:
    strategy:
      matrix:
        include:
          - platform: ubuntu-22.04
            args: ''
          - platform: windows-latest
            args: ''
          - platform: macos-latest
            args: '--target aarch64-apple-darwin'
          - platform: macos-latest
            args: '--target x86_64-apple-darwin'

    runs-on: ${{ matrix.platform }}
    steps:
      - uses: actions/checkout@v4

      - name: Install dependencies (Linux)
        if: matrix.platform == 'ubuntu-22.04'
        run: |
          sudo apt-get update
          sudo apt-get install -y libwebkit2gtk-4.1-dev

      - uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          TAURI_SIGNING_PRIVATE_KEY: ${{ secrets.TAURI_SIGNING_PRIVATE_KEY }}
        with:
          tagName: v__VERSION__
          releaseName: 'v__VERSION__'
```

### 7.2 Platform-Specific Notes

| Platform | WebView | Requirements | Architectures |
|----------|---------|-------------|---------------|
| **Windows** | WebView2 | Pre-installed Win 10+ | x64 |
| **macOS** | WebKit | Xcode CLI tools | x86_64, aarch64, universal |
| **Linux** | WebKitGTK | libwebkit2gtk-4.1-dev | x86_64 |

### 7.3 macOS Universal Binaries

Build for both Intel and Apple Silicon:
```bash
# Individual builds
cargo tauri build --target x86_64-apple-darwin
cargo tauri build --target aarch64-apple-darwin

# Universal binary
cargo tauri build --target universal-apple-darwin
```

### 7.4 Linux Distribution

**AppImage** (recommended for widest compatibility):
- Self-contained, runs on most Linux distros
- Include all GTK/WebKit dependencies

**Debian (.deb)**:
- For Ubuntu/Debian users
- Proper dependency management via apt

---

## 8. Offline-First AI Architecture with Tauri

### 8.1 Three-Layer Architecture

```
+--------------------------------------------------+
|  Layer 1: Frontend (Svelte 5 + Tailwind)          |
|  - Chat UI, settings panels, model management     |
|  - Communicates via Tauri IPC commands/events      |
+--------------------------------------------------+
|  Layer 2: Rust Mediator (Tauri Backend)            |
|  - State management, business logic               |
|  - SQLite/SurrealDB for local persistence          |
|  - FastEmbed for local embeddings                  |
|  - Plugin system for extensibility                 |
+--------------------------------------------------+
|  Layer 3: AI Inference Layer                       |
|  Option A: Sidecar (llama.cpp / candle binary)     |
|  Option B: HTTP client (Ollama on localhost)       |
|  Option C: Embedded (candle in Rust backend)       |
+--------------------------------------------------+
```

### 8.2 Sidecar Pattern for Local LLM

**Architecture:**
- LLM sidecar runs as a separate high-performance binary
- Tauri manages spawning, IPC channels, and permission gating
- Fault isolation: if LLM crashes, app continues
- Simplified debugging with separated concerns

**Implementation options:**

| Approach | Binary | Performance | Complexity |
|----------|--------|-------------|-----------|
| **Ollama HTTP** | External (user-installed) | Best (GPU optimized) | Lowest |
| **llama.cpp sidecar** | Bundled | Very good (CPU/GPU) | Medium |
| **Candle embedded** | In-process | Good (CPU/GPU via Rust) | Highest |
| **candle-vllm sidecar** | Bundled | Very good | Medium |

**Sidecar binary bundling:**
```
src-tauri/
  runtime/
    llama-cpp/
      bin/
        x86_64-unknown-linux-gnu
        x86_64-pc-windows-msvc.exe
        aarch64-apple-darwin
```

### 8.3 Ollama Integration Pattern

```rust
// Tauri command for Ollama chat
#[tauri::command]
async fn chat_with_model(
    app: tauri::AppHandle,
    model: String,
    prompt: String,
) -> Result<(), String> {
    let client = reqwest::Client::new();
    let mut stream = client
        .post("http://localhost:11434/api/generate")
        .json(&serde_json::json!({
            "model": model,
            "prompt": prompt,
            "stream": true
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .bytes_stream();

    while let Some(chunk) = stream.next().await {
        let data = chunk.map_err(|e| e.to_string())?;
        app.emit("chat-token", &data).map_err(|e| e.to_string())?;
    }
    Ok(())
}
```

### 8.4 Performance Benchmarks

| Hardware | Model | Tokens/sec | Method |
|----------|-------|------------|--------|
| Modern CPU | 7B Q4 | 10-20 | llama.cpp |
| NVIDIA GPU | 7B Q4 | 80-120 | llama.cpp CUDA |
| AMD GPU | 7B Q4 | 60-100 | llama.cpp ROCm |
| Apple M-series | 7B Q4 | 30-50 | llama.cpp Metal |

### 8.5 Privacy Model

- All inference runs locally (no cloud dependency)
- Prompts and data never leave the device
- SQLite/SurrealDB for local vector storage
- FastEmbed for local embedding generation
- Optional cloud fallback with explicit user consent

---

## 9. Performance Optimization Playbook

### 9.1 Rust Binary Optimization

```toml
# Cargo.toml - Production profile
[profile.release]
opt-level = "z"        # Optimize for size (or "s" for balanced)
lto = true             # Link Time Optimization
codegen-units = 1      # Single codegen unit (better optimization)
strip = true           # Remove debug symbols (20-30% reduction)
panic = "abort"        # No unwinding overhead
```

**Results:**
- 40% binary size reduction with these settings alone
- Combined with LTO and PGO on Windows: 30% execution speed improvement

### 9.2 Frontend Bundle Optimization

**Vite/SvelteKit optimizations:**
- Tree shaking (enabled by default): 50%+ JS bundle reduction
- Route-based code splitting with dynamic imports
- Image optimization (WebP/AVIF + lazy loading)
- Minification and compression

### 9.3 Startup Time

**Target**: 0.2 - 0.9 seconds cold start

**Techniques:**
- Defer non-essential initialization
- Async database connections and network requests
- Lazy-load heavy modules
- Pre-allocate `Vec` capacity with `with_capacity`

**Benchmarks:**
- Business tool: 3 seconds down to under 1 second
- Dependency removal: 40% startup improvement

### 9.4 Memory Efficiency

**Rust-side:**
- Leverage ownership system (no GC pause times)
- Use `Weak` references to avoid circular references
- Streaming APIs for large data processing
- Pre-allocate collections

**Frontend-side:**
- Virtual scrolling for long lists (`svelte-virtual-list`)
- Cleanup side effects in `$effect` returns
- Streaming APIs for large data

### 9.5 Build Time Optimization

| Technique | Improvement |
|-----------|-------------|
| `sccache` (shared compilation cache) | 50% build time reduction |
| Parallel compilation (`cargo build -j`) | 40% CI/CD improvement |
| Vite (over Webpack) | Hot reload: seconds to milliseconds |
| Incremental compilation | 60-80% rebuild speedup |

### 9.6 Rendering Performance

- Virtual scrolling for 100K+ record datasets
- DOM batching with Svelte's built-in update coalescing
- WebP/AVIF image formats with lazy loading
- CSS containment for isolated paint regions

**Benchmark**: Dashboard rendering time reduced by 70% with these techniques.

---

## 10. Licensing Summary

All recommended technologies use permissive open-source licenses suitable for commercial use:

| Technology | License | Commercial Use | Notes |
|------------|---------|---------------|-------|
| **Tauri** | MIT / Apache-2.0 | Yes | Dual-licensed |
| **Svelte / SvelteKit** | MIT | Yes | Fully permissive |
| **Rust** | MIT / Apache-2.0 | Yes | Language + toolchain |
| **rusqlite** | MIT | Yes | |
| **SurrealDB** | Apache-2.0 (embedded) | Yes | BSL for some cloud features |
| **Axum** | MIT | Yes | |
| **FastEmbed-rs** | MIT | Yes | |
| **Leptos** | MIT | Yes | |
| **Dioxus** | MIT / Apache-2.0 | Yes | Dual-licensed |
| **Yew** | MIT / Apache-2.0 | Yes | Dual-licensed |
| **shadcn-svelte** | MIT | Yes | |
| **Bits UI** | MIT | Yes | |
| **GitHub Spec Kit** | MIT | Yes | |
| **Candle** | MIT / Apache-2.0 | Yes | HuggingFace |

---

## 11. Recommendations for Nexus

Based on this research, the following recommendations apply to the Nexus AI Workstation Builder:

### 11.1 Confirmed Stack (Already in Use)

| Layer | Technology | Status |
|-------|-----------|--------|
| Framework | Tauri 2.x | Correct choice |
| Frontend | Svelte 5 + Runes | Correct choice |
| Styling | Tailwind CSS | Correct choice |
| Database | SQLite (rusqlite, WAL mode) | Correct choice |
| Components | shadcn-svelte | Recommended addition |

### 11.2 Recommended Additions

1. **FastEmbed-rs** for local embedding generation (no Python dependency)
2. **tauri-plugin-axum** for embedded API server (plugin communication)
3. **GitHub Spec Kit** for structured feature development workflow
4. **MessagePack IPC** for large payload transfer (model weights, embeddings)
5. **CrabNebula Cloud** or self-hosted for auto-update infrastructure

### 11.3 Architecture Patterns to Adopt

1. **Sidecar Pattern**: Bundle llama.cpp/candle binaries for platforms where Ollama is not installed
2. **Channel IPC**: Use `tauri::ipc::Channel` for LLM token streaming
3. **Capabilities-Based Security**: Define minimal permissions per window
4. **Progressive Disclosure UX**: Advanced settings behind expandable sections
5. **Offline-First with Cloud Fallback**: Local inference default, API fallback optional

### 11.4 Build & Distribution

1. GitHub Actions matrix builds (Linux/Windows/macOS)
2. Tauri update signing keypair for auto-updates
3. macOS universal binaries (Intel + Apple Silicon)
4. AppImage for Linux distribution
5. NSIS installer for Windows

### 11.5 Performance Targets

| Metric | Target | Technique |
|--------|--------|-----------|
| Binary size | < 5 MB | opt-level="z", LTO, strip |
| Cold start | < 1 second | Lazy init, async DB |
| RAM idle | < 50 MB | Native WebView, Rust ownership |
| IPC latency | < 1 ms | Raw payloads, MessagePack |
| Build time | < 3 minutes | sccache, parallel |

---

## Sources

### Tauri 2.x
- [Tauri Official Documentation](https://v2.tauri.app/)
- [Tauri 2.0 Stable Release](https://v2.tauri.app/blog/tauri-20/)
- [Tauri IPC Concepts](https://v2.tauri.app/concept/inter-process-communication/)
- [Tauri Security Model](https://v2.tauri.app/security/)
- [Tauri Plugin Development](https://v2.tauri.app/develop/plugins/)
- [Tauri App Size Optimization](https://v2.tauri.app/concept/size/)
- [Tauri v2 Performance & Bundle Size Guide](https://www.oflight.co.jp/en/columns/tauri-v2-performance-bundle-size)
- [12 Proven Tauri 2.0 AI App Techniques](https://ainexislab.com/tauri-2-0-ai-app-desktop-development-techniques/)
- [Building Desktop Apps with Rust and Tauri: 2025 Guide](https://www.plutenium.com/blog/building-desktop-apps-with-rust-and-tauri)
- [Rust & React/Vue/Tauri Frontends (2026)](https://dasroot.net/posts/2026/02/rust-react-vue-tauri-desktop-apps/)
- [Tauri GitHub Action](https://github.com/tauri-apps/tauri-action)
- [Tauri Cross-Platform Builds](https://v2.tauri.app/distribute/pipelines/github/)
- [Tauri Updater Plugin](https://v2.tauri.app/plugin/updater/)
- [Ship Tauri v2: Code Signing](https://dev.to/tomtomdu73/ship-your-tauri-v2-app-like-a-pro-code-signing-for-macos-and-windows-part-12-3o9n)
- [Production macOS App with Tauri 2.0](https://dev.to/0xmassi/shipping-a-production-macos-app-with-tauri-20-code-signing-notarization-and-homebrew-mc3)
- [3MB Tauri App Size](https://medium.com/@connect.hashblock/how-i-achieved-a-3mb-app-size-in-tauri-without-sacrificing-ux-0e9f09ded46e)
- [Tauri Testing](https://v2.tauri.app/develop/tests/)

### Svelte 5
- [Svelte 5 Runes Introduction](https://svelte.dev/blog/runes)
- [Svelte 5 $state Docs](https://svelte.dev/docs/svelte/$derived)
- [Svelte 5 Runes Complete Guide](https://teta.so/blog/svelte-5-runes-state-derived-effect)
- [Runes and Global State](https://mainmatter.com/blog/2025/03/11/global-state-in-svelte-5/)
- [SvelteKit Tauri Configuration](https://v2.tauri.app/start/frontend/sveltekit/)
- [Svelte 5 2025 Review](https://www.scalablepath.com/javascript/svelte-5-review)
- [Tauri + Rust + Svelte Stack (2025)](https://medium.com/@puneetpm/native-apps-reimagined-why-tauri-rust-and-svelte-is-my-go-to-stack-in-2025-209f5b2937a1)
- [shadcn-svelte](https://www.shadcn-svelte.com/)
- [Tauri 2 + Svelte 5 + shadcn Template](https://github.com/alysonhower/tauri2-svelte5-shadcn)

### Rust Full-Stack
- [Rust ORMs in 2026 Comparison](https://aarambhdevhub.medium.com/rust-orms-in-2026-diesel-vs-sqlx-vs-seaorm-vs-rusqlite-which-one-should-you-actually-use-706d0fe912f3)
- [Rusqlite GitHub](https://github.com/rusqlite/rusqlite)
- [SurrealDB Embedded Power](https://surrealdb.com/blog/the-power-of-surrealdb-embedded)
- [SurrealDB Embedded Rust](https://surrealdb.com/docs/surrealdb/embedding/rust)
- [AWESOME-APP (Tauri + SurrealDB)](https://awesomeapp.dev/)
- [FastEmbed-rs GitHub](https://github.com/Anush008/fastembed-rs)
- [Candle ML Framework](https://github.com/huggingface/candle)
- [Rust Web Frameworks 2026](https://aarambhdevhub.medium.com/rust-web-frameworks-in-2026-axum-vs-actix-web-vs-rocket-vs-warp-vs-salvo-which-one-should-you-2db3792c79a2)
- [tauri-plugin-axum](https://docs.rs/tauri-plugin-axum)

### Spec-Driven Development
- [Thoughtworks: Spec-Driven Development](https://www.thoughtworks.com/en-us/insights/blog/agile-engineering-practices/spec-driven-development-unpacking-2025-new-engineering-practices)
- [GitHub Blog: Spec Kit](https://github.blog/ai-and-ml/generative-ai/spec-driven-development-with-ai-get-started-with-a-new-open-source-toolkit/)
- [GitHub Spec Kit Repository](https://github.com/github/spec-kit)
- [SDD with Claude Code](https://heeki.medium.com/using-spec-driven-development-with-claude-code-4a1ebe5d9f29)
- [cc-sdd (Kiro-style)](https://github.com/gotalab/cc-sdd)
- [SDD Complete Guide 2025](https://www.softwareseni.com/spec-driven-development-in-2025-the-complete-guide-to-using-ai-to-write-production-code/)
- [Augment Code: SDD Guide](https://www.augmentcode.com/guides/what-is-spec-driven-development)
- [MIT Technology Review: Context Engineering 2025](https://www.technologyreview.com/2025/11/05/1127477/from-vibe-coding-to-context-engineering-2025-in-software-development/)

### Context Engineering
- [Martin Fowler: Context Engineering for Coding Agents](https://martinfowler.com/articles/exploring-gen-ai/context-engineering-coding-agents.html)
- [Writing a Good CLAUDE.md](https://www.humanlayer.dev/blog/writing-a-good-claude-md)
- [Claude Code Best Practices](https://www.anthropic.com/engineering/claude-code-best-practices)
- [AGENTS.md Best Tips](https://www.builder.io/blog/agents-md)
- [Context Engineering Intro](https://github.com/coleam00/context-engineering-intro)
- [Effective Context Engineering (Anthropic)](https://www.anthropic.com/engineering/effective-context-engineering-for-ai-agents)

### Desktop UX
- [Microsoft: New UI for Enterprise AI](https://microsoft.design/articles/the-new-ui-for-enterprise-ai/)
- [Agentic AI Design Patterns](https://www.aufaitux.com/blog/agentic-ai-design-patterns-enterprise-guide/)
- [AI Design Patterns for Enterprise Dashboards](https://www.aufaitux.com/blog/ai-design-patterns-enterprise-dashboards/)
- [Enterprise UX Design Patterns](https://www.onething.design/post/top-7-enterprise-ux-design-patterns)
- [Enterprise AI App Builders 2026](https://reflex.dev/blog/2025-12-17-top-7-enterprise-ai-app-builders-2026/)

### Rust Frontend Frameworks
- [Leptos vs Yew vs Dioxus 2026](https://reintech.io/blog/leptos-vs-yew-vs-dioxus-rust-frontend-framework-comparison-2026)
- [Dioxus 0.7 Release](https://medium.com/@trivajay259/dioxus-0-7-the-rust-ui-release-that-finally-feels-full-stack-everywhere-89f482ee97e3)
- [Leptos GitHub](https://github.com/leptos-rs/leptos)
- [Dioxus Official](https://dioxuslabs.com/)
- [Leptos Book](https://book.leptos.dev/)

### Offline-First AI
- [Technical Blueprint: Local-First AI with Rust & Tauri](https://medium.com/@Musbell008/a-technical-blueprint-for-local-first-ai-with-rust-and-tauri-b9211352bc0e)
- [Building Local LM Desktop Apps with Tauri](https://medium.com/@dillon.desilva/building-local-lm-desktop-applications-with-tauri-f54c628b13d9)
- [Local AI with Postgres + pgvector + Tauri](https://electric-sql.com/blog/2024/02/05/local-first-ai-with-tauri-postgres-pgvector-llama)
- [Tauri + Ollama Client](https://github.com/elijahmg/ollama-tauri-client)
- [Crane: Pure Rust LLM Inference](https://github.com/lucasjinreal/Crane)
- [Evil Martians: Tauri + Sidecar](https://evilmartians.com/chronicles/making-desktop-apps-with-revved-up-potential-rust-tauri-sidecar)
