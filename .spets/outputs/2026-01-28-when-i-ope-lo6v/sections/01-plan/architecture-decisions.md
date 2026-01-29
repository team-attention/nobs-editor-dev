## Architecture Decisions

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

# Alternatives Considered

## Alternative 1: Keep CanJoinAllSpaces + Additional Logic

**Description:** Keep current behavior and add code to detect current Space and move window programmatically

**Pros:**
- More control over window placement
- Could potentially handle edge cases with custom logic

**Cons:**
- Complex implementation requiring significant additional code
- Requires private APIs or Accessibility permissions
- More points of failure and maintenance burden

**Decision:** Not chosen because it's over-engineering when the `MoveToActiveSpace` flag does exactly what's needed natively.

---

## Alternative 2: NSWindowCollectionBehaviorDefault

**Description:** Remove all Space-related behavior flags entirely, relying on default macOS behavior

**Pros:**
- Simpler code with fewer flags to manage
- Less risk of flag conflicts

**Cons:**
- Window may not follow expected Space behavior
- Could get stuck on original Space where app was first launched
- Unpredictable behavior across different macOS versions

**Decision:** Not chosen because it doesn't guarantee the window appears on the active Space when activated from the dock.

---

## Alternative 3: Combine CanJoinAllSpaces + MoveToActiveSpace

**Description:** Use both flags together: `(1 << 0) | (1 << 1) | (1 << 8)`

**Pros:**
- Might provide fallback behavior if one flag doesn't work
- Covers multiple scenarios

**Cons:**
- Flags are mutually exclusive in practice - `MoveToActiveSpace` takes precedence anyway
- Creates confusing code that suggests both behaviors are active
- May lead to undefined behavior in edge cases

**Decision:** Not chosen because it's redundant and potentially confusing. When `MoveToActiveSpace` is set, it overrides `CanJoinAllSpaces` behavior, making the combination unnecessary.

**Chosen approach:** Manual QA Only

**Rationale:** macOS window behavior and Space management cannot be unit tested - these are OS-level behaviors that require visual verification on a real macOS system. The change is a single flag value modification with well-documented behavior from Apple's AppKit documentation.

**Manual Verification Steps:**
1. Build the app with `cargo tauri build`
2. Create a test markdown file
3. Open Terminal in Space 2 (not Desktop 1)
4. Run `open test.md` or click a file:// link
5. Verify JustViewer window appears on Space 2 (same as Terminal)
6. Repeat test from different Spaces to confirm consistent behavior
