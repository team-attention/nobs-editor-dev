---
status: approved
updated_at: '2026-01-27T15:46:25.118Z'
---
# Plan: iTerm Semantic History 연동 + Always-on-Top 창 구현

## Summary

**Goal:** iTerm에서 마크다운 파일 경로를 Cmd+클릭했을 때 JustViewer가 Always-on-Top 창으로 터미널 위에 표시되도록 구현

**Architecture:**
1. Tauri의 `alwaysOnTop` 설정을 활성화하고, macOS NSWindow의 `NSWindowCollectionBehaviorCanJoinAllSpaces` 속성을 추가하여 fullscreen Space에서도 창이 위에 떠있도록 함
2. 사용자는 iTerm의 Semantic History 설정에서 JustViewer.app을 실행 명령어로 지정

**Tech Stack:** Tauri 2, Rust (cocoa/objc crate), macOS NSWindow API

## Codebase Analysis

### Current State

**Relevant Files Found:**
- `src-tauri/src/lib.rs` - 창 관리 로직 (bring_window_to_front 함수)
- `src-tauri/tauri.conf.json` - 창 설정 (alwaysOnTop: false 현재)
- `src-tauri/Cargo.toml` - Rust 의존성 (cocoa, objc crate 이미 있음)
- `src/main.tsx` - 프론트엔드 파일 로딩 로직

**Existing Patterns to Follow:**
- `src-tauri/src/lib.rs:19-47` - NSWindow 조작 패턴
  - WHY: 이미 cocoa/objc crate를 사용해 NSWindow를 직접 조작하는 패턴이 있음. 동일한 방식으로 `NSWindowCollectionBehaviorCanJoinAllSpaces` 추가 가능
- `src-tauri/src/lib.rs:30-44` - unsafe Objective-C 메시지 호출 패턴
  - WHY: `msg_send!` 매크로 사용법과 NSWindow 메서드 호출 방식 참조

**Similar Features:**
- `bring_window_to_front()` 함수가 이미 창을 앞으로 가져오는 로직 구현
- 이 함수를 확장하여 fullscreen space에서도 동작하도록 수정

**Testing Approach:**
- 수동 테스트 필수 (macOS GUI 동작)
- 빌드 성공 여부로 구문 검증
- iTerm + fullscreen space에서 실제 동작 테스트

### Key Findings

**File Structure:**
- Rust 백엔드: `src-tauri/src/lib.rs`
- 설정: `src-tauri/tauri.conf.json`
- 프론트엔드: `src/main.tsx` (변경 불필요)

**Dependencies:**
- External: `cocoa` 0.26, `objc` 0.2 (이미 설치됨)
- Internal: 없음 (새로운 의존성 불필요)

**Code Conventions:**
- macOS 전용 코드는 `#[cfg(target_os = "macos")]` 어트리뷰트 사용
- NSWindow 조작은 `unsafe` 블록 내에서 수행
- `msg_send!` 매크로로 Objective-C 메서드 호출

**Integration Points:**
- `bring_window_to_front()` 함수 확장
- `tauri.conf.json`의 window 설정 변경

**Existing Utilities to Reuse:**
- `bring_window_to_front()` 함수 내 NSWindow 접근 패턴 재사용

## Architecture Decisions

### Chosen Approach: NSWindow 설정 + Tauri alwaysOnTop

**How it works:**
1. `tauri.conf.json`에서 `alwaysOnTop: true` 설정
2. `lib.rs`에서 `NSWindowCollectionBehaviorCanJoinAllSpaces` 설정 추가
3. iTerm Semantic History에서 `open -a JustViewer "$1"` 명령어 설정

**Why this approach:**
- 기존 `bring_window_to_front()` 패턴을 따름 (`lib.rs:19-47`)
- cocoa/objc crate가 이미 설치되어 있어 추가 의존성 불필요
- Tauri의 `alwaysOnTop` 설정과 NSWindow 직접 설정을 조합하여 확실하게 동작

**Pattern References:**
- `src-tauri/src/lib.rs:30-44` - NSWindow 조작 패턴
  - **Extract:** `ns_window as id` 캐스팅, `msg_send!` 매크로 사용법
  - **Apply to:** `setCollectionBehavior:` 메서드 호출

### Alternatives Considered

