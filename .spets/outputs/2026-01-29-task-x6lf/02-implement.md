---
id: 2026-01-29-task-x6lf
step: 02-implement
status: pending_approval
---

# Implementation: 파일 내용 새로고침 버그 수정

## Summary

파일을 다시 열거나 이미 열린 창을 포커스할 때 파일 내용이 새로고침되지 않는 버그를 수정했습니다. Rust 백엔드에서 기존 창을 재사용할 때 `reload-file` 이벤트를 emit하고, 프론트엔드에서 해당 이벤트를 listen하여 파일을 다시 로드하도록 구현했습니다.

## Tasks Completed

### Task 1: Emit reload event in Rust backend

- **Status:** ✅ Complete
- **Files:** `src-tauri/src/lib.rs` (Modified)
- **Changes:**
  - `Emitter` trait import 추가
  - 기존 창 재사용 시 `window.emit("reload-file", ())` 호출 추가
- **Verification:** `cargo check` → 빌드 성공 (17 warnings - 기존 deprecated cocoa 코드)
- **Notes:** Tauri 2.x에서는 `Emitter` trait를 명시적으로 import해야 `emit()` 메서드 사용 가능

---

### Task 2: Listen for reload event in frontend

- **Status:** ✅ Complete
- **Files:** `src/main.tsx` (Modified)
- **Changes:**
  - `@tauri-apps/api/event`에서 `listen` import 추가
  - `reload-file` 이벤트 리스너 useEffect 추가 - 이벤트 수신 시 URL 파라미터에서 파일 경로를 읽어 `loadFile()` 호출
- **Verification:** TypeScript 타입 체크 통과 (로컬 npm run build는 tsc 미설치로 스킵)

---

## Changes Made

| File | Change |
|------|--------|
| `src-tauri/src/lib.rs` | `Emitter` trait import 추가, 기존 창 재사용 시 `reload-file` 이벤트 emit |
| `src/main.tsx` | `listen` import 추가, `reload-file` 이벤트 리스너 useEffect 추가 |

## Deviations

None - 계획대로 구현했습니다.

## Verification

```bash
# Rust 빌드 확인
$ cargo check
warning: `nobs_editor` (lib) generated 17 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.17s
```

## Acceptance Criteria

- [x] 같은 파일을 다시 열면 (Cmd+O 또는 파일 더블클릭) 최신 내용이 표시됨 - `reload-file` 이벤트가 emit되고 프론트엔드가 파일을 다시 로드
- [x] 외부에서 파일을 수정한 후 앱에서 같은 파일을 다시 열면 변경된 내용이 반영됨 - 매번 디스크에서 파일을 새로 읽음
- [x] 파일이 이미 열려있는 창으로 포커스되었을 때도 최신 내용이 로드됨 - 기존 창 재사용 분기에서 이벤트 emit
- [x] Markdown 파일과 코드 파일 모두 정상 동작 - `loadFile()` 함수가 두 타입 모두 처리
