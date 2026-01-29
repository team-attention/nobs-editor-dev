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
