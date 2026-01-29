---
id: 2026-01-29-windows-ar-72k3
step: 02-implement
status: approved
updated_at: '2026-01-29T00:47:34.428Z'
---

# Implementation: Fix window space-following behavior

## Summary

Removed the `NSWindowCollectionBehaviorCanJoinAllSpaces` flag from the window collection behavior in `lib.rs`. Windows will now stay on their original macOS Space instead of following the user across Spaces. The `FullScreenAuxiliary` flag is preserved to maintain the ability to appear on fullscreen spaces.

## Tasks Completed

### Task 1: Remove CanJoinAllSpaces flag in lib.rs

- **Status:** ✅ Complete
- **Files:** `src-tauri/src/lib.rs` (Modified)
- **Changes:**
  - Changed `let behavior: u64 = (1 << 0) | (1 << 8);` to `let behavior: u64 = 1 << 8;`
  - Updated comment to explain the change and why CanJoinAllSpaces was removed
- **Verification:** `cargo build --manifest-path src-tauri/Cargo.toml` → Finished `dev` profile in 9.04s (warnings are pre-existing)

---

## Changes Made

| File | Change |
|------|--------|
| `src-tauri/src/lib.rs` | Removed CanJoinAllSpaces (1<<0) from collection behavior, kept only FullScreenAuxiliary (1<<8) |

## Deviations

None - plan was followed exactly.

## Verification

```bash
$ cargo build --manifest-path src-tauri/Cargo.toml
   Compiling justviewer v0.3.1 (/Users/eatnug/Workspace/JustViewer/src-tauri)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 9.04s
```

## Acceptance Criteria

- [x] App compiles without errors: `cargo build --manifest-path src-tauri/Cargo.toml` → Build succeeded
- [ ] Windows stay on their original Space when user switches Spaces (requires manual testing)
- [ ] Windows still appear correctly on fullscreen Spaces (requires manual testing)