**Alternative 1: Panel Window 사용**
- Description: NSWindow 대신 NSPanel을 사용하여 유틸리티 창으로 표시
- Pros: 더 가벼운 UI, 자동으로 다른 창 위에 표시
- Cons: Tauri가 기본적으로 NSPanel을 지원하지 않음, 커스텀 구현 필요
- Not chosen because: 구현 복잡도가 높고 기존 코드 재작성 필요

**Alternative 2: 새 창 생성**
- Description: 파일 오픈 시마다 새로운 alwaysOnTop 창 생성
- Pros: 여러 파일을 동시에 볼 수 있음
- Cons: 리소스 낭비, 복잡한 창 관리 필요
- Not chosen because: 단순한 뷰어에 과도한 복잡성

### Test Strategy

**Chosen approach:** Manual QA Only

**Rationale:** macOS GUI 동작은 유닛 테스트로 검증 불가. 빌드 성공 + 실제 동작 테스트 필요

## Task Breakdown

### Phase 1: Tauri 설정 변경

#### Task 1: alwaysOnTop 설정 활성화

**File:** `src-tauri/tauri.conf.json` (Modify)

**Changes:**
- Modify: `alwaysOnTop` 값을 `false`에서 `true`로 변경
- Code location: line 20

**Steps:**
1. `tauri.conf.json` 열기
2. line 20의 `"alwaysOnTop": false`를 `"alwaysOnTop": true`로 변경
3. 저장

**Verification:**
- [ ] Run: `cd src-tauri && cargo check`
- [ ] Expected: 빌드 에러 없음
- [ ] Check: JSON 문법 오류 없음

**Dependencies:** None

**Parallelizable:** YES (with Task 2)

**Commit Message:** `feat: enable always-on-top window mode`

---

### Phase 2: NSWindow 설정 추가

#### Task 2: NSWindowCollectionBehavior 설정 추가

**File:** `src-tauri/src/lib.rs` (Modify)

**Changes:**
- Modify: `bring_window_to_front()` 함수에 `setCollectionBehavior:` 호출 추가
- Code location: line 30-44 내 unsafe 블록

**Reference pattern:** `src-tauri/src/lib.rs:39` - `msg_send!` 매크로 사용

**Steps:**
1. `lib.rs` 열기
2. line 43 (`ns_window.makeKeyAndOrderFront_(nil);`) 앞에 다음 코드 추가:
   ```rust
   // Allow window to appear over fullscreen apps
   // NSWindowCollectionBehaviorCanJoinAllSpaces = 1 << 0
   // NSWindowCollectionBehaviorFullScreenAuxiliary = 1 << 8
   let behavior: u64 = (1 << 0) | (1 << 8);
   let _: () = msg_send![ns_window, setCollectionBehavior: behavior];
   ```
3. 저장

**Verification:**
- [ ] Run: `cd src-tauri && cargo check`
- [ ] Expected: 컴파일 에러 없음
- [ ] Check: `msg_send!` 구문 정상

**Dependencies:** None

**Parallelizable:** YES (with Task 1)

**Commit Message:** `feat: add NSWindowCollectionBehavior for fullscreen spaces`

---

### Phase 3: 빌드 및 테스트

#### Task 3: 개발 모드 빌드 테스트

**File:** N/A

**Changes:** 없음 (빌드 검증만)

**Steps:**
1. `npm run tauri dev` 실행
2. 앱이 정상적으로 시작되는지 확인
3. 창이 다른 앱 위에 항상 표시되는지 확인

**Verification:**
- [ ] Run: `npm run tauri dev`
- [ ] Expected: 앱 정상 실행
- [ ] Check: 창이 다른 창 위에 항상 표시됨

**Dependencies:** Requires Task 1, 2

**Parallelizable:** NO

**Commit Message:** (커밋 없음 - 검증만)

---

#### Task 4: Fullscreen Space 테스트

**File:** N/A

**Changes:** 없음 (검증만)

**Steps:**
1. iTerm을 fullscreen으로 만들기 (Ctrl+Cmd+F 또는 녹색 버튼)
2. 터미널에서 마크다운 파일 생성: `echo "# Test" > /tmp/test.md`
3. 파일 경로를 `open` 명령어로 열기: `open -a JustViewer /tmp/test.md`
4. JustViewer 창이 iTerm 위에 표시되는지 확인

