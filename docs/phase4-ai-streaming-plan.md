# Phase 4 AI Streaming Plan

## Goal
AI CLI 실행의 체감 UX를 개선하기 위해 출력 스트리밍, 실행 타임아웃, 블록 단위 복사 기능을 추가한다.

## Scope
- AI stdout/stderr line streaming
- Config-based AI timeout
- Copy button for command/AI blocks

## Acceptance
- `cargo check` 통과
- `cargo test` 통과
- AI 실행 중 출력이 순차적으로 블록에 반영
- timeout 초과 시 failed 상태 반영
