#### Task 2: Update NSWindowCollectionBehavior in setup handler

**File:** `src-tauri/src/lib.rs` (Modify)

**Changes:**
- Modify: Window setup code in `run()` function
- Code location: Line 104
- Change from:
  ```rust
  // NSWindowCollectionBehaviorCanJoinAllSpaces | NSWindowCollectionBehaviorFullScreenAuxiliary
  let behavior: u64 = (1 << 0) | (1 << 8);
  ```
  To:
  ```rust
  // NSWindowCollectionBehaviorMoveToActiveSpace | NSWindowCollectionBehaviorFullScreenAuxiliary
  let behavior: u64 = (1 << 1) | (1 << 8);
  ```

**Steps:**
1. Navigate to line 104 in the `.setup()` handler
2. Change `(1 << 0)` to `(1 << 1)` in the behavior constant
3. Update comment to reflect new flag name: MoveToActiveSpace

**Verification:**
- [ ] Run: `cd src-tauri && cargo check`
- [ ] Expected: No compilation errors
- [ ] Check: Both occurrences of behavior constant use `(1 << 1)`

**Dependencies:** Task 1 (logical grouping, same file)

**Parallelizable:** YES (with Task 1, same file different locations)

**Commit Message:** (Combined with Task 1 in single commit)

---