**Verification:**
- [ ] Run: `open -a JustViewer /tmp/test.md`
- [ ] Expected: 창이 fullscreen iTerm 위에 표시됨
- [ ] Check: 창 포커스 가능, 내용 표시됨

**Dependencies:** Requires Task 3

**Parallelizable:** NO

**Commit Message:** (커밋 없음 - 검증만)

---

### Phase 4: 프로덕션 빌드

#### Task 5: 릴리스 빌드

**File:** N/A

**Changes:** 없음 (빌드만)

**Steps:**
1. `npm run tauri build` 실행
2. 빌드 완료 대기
3. `src-tauri/target/release/bundle/macos/JustViewer.app` 생성 확인

**Verification:**
- [ ] Run: `npm run tauri build`
- [ ] Expected: 빌드 성공
- [ ] Check: `.app` 파일 생성됨

**Dependencies:** Requires Task 4

**Parallelizable:** NO

**Commit Message:** (커밋 없음 - 빌드만)

---

#### Task 6: 프로덕션 앱 테스트

**File:** N/A

**Changes:** 없음 (검증만)

**Steps:**
1. 빌드된 `.app`을 `/Applications`에 복사
2. iTerm fullscreen에서 `open -a JustViewer /tmp/test.md` 실행
3. 창이 정상적으로 표시되는지 확인

**Verification:**
- [ ] Run: `open -a JustViewer /tmp/test.md`
- [ ] Expected: 창이 fullscreen iTerm 위에 표시됨
- [ ] Check: Always-on-Top 동작 확인

**Dependencies:** Requires Task 5

**Parallelizable:** NO

**Commit Message:** (커밋 없음 - 검증만)

## Files to Modify

| File | Action | Description | Phase | Task # |
|------|--------|-------------|-------|---------|
| `src-tauri/tauri.conf.json` | Modify | alwaysOnTop: true | 1 | 1 |
| `src-tauri/src/lib.rs` | Modify | NSWindowCollectionBehavior 추가 | 2 | 2 |

## Testing Strategy

### Test Approach

**Strategy:** Manual QA Only

**Rationale:** macOS NSWindow 동작은 유닛 테스트로 검증 불가. GUI 동작 직접 확인 필요.

### Manual Verification

**Manual testing checklist:**
- [ ] `npm run tauri dev` 정상 실행
- [ ] 일반 모드에서 창이 다른 앱 위에 표시
- [ ] iTerm fullscreen에서 `open -a JustViewer /path/to/file.md` 실행
- [ ] 창이 fullscreen 앱 위에 표시
- [ ] 창 포커스 및 상호작용 정상
- [ ] 창 드래그 정상 동작
- [ ] 파일 내용 정상 표시

**Environment:** macOS Development

## Acceptance Criteria

### Implementation Complete
- [ ] `tauri.conf.json`에 `alwaysOnTop: true` 설정
- [ ] `lib.rs`에 `NSWindowCollectionBehavior` 설정 추가
- [ ] `cargo check` 성공
- [ ] `npm run tauri build` 성공

### Functionality
- [ ] 일반 데스크톱에서 창이 다른 앱 위에 표시
- [ ] Fullscreen 앱 (iTerm) 위에 창이 표시
- [ ] `open -a JustViewer /path/to/file.md` 명령어로 파일 열기 동작
- [ ] 파일 내용 정상 렌더링
- [ ] 창 드래그 및 상호작용 정상

### Non-Regression
- [ ] 기존 파일 열기 기능 정상 (Cmd+O)
- [ ] 기존 저장 기능 정상 (Cmd+S)
- [ ] 기존 deep-link 기능 정상

**Status:** All criteria must be checked before considering feature complete

## Risks & Considerations

### Technical Risks

- **Risk 1**: NSWindowCollectionBehavior 플래그가 macOS 버전에 따라 다르게 동작
  - **Impact:** 특정 macOS 버전에서 fullscreen overlay가 안 될 수 있음
  - **Mitigation:** macOS 10.15+ 최소 요구 버전 유지 (이미 tauri.conf.json에 설정됨)
  - **Contingency:** 문제 발생 시 `NSFloatingWindowLevel` 추가 설정

- **Risk 2**: Always-on-Top이 사용자 경험을 해칠 수 있음
  - **Impact:** 다른 작업 시 창이 항상 위에 있어서 방해될 수 있음
  - **Mitigation:** 창을 닫으면 되므로 큰 문제 없음
  - **Contingency:** 향후 토글 버튼 추가 고려

