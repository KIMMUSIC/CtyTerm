# Phase 0 Design (Technical Foundation)

## Objective
Build a Windows-first terminal foundation that proves the end-to-end pipeline:
- launch GPU UI shell (wgpu + egui),
- spawn PowerShell in ConPTY-backed PTY,
- parse VT output minimally,
- surface output into a terminal text grid in the center pane.

## Layered Modules
- `terminal_core`
  - `pty/`: PTY lifecycle, shell spawning, async output channel, input write path.
  - `vt_parser/`: minimal ANSI/VT sequence stripping and line reconstruction.
  - `grid/`: visible text grid clipping (width/height).
  - `scrollback/`: bounded line buffer utility.
- `ux_model`
  - `blocks/`: `CommandBlock` metadata model.
  - `history/`: command history ring-like list.
  - `search/`: simple query against command/output text.
  - `session/`: active blocks + pending line + visible-line projection.
- `render`
  - `font/`, `atlas/`: early renderer metadata structures.
  - `text_renderer/`: terminal grid rendering adapter used by UI.
- `ui`
  - IDE-like panel composition (tabs / sidebar / center / input / status).
  - PTY polling and state synchronization.

## Data Flow
1. `PtySession::spawn_powershell` opens PTY and launches `powershell.exe`.
2. Background reader thread sends raw bytes to a channel.
3. UI frame loop drains channel and feeds bytes into `MinimalVtParser`.
4. Parsed lines update `SessionState` blocks.
5. `SessionState::visible_lines` feeds `TextGrid`.
6. `render::text_renderer` draws the grid inside the center pane.

## Current Constraints
- VT parsing is intentionally minimal (CSI/OSC stripped, basic control chars).
- Rendering currently uses a dedicated render crate path but still leans on egui text primitives.
- Sidebar and palette are structural placeholders for later phases.
- Command block metadata is present; complete command lifecycle metrics are deferred.

## Next Phase 0 Expansion
- Replace egui text primitive path with a custom wgpu text pipeline in `render`.
- Improve VT handling (cursor movement, color attributes, partial line rewrites).
- Add pane splitting behavior and persistent tab/session state.
