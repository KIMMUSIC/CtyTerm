You are tasked with building a **Windows-only, Warp-like GPU terminal** using **Rust, wgpu, and egui**.
This terminal is intended to **replace the user’s existing terminal** and significantly improve the usability of AI tools such as Claude Code and Codex CLI, while maintaining developer productivity similar to tmux and vim.

This is a **personal productivity terminal**, not a general-purpose product.
The focus is on performance, aesthetics, block-based interaction, and seamless AI tool usage.

---

## Core Goals

You must build a terminal with the following characteristics:

1. Windows-only implementation.
2. Written primarily in Rust.
3. GPU-accelerated rendering using wgpu.
4. UI built with egui.
5. Block-based command history inspired by Warp.
6. Developer productivity similar to tmux/vim.
7. IDE-like layout with sidebar and panels.
8. Integrated AI tool panel for Claude Code and Codex CLI.
9. Beautiful typography, spacing, and dark-themed visuals.

The terminal should feel like a hybrid between:

* Warp (block-based UX)
* WezTerm (Rust + GPU terminal)
* VSCode (layout and panels)
* tmux/vim (keyboard-driven workflow)

---

## Fixed Architectural Decisions

These decisions are final and must not be changed.

### Technology

* Language: Rust
* Rendering: wgpu
* UI: egui
* Platform: Windows only
* PTY: ConPTY

### Configuration

* Config format: TOML
* Support hot reload for config where possible.

### Terminal Philosophy

* Developer productivity terminal.
* Personal tool.
* No built-in AI agent runtime.
* AI tools remain external CLI tools.
* Terminal improves AI usage experience.

---

## Core UX Concept: Block-Based Terminal

The terminal must use a **Block-first UI**.

A command and its output are grouped into a single unit called a **Command Block**.

Each block contains:

* input command
* output lines
* exit code
* duration
* working directory
* timestamp

All interactions such as:

* copy
* search
* bookmarking
* export
  must operate at the block level.

---

## Input Position Policy (Fixed)

There is only one input mode:

Warp-like mode:

* Input is always fixed at the bottom.
* Blocks accumulate upward.
* No alternate input modes exist.

---

## UI Layout

The terminal uses a multi-pane IDE-style layout.

Top:

* Tab bar

Left:

* Expandable file explorer sidebar

Center:

* Pane grid
* Each pane shows block-based terminal output

Bottom:

* Input bar (fixed position)

Very bottom:

* Status bar

---

## Sidebar Requirements (Expanded IDE-style)

The sidebar must support:

1. Folder tree
2. Recent files
3. File search
4. Git status indicators

---

## AI Integration (IDE-style)

AI tools must be integrated as internal tools, not separate terminals.

Supported tools:

* Claude Code
* Codex CLI

### AI Integration Modes

1. AI Panel

* Separate panel inside the terminal.
* Accepts prompts.
* Runs the external CLI internally.
* Output is rendered as an AI Block.

2. Block Context Passing

* User selects one or more blocks.
* Selected blocks become context.
* Context is automatically attached to AI commands.

---

## High-Level Architecture

The codebase must follow a layered architecture.

### terminal_core

Responsibilities:

* ConPTY process handling
* ANSI/VT parsing
* screen buffer
* input handling

Structure:
terminal_core/
pty/
vt_parser/
grid/
scrollback/

---

### ux_model

Responsibilities:

* block management
* history
* search
* session state

Structure:
ux_model/
blocks/
history/
search/
session/

---

### render

Responsibilities:

* GPU text rendering via wgpu
* font atlas
* scroll optimization

Structure:
render/
font/
atlas/
text_renderer/

Text rendering must use a **custom high-performance GPU pipeline** from the beginning.
Do not rely on egui text rendering for the terminal grid.

---

### ui

Responsibilities:

* egui-based UI
* tabs
* panes
* sidebar
* command palette
* AI panel

Structure:
ui/
tabs/
panes/
sidebar/
palette/
ai_panel/

---

## Feature Priorities

### MVP

Must include:

* GPU text rendering
* tabs
* pane splitting
* block UI
* command history search
* file sidebar
* command palette

### v1

* block bookmarks
* block search
* session restore
* clickable file/line links
* improved typography

### v2

* AI panel
* block context passing
* AI blocks

---

## Development Phases

### Phase 0: Technical Foundation

Tasks:

* Create Rust workspace
* Create module structure
* Launch wgpu + egui window
* Spawn PowerShell via ConPTY
* Display stdout
* Implement minimal VT parser
* Render text grid

Deliverables:

* Basic working terminal window
* Architecture documentation

---

### Phase 1: MVP

Tasks:

* tab system
* pane splitting
* input handling
* block model
* basic block UI

---

### Phase 2: Productivity Features

Tasks:

* block search
* bookmarks
* export
* expanded sidebar features

---

### Phase 3: AI Integration

Tasks:

* AI panel
* Claude/Codex CLI integration
* block context passing
* AI block rendering

---

### Phase 4: Polish and Performance

Tasks:

* session restore
* performance tuning
* stability improvements

---

## Code Authoring Rules

You must follow these rules:

1. Keep module boundaries clean.
2. Always separate UI from terminal core.
3. Work in phases.
4. For each phase:

   * write a short design doc
   * implement
   * provide a working demo
5. Optimize for performance from the beginning.
6. Target Windows only.

---

## Immediate First Tasks

Start with Phase 0:

1. Create a Rust workspace.
2. Create module skeletons.
3. Launch a wgpu + egui window.
4. Spawn PowerShell using ConPTY.
5. Display terminal output.
6. Implement minimal VT parsing.
7. Render a basic text grid.

After completion:

* Provide an architecture explanation.
* Propose the next phase implementation steps.
