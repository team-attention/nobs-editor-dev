## Codebase Analysis

**Relevant Files Found:**
- `src-tauri/src/lib.rs` - Main Tauri app logic with macOS window management via cocoa/objc
- `src-tauri/Cargo.toml` - Rust dependencies including cocoa v0.26 and objc v0.2

**Existing Patterns to Follow:**
- `src-tauri/src/lib.rs:32-36` - NSWindowCollectionBehavior flag setting pattern
  - WHY: Shows exact syntax for setting collection behavior via `msg_send!` macro with bitwise OR flags
- `src-tauri/src/lib.rs:100-107` - Startup-time collection behavior setting
  - WHY: Same pattern applied during window setup, both locations need modification for consistency
- `src-tauri/src/lib.rs:20-52` - Complete `bring_window_to_front()` implementation
  - WHY: Contains all macOS window activation logic including level setting and activation

**Similar Features:**
- Window activation in `bring_window_to_front()` - Uses cocoa/objc for direct NSWindow access via `ns_window()` method
- Collection behavior already set in two places (setup and bring_window_to_front), just needs flag value change from `CanJoinAllSpaces` to `MoveToActiveSpace`
- Window level override at line 39 (`setLevel: 101i64`) may need adjustment alongside behavior change

**Testing Approach:**
- Testing library: Manual QA only (macOS window behavior cannot be unit tested)
- Verification steps:
  1. Build app with `cargo tauri build` or run dev with `cargo tauri dev`
  2. Open JustViewer in Space 1
  3. Switch to Space 2 (fullscreen or regular)
  4. Run `open -a JustViewer /path/to/file.md` from terminal
  5. Verify: Window should move to Space 2 (current active Space) instead of appearing on Space 1

**File Structure:**
- `src-tauri/src/` - Rust backend code
- `src-tauri/src/lib.rs` - Core app logic including window management (163 lines)
- All macOS-specific code in `bring_window_to_front()` function

**Dependencies:**
- External: `cocoa = "0.26"`, `objc = "0.2"` (already present in Cargo.toml)
- Internal: Uses `tauri::Manager`, `tauri::AppHandle` for window access

**Code Conventions:**
- macOS code wrapped in `#[cfg(target_os = "macos")]`
- Uses unsafe blocks with explicit `msg_send!` macro for Objective-C calls
- Error handling: Uses `if let` pattern for optional window access

**Integration Points:**
- `bring_window_to_front()` called from `RunEvent::Ready` and `RunEvent::Opened`
- Also called from `process_urls()` after file handling

**Existing Utilities to Reuse:**
- No additional utilities needed - single flag change in existing code

**Key Finding:**
- Current implementation uses `NSWindowCollectionBehaviorCanJoinAllSpaces` (1 << 0) which causes window to appear on ALL Spaces simultaneously
- Solution: Change to `NSWindowCollectionBehaviorMoveToActiveSpace` (1 << 1) which moves window to current Space when activated (Arc browser behavior)
