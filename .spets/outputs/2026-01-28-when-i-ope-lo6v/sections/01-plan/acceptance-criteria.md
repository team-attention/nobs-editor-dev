## Acceptance Criteria

### Implementation Complete
- [ ] `src-tauri/src/lib.rs` line 35: behavior uses `(1 << 1)` (MoveToActiveSpace)
- [ ] `src-tauri/src/lib.rs` line 104: behavior uses `(1 << 1)` (MoveToActiveSpace)
- [ ] Comments updated to reflect new flag name

### Tests Passing
- [ ] `cargo check` passes without errors
- [ ] `cargo build` completes successfully

### Code Quality
- [ ] No new warnings introduced
- [ ] Comments accurately describe the behavior flags

### Functionality
- [ ] Window appears on same Space as terminal when opening file
- [ ] Window appears on same Space when clicking file:// link
- [ ] FullScreenAuxiliary behavior preserved (appears over fullscreen apps)

### Non-Regression
- [ ] Hide-on-close behavior works
- [ ] Dock activation works
- [ ] Window can be manually moved between Spaces
- [ ] App shows in Cmd+Tab app switcher
