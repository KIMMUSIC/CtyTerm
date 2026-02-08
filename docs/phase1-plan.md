# Phase 1 Plan (MVP)

## Goal
Phase 0 기반 위에서 실제 생산성 기능을 갖춘 MVP 상호작용을 만든다.

## Scope
- Tab system 강화
- Pane splitting (single / vertical / horizontal)
- Block-first UI 렌더링
- Command palette + history search
- Sidebar 확장 (folder tree / file search / git indicator)

## Functional Targets
1. Tabs
   - 기본 탭 전환 가능
   - 새 탭 추가 가능 (세션 라벨 기반)
2. Panes
   - 레이아웃 전환: single, vertical split, horizontal split
   - 활성 pane 표시
3. Block UI
   - command/output 메타를 카드 단위로 렌더링
   - 최신 pending line 표시
4. Palette
   - 단축키/버튼으로 열기
   - 히스토리 기반 검색 및 선택 시 입력창 반영
5. Sidebar
   - 루트 기준 폴더/파일 목록
   - 파일 검색 필터
   - `.git` 존재 여부 표시

## Out Of Scope
- 완전한 멀티-PTY pane 세션 분리
- 고급 VT 커서 제어/색상 렌더링
- session restore와 persistent workspace state

## Architecture Delta
- `ux_model`에 UI 소비용 읽기 API를 추가한다.
- `ui`는 레이아웃 상태 머신(`PaneLayout`)을 중심으로 중앙 렌더링을 분기한다.
- Block 렌더링은 우선 egui 기반 카드로 구현하고 추후 `render`의 custom wgpu path로 치환한다.

## Acceptance
- `cargo check` 통과
- UI에서 split/palette/sidebar 기능이 동작
- 블록 카드 단위로 출력 표현
