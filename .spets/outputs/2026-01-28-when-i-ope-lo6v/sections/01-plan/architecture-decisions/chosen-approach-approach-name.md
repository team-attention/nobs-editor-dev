### Chosen Approach: MoveToActiveSpace Flag

**How it works:**
1. Change `NSWindowCollectionBehavior` from `CanJoinAllSpaces` (1 << 0) to `MoveToActiveSpace` (1 << 1)
2. Keep `FullScreenAuxiliary` flag (1 << 8) to maintain compatibility with fullscreen apps
3. Window will automatically move to whichever Space is currently active when app is activated

**Why this approach:**
- Follows pattern from `src-tauri/src/lib.rs:32-36` - Same code structure, just different flag value
- Consistent with Arc browser behavior (same macOS API flag)
- Minimal change - single constant value modification
- Simple and focused (avoids over-engineering)

**Pattern References:**
- `src-tauri/src/lib.rs:35` - Collection behavior flag setting
  - **Extract:** `let behavior: u64 = (1 << X) | (1 << 8);` pattern
  - **Apply to:** Change X from 0 to 1
- Apple Documentation: NSWindowCollectionBehaviorMoveToActiveSpace = 1 << 1
  - **Extract:** Correct flag value for desired behavior
  - **Apply to:** Replace CanJoinAllSpaces with MoveToActiveSpace
