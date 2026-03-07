# NEXUS Tech Research - Tauri 2 + SvelteKit 5 Desktop Application

**Date**: 2026-03-07
**Purpose**: Technical research for NEXUS AI Workstation Builder

---

## 1. PaneForge - Resizable Split Panes for Svelte 5

### Package Info
- **npm**: `paneforge` ([npmjs.com/package/paneforge](https://www.npmjs.com/package/paneforge))
- **Docs**: [paneforge.com/docs](https://paneforge.com/docs)
- **GitHub**: [svecosystem/paneforge](https://github.com/svecosystem/paneforge)
- **Svelte 5**: Fully supported (tagged `svelte-5` on GitHub)
- **Inspiration**: React-resizable-panels by Bryan Vaughn

### Installation
```bash
npm install paneforge
```

### Core Components

| Component | Purpose |
|-----------|---------|
| `PaneGroup` | Container, sets direction (`horizontal` / `vertical`) |
| `Pane` | Individual resizable panel |
| `PaneResizer` | Draggable handle between panes |

### Basic Usage (Svelte 5)
```svelte
<script lang="ts">
  import { PaneGroup, Pane, PaneResizer } from "paneforge";
</script>

<PaneGroup direction="horizontal">
  <Pane defaultSize={50}>Pane 1</Pane>
  <PaneResizer />
  <Pane defaultSize={50}>Pane 2</Pane>
</PaneGroup>
```

### Collapsible Panes (IDE-style sidebar)
```svelte
<script lang="ts">
  import { PaneGroup, Pane, PaneResizer } from "paneforge";

  let paneOne: ReturnType<typeof Pane>;
  let collapsed = $state(false);
</script>

{#if collapsed}
  <button onclick={paneOne.expand}>Expand</button>
{:else}
  <button onclick={paneOne.collapse}>Collapse</button>
{/if}

<PaneGroup direction="horizontal">
  <Pane
    defaultSize={50}
    collapsedSize={5}
    collapsible={true}
    minSize={15}
    bind:this={paneOne}
    onCollapse={() => (collapsed = true)}
    onExpand={() => (collapsed = false)}
  />
  <PaneResizer />
  <Pane defaultSize={50}>
    <!-- Nested vertical split -->
    <PaneGroup direction="vertical">
      <Pane defaultSize={50} />
      <PaneResizer />
      <Pane defaultSize={50} />
    </PaneGroup>
  </Pane>
</PaneGroup>
```

### Pane Props
| Prop | Type | Description |
|------|------|-------------|
| `defaultSize` | `number` | Initial size as percentage |
| `minSize` | `number` | Minimum size percentage |
| `maxSize` | `number` | Maximum size percentage |
| `collapsible` | `boolean` | Whether pane can collapse |
| `collapsedSize` | `number` | Size when collapsed |
| `onCollapse` | `() => void` | Callback on collapse |
| `onExpand` | `() => void` | Callback on expand |
| `onResize` | `(size: number) => void` | Callback on resize |

### PaneResizer Props
| Prop | Type | Description |
|------|------|-------------|
| `disabled` | `boolean` | Disable resize handle |
| `onDraggingChange` | `(isDragging: boolean) => void` | Drag state callback |

### PaneResizer Data Attributes (for CSS styling)
- `data-direction` - `"horizontal"` or `"vertical"`
- `data-active` - `"pointer"` or `"keyboard"` when active
- `data-enabled` - boolean
- `data-pane-resizer` - marker attribute

### Features
- Nested groups for complex layouts (IDE-like)
- LocalStorage/cookie persistence for layout state
- Keyboard accessible
- Provides only positioning CSS -- full visual control to developer

### NEXUS Application
Use PaneForge for the main IDE-like layout:
- Left: Service sidebar (collapsible)
- Center: Main content area (chat, editor, etc.)
- Right: Optional panel (settings, logs)
- Vertical splits within center for terminal/output

---

## 2. WebView in Tauri 2

### Creating Additional WebView Windows

**Docs**: [v2.tauri.app/reference/javascript/api/namespacewebview](https://v2.tauri.app/reference/javascript/api/namespacewebview/)

#### From JavaScript (Frontend)
```typescript
import { Window } from '@tauri-apps/api/window';
import { Webview } from '@tauri-apps/api/webview';

// Create a new window with a webview pointing to localhost service
const appWindow = new Window('n8n-browser');
const webview = new Webview(appWindow, 'n8n-webview', {
  url: 'http://localhost:5678',  // n8n workflow editor
  x: 0,
  y: 0,
  width: 1200,
  height: 800,
});
```

#### From Rust (Backend)
```rust
use tauri::webview::WebviewWindowBuilder;

// Create webview window from Rust
fn open_service_browser(app: &tauri::AppHandle, url: &str, label: &str) {
    WebviewWindowBuilder::new(app, label, tauri::WebviewUrl::External(url.parse().unwrap()))
        .title(format!("NEXUS - {}", label))
        .inner_size(1200.0, 800.0)
        .build()
        .unwrap();
}
```

#### WebviewBuilder (child webview in existing window)
```rust
use tauri::webview::WebviewBuilder;

// Add a webview as a child of an existing window
fn embed_webview(window: &tauri::Window, url: &str) {
    let webview = WebviewBuilder::new(
        "embedded-browser",
        tauri::WebviewUrl::External(url.parse().unwrap())
    );
    window.add_child(
        webview,
        tauri::LogicalPosition::new(0, 0),
        tauri::LogicalSize::new(800, 600)
    ).unwrap();
}
```

### Required Permissions
Add to `src-tauri/capabilities/default.json`:
```json
{
  "permissions": [
    "core:webview:allow-create-webview-window",
    "core:webview:allow-create-webview",
    "core:window:allow-create"
  ]
}
```

### NEXUS Application
- Embed n8n (localhost:5678) as a webview tab within the main window
- Open Ollama web UI, ComfyUI, or other localhost services
- Use `WebviewBuilder` for in-window embedding alongside PaneForge panels
- Use `WebviewWindowBuilder` for pop-out windows

### Key Considerations
- External URLs require the `http` or `https` scheme
- Webviews share the same web process when created with `WebviewBuilder`
- Each webview has its own navigation state
- Cross-origin restrictions apply (CSP configuration needed for localhost)

---

## 3. SSE Streaming in Tauri 2 (Rust to Frontend)

### Recommended: `tauri::ipc::Channel<T>`

**Docs**: [v2.tauri.app/develop/calling-frontend](https://v2.tauri.app/develop/calling-frontend/)

Channels are the recommended mechanism for streaming data. They provide **fast, ordered delivery** optimized for streaming scenarios.

### Rust Side: Define Events and Command

```rust
use tauri::ipc::Channel;
use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
enum ChatStreamEvent {
    #[serde(rename_all = "camelCase")]
    Started {
        model: String,
        request_id: String,
    },
    #[serde(rename_all = "camelCase")]
    Delta {
        content: String,
    },
    #[serde(rename_all = "camelCase")]
    Finished {
        request_id: String,
        usage: TokenUsage,
    },
    #[serde(rename_all = "camelCase")]
    Error {
        message: String,
    },
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct TokenUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
}

#[tauri::command]
async fn chat_stream(
    message: String,
    model: String,
    on_event: Channel<ChatStreamEvent>,
) -> Result<(), String> {
    on_event.send(ChatStreamEvent::Started {
        model: model.clone(),
        request_id: "req-123".to_string(),
    }).map_err(|e| e.to_string())?;

    // Stream from OpenRouter/Ollama via reqwest
    // For each SSE chunk received:
    on_event.send(ChatStreamEvent::Delta {
        content: "Hello ".to_string(),
    }).map_err(|e| e.to_string())?;

    on_event.send(ChatStreamEvent::Finished {
        request_id: "req-123".to_string(),
        usage: TokenUsage {
            prompt_tokens: 50,
            completion_tokens: 120,
        },
    }).map_err(|e| e.to_string())?;

    Ok(())
}
```

### Frontend Side: Receive Stream Events

```typescript
import { invoke, Channel } from '@tauri-apps/api/core';

type ChatStreamEvent =
  | { event: 'started'; data: { model: string; requestId: string } }
  | { event: 'delta'; data: { content: string } }
  | { event: 'finished'; data: { requestId: string; usage: { promptTokens: number; completionTokens: number } } }
  | { event: 'error'; data: { message: string } };

async function sendChat(message: string, model: string) {
  let fullResponse = '';

  const onEvent = new Channel<ChatStreamEvent>();
  onEvent.onmessage = (event) => {
    switch (event.event) {
      case 'started':
        console.log(`Streaming from ${event.data.model}`);
        break;
      case 'delta':
        fullResponse += event.data.content;
        // Update UI reactively
        break;
      case 'finished':
        console.log(`Done. Tokens: ${event.data.usage.completionTokens}`);
        break;
      case 'error':
        console.error(event.data.message);
        break;
    }
  };

  await invoke('chat_stream', { message, model, onEvent });
}
```

### Alternative: Tauri Event System
```rust
// Rust: emit events globally
app_handle.emit("chat-delta", payload)?;

// Frontend: listen
import { listen } from '@tauri-apps/api/event';
await listen('chat-delta', (event) => { /* ... */ });
```

Events are simpler but NOT optimized for high-throughput streaming. Use Channels for chat streaming.

### Also Available: `tauri-plugin-sse`
- **Crate**: [crates.io/crates/tauri-plugin-sse](https://crates.io/crates/tauri-plugin-sse)
- Wraps native SSE into Tauri plugin
- Useful if connecting directly to an SSE endpoint (like Ollama)

### NEXUS Architecture for Chat Streaming
```
Frontend (SvelteKit)          Rust Backend               External
┌─────────────┐    Channel    ┌──────────────┐  reqwest   ┌──────────┐
│ Chat UI     │◄──────────────│ chat_stream  │──────────►│ OpenRouter│
│ (Svelte 5)  │  ChatEvent    │ command      │  SSE       │ /Ollama  │
└─────────────┘               └──────────────┘           └──────────┘
```

---

## 4. OpenRouter API

### Overview
- **Base URL**: `https://openrouter.ai/api/v1`
- **Format**: OpenAI-compatible (`/chat/completions`)
- **Models**: 400+ models through one API
- **Docs**: [openrouter.ai/docs](https://openrouter.ai/docs/quickstart)

### Authentication
```
Authorization: Bearer <OPENROUTER_API_KEY>
```

Optional headers:
```
HTTP-Referer: https://your-app.com    (for ranking/analytics)
X-OpenRouter-Title: NEXUS              (your app name)
```

### Standard Chat Request
```json
POST https://openrouter.ai/api/v1/chat/completions
{
  "model": "anthropic/claude-sonnet-4",
  "messages": [
    { "role": "system", "content": "You are a helpful assistant." },
    { "role": "user", "content": "Hello!" }
  ],
  "stream": false
}
```

### Streaming Request
```json
POST https://openrouter.ai/api/v1/chat/completions
{
  "model": "anthropic/claude-sonnet-4",
  "messages": [{ "role": "user", "content": "Explain quantum computing" }],
  "stream": true
}
```

### SSE Response Format
```
data: {"id":"gen-abc","object":"chat.completion.chunk","choices":[{"delta":{"content":"Quantum"},"index":0}]}

data: {"id":"gen-abc","object":"chat.completion.chunk","choices":[{"delta":{"content":" computing"},"index":0}]}

data: [DONE]
```

Notes:
- Ignore `data: : OPENROUTER PROCESSING` keepalive comments
- `usage` object appears only in the final chunk
- `finish_reason` in last chunk: `"stop"`, `"length"`, or `"error"`

### Error Handling
| Code | Meaning |
|------|---------|
| 400 | Bad request (invalid params) |
| 401 | Invalid API key |
| 402 | Insufficient credits |
| 429 | Rate limited |
| 502/503 | Provider error |

Mid-stream errors arrive as SSE events with `finish_reason: "error"`.

### Free Tier
- **Router**: `openrouter/free` (random free model selection)
- **Rate limits**: 50 req/day (no credits) or 1000 req/day (with $10+ credits purchased)
- **Cost**: $0/M input + $0/M output tokens
- **Context**: Up to 200K tokens
- **Available free models** (as of 2026-03):
  - Google Gemma 3 (4B, 12B, 27B)
  - Meta Llama 3.2/3.3 variants
  - Mistral Small 3.1 24B
  - Qwen 3 variants
  - Nvidia Nemotron variants
  - StepFun Step 3.5 Flash
  - OpenAI GPT-OSS 20B
  - Liquid LFM 2.5

### Rust Implementation (reqwest + SSE)
```rust
use reqwest::Client;
use futures_util::StreamExt;

async fn stream_openrouter(
    api_key: &str,
    model: &str,
    messages: Vec<Message>,
    on_delta: impl Fn(&str),
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("HTTP-Referer", "https://nexus-app.local")
        .header("X-OpenRouter-Title", "NEXUS")
        .json(&serde_json::json!({
            "model": model,
            "messages": messages,
            "stream": true
        }))
        .send()
        .await?;

    let mut stream = response.bytes_stream();
    let mut buffer = String::new();

    while let Some(chunk) = stream.next().await {
        let text = String::from_utf8_lossy(&chunk?);
        buffer.push_str(&text);

        for line in buffer.lines() {
            if line.starts_with("data: [DONE]") {
                return Ok(());
            }
            if let Some(json_str) = line.strip_prefix("data: ") {
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(json_str) {
                    if let Some(content) = parsed["choices"][0]["delta"]["content"].as_str() {
                        on_delta(content);
                    }
                }
            }
        }
        buffer.clear();
    }
    Ok(())
}
```

### Stream Cancellation
Supported via `AbortController` (JS) or dropping the request (Rust). Works with 20+ providers.

---

## 5. Intelligent Model Routing - RouteLLM

### Paper
- **Title**: RouteLLM: Learning to Route LLMs with Preference Data
- **Authors**: Isaac Ong, Amjad Almahairi, Vincent Wu, Wei-Lin Chiang, Tianhao Wu, Joseph E. Gonzalez, M Waleed Kadous, Ion Stoica
- **Affiliations**: UC Berkeley, Anyscale, Canva
- **Published**: ICLR 2025 (conference paper)
- **arXiv**: [2406.18665](https://arxiv.org/abs/2406.18665)
- **GitHub**: [lm-sys/RouteLLM](https://github.com/lm-sys/RouteLLM)
- **Blog**: [lmsys.org/blog/2024-07-01-routellm](https://lmsys.org/blog/2024-07-01-routellm/)

### Core Concept
Route between a **strong** (expensive) model and a **weak** (cheap) model per-query. The router learns from human preference data (Chatbot Arena) which queries need the strong model.

### Four Router Architectures

| Router | Method | Best For |
|--------|--------|----------|
| **SW (Similarity-Weighted)** | Weighted Elo based on prompt similarity | Fast, lightweight |
| **Matrix Factorization** | Learned scoring for model-prompt compatibility | Good generalization |
| **BERT Classifier** | Neural binary classifier (strong vs weak) | High accuracy |
| **Causal LLM** | Language model comparison | Most flexible |

### Results (Cost Savings)

| Benchmark | GPT-4 Quality Maintained | Cost Reduction |
|-----------|--------------------------|----------------|
| MT Bench | 95% | 75-86% fewer GPT-4 calls |
| MMLU | 95% | ~46% fewer calls |
| GSM8K | 95% | ~35% fewer calls |

### Key Insight for NEXUS
The router generalizes across model pairs without retraining. This means:
- Train on GPT-4 vs Mixtral preference data
- Apply to Claude vs Llama, or any strong/weak pair
- Works for OpenRouter (expensive model) vs Ollama (free local model)

### NEXUS Routing Strategy
```
User Query
    │
    ▼
┌─────────────────┐
│ Task Classifier  │  (lightweight BERT or rule-based)
│ (RouteLLM-style)│
└────────┬────────┘
         │
    ┌────┴────┐
    │ Simple? │
    └────┬────┘
     Yes │    No
         │         │
    ┌────▼────┐  ┌─▼──────────┐
    │ Local   │  │ OpenRouter  │
    │ Ollama  │  │ Claude/GPT  │
    │ (Free)  │  │ (Paid)      │
    └─────────┘  └─────────────┘
```

### Practical Implementation for NEXUS

1. **Rule-based first** (no ML needed):
   - Code completion / simple Q&A -> local Ollama (Qwen 2.5 Coder)
   - Complex reasoning / long context -> OpenRouter (Claude)
   - Translation / formatting -> local model
   - Creative writing / analysis -> cloud model

2. **ML-based later** (RouteLLM framework):
   - Collect user preference data from NEXUS usage
   - Train lightweight classifier
   - Dynamically route based on learned patterns

### Related Work
- **"Learning How Hard to Think"** (ICLR 2025) - Adaptive compute allocation
- **Swfte AI** - Commercial intelligent routing (85% cost reduction claims)

---

## 6. Opera GX UI Design - Dark Theme with Neon Accents

### Opera GX Dark Mode Color Palette

| Role | Hex | RGB | Usage |
|------|-----|-----|-------|
| **Neon Accent** | `#fa1e4e` | (250, 30, 78) | Primary highlights, buttons, active states |
| **Deep Background** | `#251f33` | (37, 31, 51) | Main background, panels |
| **Light Surface** | `#e3dfec` | (227, 223, 236) | Cards, elevated surfaces (light mode) |
| **Text Primary** | `#bbbbbb` | (187, 187, 187) | Primary text |
| **Text Secondary** | `#aaaaaa` | (170, 170, 170) | Secondary text, labels |

### Extended Gaming Color Presets (Opera GX Secondary Colors)

| Preset | Primary Accent | Vibe |
|--------|---------------|------|
| **GX Classic** | `#fa1e4e` (Hot Pink) | Default gaming |
| **Ultraviolet** | `#7b2fff` (Purple) | Cyberpunk |
| **Hackerman** | `#00ff41` (Matrix Green) | Hacker |
| **Rose Quartz** | `#ff6b9d` (Soft Pink) | Pastel gamer |
| **Frutti di Mare** | `#ff6b35` (Orange) | Warm energy |

### NEXUS Color System (Opera GX Inspired)

```css
:root {
  /* Background layers (dark to darker) */
  --nexus-bg-deep:      #0d0d12;    /* Deepest background */
  --nexus-bg-primary:   #1a1a2e;    /* Main panels */
  --nexus-bg-secondary: #16213e;    /* Secondary panels */
  --nexus-bg-elevated:  #252540;    /* Cards, modals */
  --nexus-bg-hover:     #2d2d50;    /* Hover states */

  /* Neon accent (user-configurable) */
  --nexus-accent:       #fa1e4e;    /* Default: Opera GX hot pink */
  --nexus-accent-glow:  rgba(250, 30, 78, 0.3);  /* Glow effect */
  --nexus-accent-dim:   rgba(250, 30, 78, 0.15); /* Subtle tint */

  /* Alternative accents */
  --nexus-cyan:         #00d4ff;
  --nexus-purple:       #7b2fff;
  --nexus-green:        #00ff41;
  --nexus-orange:       #ff6b35;

  /* Text */
  --nexus-text-primary:   #e0e0e0;
  --nexus-text-secondary: #8888aa;
  --nexus-text-muted:     #555577;

  /* Borders */
  --nexus-border:       #2a2a45;
  --nexus-border-glow:  var(--nexus-accent);

  /* Status */
  --nexus-success:      #00ff88;
  --nexus-warning:      #ffaa00;
  --nexus-error:        #ff3355;
  --nexus-info:         #00aaff;
}
```

### Design Patterns

1. **Glow Effects**: Use `box-shadow` with accent color at low opacity
   ```css
   .active-panel {
     box-shadow: 0 0 15px var(--nexus-accent-glow),
                 inset 0 0 5px var(--nexus-accent-dim);
     border: 1px solid var(--nexus-accent);
   }
   ```

2. **Gradient Borders**: Neon border glow on focus/active
   ```css
   .input-field:focus {
     outline: none;
     border-color: var(--nexus-accent);
     box-shadow: 0 0 10px var(--nexus-accent-glow);
   }
   ```

3. **Animated Accents**: Subtle pulse on active elements
   ```css
   @keyframes neon-pulse {
     0%, 100% { opacity: 1; }
     50% { opacity: 0.7; }
   }
   .status-indicator.active {
     animation: neon-pulse 2s infinite;
     color: var(--nexus-accent);
   }
   ```

4. **Sidebar Pattern**: Dark sidebar with accent-highlighted active item
   ```css
   .sidebar-item.active {
     background: var(--nexus-accent-dim);
     border-left: 3px solid var(--nexus-accent);
     color: var(--nexus-text-primary);
   }
   ```

5. **Glass Morphism** (optional overlay panels):
   ```css
   .floating-panel {
     background: rgba(26, 26, 46, 0.85);
     backdrop-filter: blur(12px);
     border: 1px solid var(--nexus-border);
   }
   ```

### Typography
- **Monospace for code**: JetBrains Mono, Fira Code
- **UI font**: Inter, system-ui
- **Size scale**: 12px (small), 14px (body), 16px (heading), 20px (title)

### Firefox-GX Reference Theme
- [Godiesc/firefox-gx](https://github.com/Godiesc/firefox-gx) - Full Opera GX skin for Firefox
- Uses CSS variables for theming
- Offers presets: Fuchsia, Blue, Green, Poison, Swamp, Red-Blur, Purple-Sky
- Secondary: Purple, Aquamarine, Orange, Cyan, GreenLight, Yellow, Gray

---

## Summary: NEXUS Architecture Decisions

| Component | Technology | Rationale |
|-----------|-----------|-----------|
| **Layout** | PaneForge | Svelte 5 native, IDE-like resizable panels, collapsible sidebar |
| **Service Browser** | Tauri WebviewBuilder | Embed localhost services (n8n, Ollama UI) in tabs |
| **Chat Streaming** | tauri::ipc::Channel | Fast ordered delivery, typed events, Rust-native |
| **Cloud LLM API** | OpenRouter | 400+ models, OpenAI-compatible, free tier available |
| **Model Routing** | RouteLLM-inspired | Route simple tasks to local Ollama, complex to cloud |
| **UI Theme** | Opera GX dark + neon | Configurable accent color, glow effects, gaming aesthetic |
