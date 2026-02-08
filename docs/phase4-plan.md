# Phase 4 Plan (Polish and Stability)

## Goal
세션 복원과 안정성 강화를 통해 일상 사용 시 상태 유실을 줄이고, 장시간 실행 시 동작을 안정화한다.

## Scope (Current Iteration)
- Session restore using TOML snapshot
- Periodic autosave while app runs
- Final save on app shutdown

## Functional Targets
1. Startup Restore
   - `state/session.toml`이 있으면 복원
   - 없으면 신규 세션으로 시작
2. Autosave
   - 실행 중 주기적으로 세션 저장
3. Shutdown Save
   - 앱 종료 시 세션 최종 저장

## Out Of Scope (Next Iteration)
- 렌더링 파이프라인 성능 튜닝
- VT 파서 고급 동작 최적화
- 크래시 복구/로그 체계 고도화

## Acceptance
- `cargo check` 통과
- `cargo test` 통과
- `state/session.toml` 생성 및 재시작 시 복원 동작
