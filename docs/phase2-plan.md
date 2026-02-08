# Phase 2 Plan (Productivity Features)

## Goal
블록 기반 워크플로우를 실제 생산성 수준으로 끌어올리기 위해 검색, 북마크, 내보내기 기능을 추가한다.

## Scope
- Block search
- Block bookmarks
- Block export (Markdown)
- Sidebar 확장 유지/개선

## Functional Targets
1. Block Search
   - 검색어로 command/output을 필터링한다.
   - 검색 결과 개수를 상태로 표시한다.
2. Bookmarks
   - 각 블록을 북마크 토글할 수 있다.
   - 북마크만 보기 필터를 제공한다.
3. Export
   - 현재 세션 블록을 Markdown으로 내보낸다.
   - 전체/북마크 전용 내보내기를 지원한다.
4. Sidebar
   - 기존 폴더/파일 검색 기능 유지
   - git 저장소 존재 상태를 계속 노출

## Architecture Delta
- `ux_model`
  - 블록 북마크 상태와 검색/내보내기 API 추가
  - Markdown export formatter 추가
- `ui`
  - block search bar + bookmark filter + export action 추가
  - block 카드에 bookmark 토글 컨트롤 추가

## Acceptance
- `cargo check` 통과
- `cargo test` 통과
- UI에서 search/bookmark/export 동작
