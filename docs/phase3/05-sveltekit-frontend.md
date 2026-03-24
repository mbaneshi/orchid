# Spec 05 — SvelteKit Frontend

## Goal

Build a SvelteKit 5 dashboard that connects to the Orchid REST API (spec 04). Provides a visual interface for running agents, viewing artifacts, and managing workflows.

## Prerequisites

- Spec 04 (REST API) endpoints must exist
- pnpm (not npm)

## Scaffold

```bash
cd /Users/bm/claude-n8n-sqlite-supabse-adk-rust-sveltekit-openclaw
pnpm create svelte@latest frontend
# Select: Skeleton project, TypeScript (strict), ESLint, Prettier, Tailwind CSS
cd frontend
pnpm install
```

### Additional dependencies

```bash
pnpm add -D @tailwindcss/typography
```

## Project structure

```
frontend/
├── src/
│   ├── lib/
│   │   ├── api.ts              — API client (fetch wrapper)
│   │   ├── types.ts            — TypeScript types matching API responses
│   │   └── components/
│   │       ├── AgentCard.svelte
│   │       ├── ArtifactList.svelte
│   │       ├── FlowRunner.svelte
│   │       ├── Nav.svelte
│   │       └── StatusBadge.svelte
│   ├── routes/
│   │   ├── +layout.svelte      — Nav + shell layout
│   │   ├── +page.svelte        — Dashboard home
│   │   ├── agents/
│   │   │   └── +page.svelte    — Agent list + run UI
│   │   ├── flows/
│   │   │   └── +page.svelte    — Workflow list + run UI
│   │   ├── artifacts/
│   │   │   └── +page.svelte    — Artifact browser
│   │   └── sessions/
│   │       └── +page.svelte    — Session list
│   └── app.html
├── svelte.config.js
├── tailwind.config.js
├── tsconfig.json
└── package.json
```

## Detailed design

### 1. API client (`src/lib/api.ts`)

```typescript
const API_BASE = 'http://localhost:3100';

export async function fetchAgents() {
    const res = await fetch(`${API_BASE}/api/agents`);
    return res.json();
}

export async function runAgent(agent: string, repo?: string, input?: string) {
    const res = await fetch(`${API_BASE}/api/agents/run`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ agent, repo, input }),
    });
    return res.json();
}

export async function fetchFlows() { ... }
export async function runFlow(workflow: string, repo?: string, input?: string) { ... }
export async function fetchArtifacts(type?: string) { ... }
export async function fetchSessions() { ... }
```

Use SvelteKit's `$env/static/public` for API_BASE in production.

### 2. Types (`src/lib/types.ts`)

```typescript
export interface Agent {
    name: string;
    description: string;
}

export interface AgentRunResult {
    agent: string;
    output: string;
    artifact_id: string;
}

export interface Workflow {
    name: string;
    description: string;
    steps: WorkflowStep[];
}

export interface WorkflowStep {
    name: string;
    agent: string;
    output?: string;
}

export interface Artifact {
    id: string;
    type: string;
    content: string;
    created_at: string;
}

export interface Session {
    id: string;
    name: string;
    created_at: string;
    updated_at: string;
}
```

### 3. Pages

#### Dashboard (`routes/+page.svelte`)

- Shows quick stats: total agents, total artifacts, recent sessions
- Quick-run buttons for common flows (dev-to-content)
- Recent artifacts list (last 5)

Use Svelte 5 runes:
```svelte
<script lang="ts">
    import { fetchArtifacts, fetchAgents } from '$lib/api';

    let agents = $state<Agent[]>([]);
    let recentArtifacts = $state<Artifact[]>([]);

    $effect(() => {
        fetchAgents().then(r => agents = r.agents);
        fetchArtifacts().then(r => recentArtifacts = r.artifacts.slice(0, 5));
    });
</script>
```

#### Agents page (`routes/agents/+page.svelte`)

- Grid of AgentCard components
- Each card: name, description, "Run" button
- Click "Run" → expands form (repo path, input text)
- Submit → calls API → shows output in expandable section
- Running state: show spinner, disable button

#### Flows page (`routes/flows/+page.svelte`)

- List of available workflows
- Click → shows step visualization (simple vertical list of steps with arrows)
- "Run" button with repo/input form
- Output shows each step's result in sequence

#### Artifacts page (`routes/artifacts/+page.svelte`)

- Filter by artifact type (dropdown)
- List view with type, preview (first 200 chars), date
- Click → full artifact view in modal or expanded section
- Copy-to-clipboard button

#### Sessions page (`routes/sessions/+page.svelte`)

- Table: name, created, updated
- Click → detail view with associated artifacts
- Delete button (with confirmation)

### 4. Components

#### `Nav.svelte`
- Left sidebar or top nav
- Links: Dashboard, Agents, Flows, Artifacts, Sessions
- Active state highlighting
- "Orchid" branding at top

#### `AgentCard.svelte`
```svelte
<script lang="ts">
    import type { Agent } from '$lib/types';
    let { agent, onRun }: { agent: Agent; onRun: (name: string) => void } = $props();
</script>
```

#### `StatusBadge.svelte`
- Props: status ("idle" | "running" | "done" | "error")
- Color-coded badge

### 5. Styling

- Tailwind CSS with dark mode (class strategy)
- Color palette: emerald/teal primary (matches "Orchid" botanical theme)
- Monospace font for code/output sections
- Responsive: works on desktop, degrades gracefully on mobile

## Dev workflow

```bash
# Terminal 1: Rust backend
cargo run -- web

# Terminal 2: SvelteKit dev server
cd frontend && pnpm dev
```

SvelteKit dev server runs on port 5173, proxies API calls to localhost:3100.

## Future: Embed in binary

Eventually, build SvelteKit as static site and embed in orchid-web via `include_dir` or similar. For now, run separately in dev mode.

Add to `frontend/svelte.config.js`:
```javascript
import adapter from '@sveltejs/adapter-static';
// ... configure for static output to ../crates/orchid-web/static/
```

## Testing

- `cd frontend && pnpm check` — TypeScript check
- `cd frontend && pnpm lint` — ESLint
- Manual: verify each page loads, run an agent, see artifacts
- Future: Playwright e2e tests

## Constraints

- Svelte 5 runes syntax only (`$state`, `$derived`, `$effect`, `$props`) — no Svelte 4 stores
- TypeScript strict mode
- pnpm only
- No SSR needed — this is a local dashboard. Use `adapter-static`.
- API_BASE should be configurable via env var for production builds