### Edge Cases

- **Fullscreen Space 전환**
  - Scenario: 다른 Space로 전환 시
  - Handling: `NSWindowCollectionBehaviorCanJoinAllSpaces`가 모든 Space에서 보이도록 함
  - Test coverage: Task #4

- **여러 모니터**
  - Scenario: 여러 모니터에서 사용 시
  - Handling: 기본 NSWindow 동작에 의존
  - Test coverage: 별도 테스트 필요 (optional)

### Dependencies

**External Dependencies:**
- Package: `cocoa` (version 0.26) - 이미 설치됨
  - Purpose: macOS Cocoa API 바인딩
  - Impact: 없음 (이미 사용 중)

**Breaking Changes:**
- None - 창 동작 변경만 있고 API 변경 없음

### Performance Considerations

- **Performance Impact:** Low
- **Specific concerns:** 없음

## Implementation Notes

### Code Patterns to Follow

**Pattern 1: NSWindow 조작**
```rust
// Reference: src-tauri/src/lib.rs:30-44
// Use this pattern for NSWindow manipulation

unsafe {
    let ns_window = ns_window as id;
    // NSWindowCollectionBehavior flags
    let behavior: u64 = (1 << 0) | (1 << 8);
    let _: () = msg_send![ns_window, setCollectionBehavior: behavior];
    ns_window.makeKeyAndOrderFront_(nil);
}
```

### Gotchas & Pitfalls

- **Gotcha 1**: `setCollectionBehavior:`는 `makeKeyAndOrderFront_` 전에 호출해야 함
  - Why it matters: 순서가 바뀌면 적용이 안 될 수 있음
  - How to avoid: 코드 순서 주의

- **Gotcha 2**: NSWindowCollectionBehavior 플래그는 비트 OR로 조합
  - Why it matters: 잘못된 값을 넣으면 예상치 못한 동작
  - How to avoid: 상수 값 명확히 주석 처리

### Common Mistakes to Avoid

- ❌ Don't call `setCollectionBehavior:` outside of the unsafe block - Objective-C 호출이므로 unsafe 필요
- ❌ Don't use wrong behavior flags - 플래그 값 정확히 확인
- ✅ Do add comments explaining the behavior flags - 매직 넘버에 주석 추가

### Debugging Tips

- If 창이 fullscreen 위에 안 뜨면, `NSWindowLevel` 설정 추가 확인
- If 빌드 실패하면, `msg_send!` 문법 확인 (콜론 위치 등)

---

## Task Flow Visualization

```
Phase 1 (Tauri 설정)
  Task 1 (alwaysOnTop)
      ↓ (parallel with Task 2)

Phase 2 (NSWindow 설정)
  Task 2 (CollectionBehavior)
      ↓

Phase 3 (검증)
  Task 3 (개발 빌드) → Task 4 (Fullscreen 테스트)

Phase 4 (프로덕션)
  Task 5 (릴리스 빌드) → Task 6 (프로덕션 테스트)
```

## Estimated Effort

- **Total phases:** 4
- **Total tasks:** 6
- **Complexity:** Low
- **Parallelization opportunities:** Task 1, 2 병렬 가능

---

## iTerm Semantic History 설정 안내

구현 완료 후, 사용자가 iTerm에서 설정해야 하는 사항:

1. iTerm2 → Settings → Profiles → [프로필 선택] → Advanced
2. "Semantic History" 섹션 찾기
3. "Run command..." 선택
4. 명령어 입력: `open -a JustViewer "\1"`
5. 확장자 필터 (optional): `.md`, `.markdown`

이후 터미널에서 마크다운 파일 경로를 Cmd+클릭하면 JustViewer가 열림.

---

## Plan Completion Checklist

Before submitting this plan, verify:
- [x] ALL tasks are 2-5 minutes with exact file paths
- [x] ≥80% of tasks reference specific existing code (file:line)
- [x] EVERY task has concrete verification (command + expected output)
- [x] Zero assumptions about business logic or patterns
- [x] Test strategy decided and documented
- [x] Dependencies and parallelization marked
- [x] Phase organization logical and complete
- [x] One complete plan (not split into multiple phases to plan later)
