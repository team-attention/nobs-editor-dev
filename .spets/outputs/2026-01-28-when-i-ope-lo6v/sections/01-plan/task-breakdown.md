## Task Breakdown

### Phase 1: Code Changes

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

### Phase 2: Build & Manual Verification

#### Task 3: Build and Test on macOS

**File:** N/A (Build verification)

**Steps:**
1. Run `cd src-tauri && cargo build`
2. If build succeeds, run `cargo tauri dev` to test locally
3. Create a test markdown file in a known location
4. Open Terminal, switch to Space 2 (or any non-Desktop Space)
5. Run `open /path/to/test.md`
6. Observe: JustViewer should appear on Space 2, not switch to Desktop 1

**Verification:**
- [ ] Run: `cd src-tauri && cargo build`
- [ ] Expected: Build completes without errors
- [ ] Manual: App window appears on same Space as terminal

**Dependencies:** Task 1, Task 2

**Parallelizable:** NO

---

### Phase 3: Testing

No automated tests for this change - macOS window behavior requires manual verification.

All testing covered in Phase 2, Task 3 manual verification steps.

---

### Phase 4: Integration & Verification

#### Task 4: Final Verification Checklist

**Verification:**
- [ ] App opens on same Space as terminal when using `open file.md`
- [ ] App opens on same Space when clicking file:// link in terminal
- [ ] App still shows above fullscreen apps (FullScreenAuxiliary behavior preserved)
- [ ] Window can still be moved between Spaces manually
- [ ] Hide-on-close behavior still works
- [ ] App activation via Dock works correctly

**Dependencies:** All previous tasks

---
