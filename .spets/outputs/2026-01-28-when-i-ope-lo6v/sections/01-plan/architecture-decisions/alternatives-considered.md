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
