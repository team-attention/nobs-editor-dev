---
id: 2026-01-28-looks-like-wmtv
step: 01-plan
status: approved
updated_at: '2026-01-28T13:23:17.140Z'
---

# Plan: Fix window opening on wrong macOS space (Desktop instead of current)

## Summary

**Goal:** Fix the bug where the JustViewer window opens on the Desktop space instead of the current active space (above the terminal).

**Architecture:** The `open_window()` function in `src-tauri/src/lib.rs` already calls `set_move_to_active_space()` for warm starts (existing hidden window), but the cold start path (new window creation) does not call it. Adding the same `set_move_to_active_space()` call after window creation in the cold start path will fix the issue.

**Tech Stack:** Rust, Tauri 2, cocoa crate (NSWindow APIs)

## Codebase Analysis

### Current State

**Relevant Files Found:**
- `src-tauri/src/lib.rs` - Main Rust backend with window management logic
- `src-tauri/Info.plist` - Sets `LSUIElement=true` (background app, no dock icon)
- `src-tauri/Cargo.toml` - Dependencies including `cocoa = 0.26`

**Existing Patterns to Follow:**
- `src-tauri/src/lib.rs:21-33` - `set_move_to_active_space()` function already exists
  - WHY: This is the exact function we need to also call on cold start
- `src-tauri/src/lib.rs:38-47` - Warm start path already calls `set_move_to_active_space()`
  - WHY: Shows the pattern to replicate in cold start path

### Key Findings

**The Bug:**
The `open_window()` function at `lib.rs:36-72` has two code paths:

1. **Warm start (line 38-47):** Window exists, calls `set_move_to_active_space(&window)`, then `show()` and `set_focus()` - **WORKS CORRECTLY**

2. **Cold start (line 49-71):** Creates new window with `WebviewWindowBuilder`, but **NEVER calls `set_move_to_active_space()`** - **BUG IS HERE**

Since the app is `LSUIElement` (background app), macOS doesn't automatically bring the window to the current space on cold start. The `MoveToActiveSpace` collection behavior must be explicitly set.

## Architecture Decisions

### Chosen Approach: Add `set_move_to_active_space()` call to cold start path

**How it works:**
1. After the window is created via `WebviewWindowBuilder::build()` at line 54-59
2. Call `set_move_to_active_space(&window)` on the newly created window
3. This sets `NSWindowCollectionBehaviorMoveToActiveSpace` so the window appears on the current space

**Why this approach:**
- Follows the exact same pattern already used for warm starts at `lib.rs:40-41`
- Minimal change (2 lines added)
- Uses existing `set_move_to_active_space()` function - no new code needed

### Test Strategy

**Chosen approach:** Manual QA Only

**Rationale:** This is a macOS-specific window behavior that requires visual verification on a real system with Spaces. No test infrastructure exists in this project, and the behavior can only be verified by opening a file from another space.

## Task Breakdown

### Phase 1: Bug Fix

#### Task 1: Add `set_move_to_active_space()` to cold start window creation in `lib.rs`

**File:** `src-tauri/src/lib.rs` (Modify)

**Changes:**
- Add `set_move_to_active_space(&window)` call inside the cold start `if let Ok(window)` block, after window is created but before `set_focus()`
- Code location: Inside the `if let Ok(window)` block starting at line 63, before `let _ = window.set_focus();` at line 70

**Steps:**
1. Open `src-tauri/src/lib.rs`
2. Find the cold start path in `open_window()` (the `if let Ok(window)` block at line 63)
3. Add the following lines after line 63 (after `{`) and before the close handler setup:
   ```rust
   #[cfg(target_os = "macos")]
   set_move_to_active_space(&window);
   ```
4. Build to verify compilation: `cd src-tauri && cargo check`

**Verification:**
- [ ] Run: `cd src-tauri && cargo check`
- [ ] Expected: Compilation succeeds with no errors
- [ ] Manual test: Build app, open file from a different space, verify window appears on current space (not Desktop)

**Dependencies:** None

**Commit Message:** `fix: set MoveToActiveSpace on cold start window creation`

## Files to Modify

| File | Action | Description | Phase | Task # |
|------|--------|-------------|-------|--------|
| `src-tauri/src/lib.rs` | Modify | Add `set_move_to_active_space()` to cold start path | 1 | 1 |

## Testing Strategy

### Manual Verification

**Manual testing checklist:**
- [ ] Build the app: `npm run tauri build` or `npm run tauri dev`
- [ ] Ensure the app is NOT running (cold start test)
- [ ] Switch to a space that is NOT Desktop (e.g., a fullscreen terminal)
- [ ] Open a file with JustViewer (e.g., double-click a .md file or use the custom URL scheme)
- [ ] Verify: Window appears on the current space, NOT on Desktop
- [ ] Test warm start: Hide the window (Cmd+W), open another file
- [ ] Verify: Window reappears on current space (existing behavior preserved)

## Acceptance Criteria

- [ ] `set_move_to_active_space()` is called for both warm and cold start paths
- [ ] `cargo check` passes
- [ ] Cold start: window opens on current space (not Desktop)
- [ ] Warm start: window still opens on current space (no regression)

## Risks & Considerations

### Technical Risks

- **Risk 1**: `set_move_to_active_space` may not work if called immediately after window creation (window might not have an NSWindow handle yet)
  - **Impact:** Window still opens on wrong space
  - **Mitigation:** The warm start path works with the same function, so it should work. If needed, add a small delay or call it after `set_focus()`.

### Edge Cases

- **First launch ever:** App has no prior window state. Should create window on current space.
- **Rapid successive opens:** Multiple files opened quickly during cold start. The 300ms sleep and 150ms event delay should handle this.

## Implementation Notes

### Code Pattern to Follow

```rust
// Reference: src-tauri/src/lib.rs:38-47 (warm start path)
// The cold start path should mirror this by also calling set_move_to_active_space
if let Ok(window) = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
    .title("JustViewer")
    .inner_size(800.0, 600.0)
    .resizable(true)
    .visible(true)
    .build()
{
    // ADD THIS: Set behavior to move to active space
    #[cfg(target_os = "macos")]
    set_move_to_active_space(&window);

    // ... existing close handler setup ...
    let _ = window.set_focus();
}
```

## Task Flow Visualization

```
Phase 1 (Bug Fix)
  Task 1: Add set_move_to_active_space to cold start → Manual verification
```

## Estimated Effort

- **Total phases:** 1
- **Total tasks:** 1
- **Complexity:** Low (2-line fix using existing function)
