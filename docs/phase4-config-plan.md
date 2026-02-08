# Phase 4 Config Plan (TOML + Hot Reload)

## Goal
고정 상수로 박혀 있는 실행 옵션을 TOML 설정으로 분리하고, 앱 실행 중 설정 변경을 감지해 자동 반영한다.

## Scope
- `config/config.toml` 도입
- AI CLI 템플릿 설정화 (Codex/Claude)
- Session autosave 주기/파일 경로 설정화
- 런타임 hot reload (mtime polling)

## Functional Targets
1. Config bootstrap
   - 설정 파일이 없으면 기본 파일 생성
   - 앱 시작 시 설정 로드
2. AI command template
   - `{prompt}` placeholder 지원
   - 툴별 command/args 설정 가능
3. Session settings
   - autosave interval (sec)
   - session file path
4. Hot reload
   - 설정 파일 수정 시 재로딩
   - 재로딩 성공/실패 상태 표시

## Acceptance
- `cargo check` 통과
- `cargo test` 통과
- `config/config.toml` 생성 확인
- 설정 수정 후 재로딩 반영 확인
