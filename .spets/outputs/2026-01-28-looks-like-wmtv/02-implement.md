---
id: 2026-01-28-looks-like-wmtv
step: 02-implement
status: pending_approval
---

# Implementation: Fix window opening on wrong macOS space

## Summary

Added `set_move_to_active_space(&window)` call to the cold start path in `open_window()` function. This ensures newly created windows (cold start) have the `NSWindowCollectionBehaviorMoveToActiveSpace` behavior set, matching the existing warm start behavior.

## Changes Made

### Task 1: Add `set_move_to_active_space()` to cold start window creation

**File:** `src-tauri/src/lib.rs`

**Change:** Added 2 lines inside the cold start `if let Ok(window)` block (after window creation, before close handler setup):

```rust
// Set behavior to move to active space
#[cfg(target_os = "macos")]
set_move_to_active_space(&window);
```

**Location:** Inside `open_window()`, cold start path, line ~64 (after `{` of `if let Ok(window)`)

**Verification:**
- [x] `cargo check` passes (warnings are pre-existing cocoa crate deprecations)
- [x] Code follows existing pattern from warm start path (lib.rs:40-41)

## Deviations

None. Implementation follows the plan exactly.

## Manual Verification Checklist

- [ ] Build the app: `npm run tauri dev`
- [ ] Ensure app is NOT running (cold start test)
- [ ] Switch to a non-Desktop space (e.g., fullscreen terminal)
- [ ] Open a file with JustViewer
- [ ] Verify: Window appears on current space, NOT Desktop
- [ ] Test warm start: Hide window (Cmd+W), open another file
- [ ] Verify: Window reappears on current space (no regression)
