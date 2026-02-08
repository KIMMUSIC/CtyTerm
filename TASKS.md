# MyTerminal-c Task Plan (Derived from PLAN.md)

## Scope
- Current focus: `Phase 0 - Technical Foundation`
- Platform: Windows only
- Stack: Rust + wgpu + egui + ConPTY

## Phase 0 Deliverables
- [x] Rust workspace created
- [x] Core module skeletons created
- [x] App launches a wgpu + egui window
- [x] PowerShell spawned through ConPTY-backed PTY layer
- [x] PTY stdout captured and surfaced to UI model
- [x] Minimal VT parser implemented
- [x] Basic text grid rendered in center terminal pane
- [x] Short architecture document added
- [x] Build verified with `cargo check`

## Task Breakdown
1. Bootstrap workspace and crates
   - [x] Create workspace `Cargo.toml`
   - [x] Create crates: `app`, `terminal_core`, `ux_model`, `render`, `ui`
2. Implement terminal core MVP
   - [x] Add PTY abstraction and Windows shell launcher
   - [x] Add background reader thread
   - [x] Add minimal VT-to-lines parser
3. Implement UX model MVP
   - [x] Add command block model
   - [x] Add terminal session state
4. Implement rendering/UI MVP
   - [x] Add app shell with egui layout (top/left/center/bottom/status)
   - [x] Add basic terminal text grid view backed by render module
5. Wire everything end-to-end
   - [x] Stream shell output into session/grid
   - [x] Show output in center pane
6. Documentation and validation
   - [x] Add `docs/phase0-design.md`
   - [x] Run `cargo check`
   - [x] Update this file with completion status

## Notes
- The custom high-performance GPU text pipeline will start as a minimal dedicated render module in Phase 0 and will be expanded in later phases.
- AI panel integration is explicitly deferred to Phase 3.
- Runtime note: `cargo run -p app` was executed and reached the GUI run loop (command timed out in automation because the app keeps running).
