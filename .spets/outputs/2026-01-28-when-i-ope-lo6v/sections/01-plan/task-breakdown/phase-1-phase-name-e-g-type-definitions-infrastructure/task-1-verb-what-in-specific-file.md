#### Task 1: Update NSWindowCollectionBehavior in bring_window_to_front()

**File:** `src-tauri/src/lib.rs` (Modify)

**Changes:**
- Modify: `bring_window_to_front()` function collection behavior flag
- Code location: Line 35
- Change from:
  ```rust
  // NSWindowCollectionBehaviorCanJoinAllSpaces = 1 << 0
  let behavior: u64 = (1 << 0) | (1 << 8);
  ```
  To:
  ```rust
  // NSWindowCollectionBehaviorMoveToActiveSpace = 1 << 1
  // NSWindowCollectionBehaviorFullScreenAuxiliary = 1 << 8
  let behavior: u64 = (1 << 1) | (1 << 8);
  ```

**Steps:**
1. Open `src-tauri/src/lib.rs`
2. Navigate to line 35 in `bring_window_to_front()` function
3. Change `(1 << 0)` to `(1 << 1)` in the behavior constant
4. Update comment to reflect new flag name: MoveToActiveSpace

**Verification:**
- [ ] Run: `cd src-tauri && cargo check`
- [ ] Expected: No compilation errors
- [ ] Check: The behavior constant uses `(1 << 1)` instead of `(1 << 0)`

**Dependencies:** None

**Parallelizable:** NO (only task in this phase)

**Commit Message:** `fix: use MoveToActiveSpace for Arc-like window behavior`

---
