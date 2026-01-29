---
id: 2026-01-29-task-x6lf
step: 01-plan
status: approved
updated_at: '2026-01-29T06:03:04.848Z'
---

# Plan: 파일 내용 새로고침 버그 수정

## Summary

파일을 다시 열거나 이미 열린 창을 포커스할 때 파일 내용이 새로고침되지 않는 버그를 수정합니다. 근본 원인은 Rust 백엔드가 기존 창을 재사용할 때 프론트엔드에 파일 다시 로드를 알리지 않기 때문입니다. Tauri 이벤트를 통해 창이 포커스될 때 파일 내용을 새로 로드하도록 구현합니다.

## Key Patterns Found

- `src-tauri/src/lib.rs:64-70` — 동일 파일 경로의 창이 이미 열려있으면 `show()`와 `set_focus()`만 호출하고 반환
- `src/main.tsx:78-108` — `loadFile()` 함수가 파일을 읽고 에디터 상태를 업데이트
- `src/main.tsx:223-230` — URL 쿼리 파라미터에서 파일 경로를 읽어 초기 로드 (창 생성 시 1회만 호출)

## Approach

창이 포커스될 때 Rust 백엔드에서 프론트엔드로 이벤트를 emit하여 파일을 다시 로드하도록 합니다. 이 방식은 기존 창 재사용 로직을 유지하면서 파일 내용 동기화 문제를 해결합니다. 대안으로 파일 시스템 감시(watch) 기능이 있지만 추가 의존성이 필요하고 더 복잡합니다.

**Test strategy:** manual QA - 앱을 실행하여 같은 파일을 다시 열었을 때 최신 내용이 표시되는지 확인

## Tasks

### Task 1: Emit reload event in Rust backend

- **File:** `src-tauri/src/lib.rs` (Modify)
- **What:** 기존 창을 재사용할 때 `show()`/`set_focus()` 후에 `reload-file` 이벤트를 해당 창에 emit
- **Pattern:** `src-tauri/src/lib.rs:64-70` — 기존 창 재사용 분기
- **Verify:** `cargo build` → 빌드 성공
- **Depends:** None

---

### Task 2: Listen for reload event in frontend

- **File:** `src/main.tsx` (Modify)
- **What:** `reload-file` 이벤트 리스너 추가하여 이벤트 수신 시 `loadFile()`을 호출하도록 구현
- **Pattern:** `src/main.tsx:232-252` — 기존 창 이벤트 리스너 패턴
- **Verify:** `npm run build` → 빌드 성공
- **Depends:** Task 1

---

## Files to Modify

| File | Action | Description |
|------|--------|-------------|
| `src-tauri/src/lib.rs` | Modify | 기존 창 재사용 시 reload-file 이벤트 emit 추가 |
| `src/main.tsx` | Modify | reload-file 이벤트 리스너 추가하여 loadFile() 호출 |

## Acceptance Criteria

- [ ] 같은 파일을 다시 열면 (Cmd+O 또는 파일 더블클릭) 최신 내용이 표시됨
- [ ] 외부에서 파일을 수정한 후 앱에서 같은 파일을 다시 열면 변경된 내용이 반영됨
- [ ] 파일이 이미 열려있는 창으로 포커스되었을 때도 최신 내용이 로드됨
- [ ] Markdown 파일과 코드 파일 모두 정상 동작

## Risks

- **에디터에 저장하지 않은 변경사항이 있을 때 덮어씌워질 수 있음**: 현재는 저장하지 않은 변경사항 감지 기능이 없으므로 이 이슈는 별도 작업으로 처리 필요
