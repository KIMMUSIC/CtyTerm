# Phase 4 Links Plan (File/Line Detection)

## Goal
터미널/AI 출력에서 `file:line[:column]` 형태를 감지해 빠르게 편집기로 이동할 수 있는 흐름을 제공한다.

## Scope
- 출력 라인에서 파일/라인 레퍼런스 파싱
- 워크스페이스 기준 상대 경로 해석
- 클릭 시 `code -g` 명령 자동 채움
- 파서 단위 테스트

## Acceptance
- `cargo check` 통과
- `cargo test` 통과
- 출력 라인에서 파일/라인 감지 시 액션 버튼 노출
