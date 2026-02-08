# Phase 3 Plan (AI Integration)

## Goal
Claude Code/Codex CLI를 내부 AI 패널에서 실행하고, 선택한 블록 컨텍스트를 자동으로 전달하며, 결과를 AI 블록으로 렌더링한다.

## Scope
- AI panel (tool/prompt/run)
- Claude Code + Codex CLI execution pipeline
- block context passing
- AI block rendering in timeline

## Functional Targets
1. AI Panel
   - 툴 선택(Claude Code / Codex CLI)
   - 프롬프트 입력 및 실행
2. CLI Integration
   - 백그라운드 스레드에서 외부 CLI 실행
   - stdout/stderr 수집
   - 종료 코드/소요 시간 상태 반영
3. Context Passing
   - 커맨드 블록 단위 컨텍스트 선택
   - 실행 시 선택 블록 내용 자동 첨부
4. AI Block Rendering
   - AI 블록 상태(running/completed/failed) 표시
   - 프롬프트/출력/컨텍스트 개수 렌더링

## Acceptance
- `cargo check` 통과
- `cargo test` 통과
- UI에서 AI run 요청 시 AI 블록 생성/업데이트/완료 상태 반영
