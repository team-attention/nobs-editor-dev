---
id: 2026-01-29-windows-ar-72k3
step: 01-plan
status: approved
updated_at: '2026-01-29T00:46:23.391Z'
---

# Plan: Fix window space-following behavior

## Summary

Windows created by JustViewer currently follow the user across macOS Spaces due to `NSWindowCollectionBehaviorCanJoinAllSpaces` being set. The fix involves removing this flag from the `setCollectionBehavior` call in `lib.rs` while keeping `FullScreenAuxiliary` to maintain the ability to appear on fullscreen spaces.

## Key Patterns Found

- `src-tauri/src/lib.rs:37-40` — Collection behavior is set using `msg_send!` macro with bitflags
- `src-tauri/src/lib.rs:26-55` — `bring_window_to_front` function handles all macOS-specific window configuration

## Approach

Remove the `CanJoinAllSpaces` flag (1<<0) from the collection behavior, keeping only `FullScreenAuxiliary` (1<<8). This is the minimal change that addresses the issue directly.

**Test strategy:** Manual QA — open file in Space 1, switch to Space 2, verify window stays in Space 1

## Tasks

### Task 1: Remove CanJoinAllSpaces flag in lib.rs

- **File:** `src-tauri/src/lib.rs` (Modify)
- **What:** Change line 39 from `let behavior: u64 = (1 << 0) | (1 << 8);` to `let behavior: u64 = 1 << 8;` and update the comment on line 37-38
- **Pattern:** `src-tauri/src/lib.rs:37-40` — existing collection behavior setup
- **Verify:** `cargo build --manifest-path src-tauri/Cargo.toml` → compiles successfully
- **Depends:** None

---

## Files to Modify

| File | Action | Description |
|------|--------|-------------|
| `src-tauri/src/lib.rs` | Modify | Remove CanJoinAllSpaces (1<<0) from collection behavior flags |

## Acceptance Criteria

- [ ] Windows stay on their original Space when user switches Spaces
- [ ] Windows still appear correctly on fullscreen Spaces
- [ ] App compiles without errors: `cargo build --manifest-path src-tauri/Cargo.toml`
