---
id: 2026-01-29-make-it-op-dgld
step: 01-plan
status: approved
updated_at: '2026-01-28T15:25:20.392Z'
---

# Plan: Open New Window Per Unique File

## Summary

Currently JustViewer reuses a single "main" window for all files. This plan modifies the Rust backend to create a new window for each unique file, but reuse the existing window if the same file is opened again. A HashMap will track which file is open in which window, allowing the app to focus the existing window when the same file is requested.

## Key Patterns Found

- `src-tauri/src/lib.rs:59-92` — `open_window` function creates/reuses single window with label "main"
- `src-tauri/src/lib.rs:70` — `WebviewWindowBuilder::new(app, "main", WebviewUrl::default())` uses static label
- `src-tauri/src/lib.rs:95-138` — `process_urls` collects paths and opens window, then emits events
- `src-tauri/src/lib.rs:5` — `OpenedFiles(Mutex<Vec<String>>)` pattern for state management
- `src/main.tsx:198-216` — Frontend listens for `file-opened` event and also checks `get_opened_files` on startup

## Approach

Replace the single-window pattern with per-file window tracking using a `HashMap<String, String>` (file path → window label). When a file is opened:
1. Check if that file is already open in an existing window
2. If yes, show and focus that window
3. If no, create a new window with a unique label and pass the file path via URL query parameter

The frontend reads the file path from URL on load. This approach balances the requirement to not reuse windows for different files while avoiding duplicate windows for the same file.

**Test strategy:** Manual QA - open multiple different files (each in new window), then reopen same file (should focus existing window)

## Tasks

### Task 1: Add file-to-window tracking state in `lib.rs`

- **File:** `src-tauri/src/lib.rs` (Modify)
- **What:**
  - Replace `OpenedFiles(Mutex<Vec<String>>)` with `OpenWindows(Mutex<HashMap<String, String>>)` mapping file path to window label
  - Update the `manage()` call to use the new state type
- **Pattern:** `src-tauri/src/lib.rs:5` — existing Mutex state pattern
- **Verify:** `cargo build --manifest-path src-tauri/Cargo.toml` → compiles without errors
- **Depends:** None

---

### Task 2: Modify `open_window` to accept file path and check for existing windows in `lib.rs`

- **File:** `src-tauri/src/lib.rs` (Modify)
- **What:**
  - Change signature to `open_window(app: &AppHandle, path: &str, state: &tauri::State<OpenWindows>)`
  - Check if `path` exists in the HashMap
  - If exists: get the window by label, show it, focus it, return early
  - If not: generate unique window label (e.g., `window_{timestamp_millis}`)
  - Create new window with `WebviewUrl::App(format!("?file={}", urlencoding::encode(path)).into())`
  - Store `(path, label)` in the HashMap
  - Keep `bring_window_to_front` and hide-on-close behavior
  - On window close/destroy, remove the entry from HashMap
- **Pattern:** `src-tauri/src/lib.rs:61-67` — existing window lookup pattern
- **Verify:** `cargo build --manifest-path src-tauri/Cargo.toml` → compiles without errors
- **Depends:** Task 1

---

### Task 3: Add `urlencoding` dependency to Cargo.toml

- **File:** `src-tauri/Cargo.toml` (Modify)
- **What:**
  - Add `urlencoding = "2"` to dependencies section for proper URL encoding of file paths
- **Verify:** `cargo build --manifest-path src-tauri/Cargo.toml` → dependency resolves
- **Depends:** None

---

### Task 4: Update `process_urls` to use new `open_window` signature in `lib.rs`

- **File:** `src-tauri/src/lib.rs` (Modify)
- **What:**
  - For each file path, call `open_window(app, &path, &state)`
  - Remove the thread spawn that emits events (no longer needed)
  - Remove the intermediate `OpenedFiles` storage logic
- **Pattern:** `src-tauri/src/lib.rs:127-137` — current processing loop
- **Verify:** `cargo build --manifest-path src-tauri/Cargo.toml` → compiles without errors
- **Depends:** Tasks 2-3

---

### Task 5: Update frontend to read file path from URL query parameter in `main.tsx`

- **File:** `src/main.tsx` (Modify)
- **What:**
  - On mount, parse `window.location.search` for `file` parameter using `URLSearchParams`
  - If present, call `loadFile(decodeURIComponent(filePath))`
  - Remove the `file-opened` event listener (lines 200-202)
  - Remove the `get_opened_files` check (lines 205-211)
  - Keep the dialog-based `openFile` function for Cmd+O usage
- **Pattern:** `src/main.tsx:140-146` — existing pendingFile processing
- **Verify:** `npm run build` → builds without errors
- **Depends:** Task 4

---

### Task 6: Remove unused commands from `lib.rs`

- **File:** `src-tauri/src/lib.rs` (Modify)
- **What:**
  - Remove `get_opened_files` and `clear_opened_files` commands
  - Update `invoke_handler` to remove these commands
- **Verify:** `cargo build --manifest-path src-tauri/Cargo.toml` → compiles without errors
- **Depends:** Task 5

---

## Files to Modify

| File | Action | Description |
|------|--------|-------------|
| `src-tauri/Cargo.toml` | Modify | Add `urlencoding` dependency |
| `src-tauri/src/lib.rs` | Modify | Add file→window tracking, create unique windows per file, reuse for same file |
| `src/main.tsx` | Modify | Read file path from URL query param instead of events |

## Acceptance Criteria

- [ ] Opening a file creates a new window for that file
- [ ] Opening a second different file creates another new window
- [ ] Opening the same file again focuses the existing window (does not create duplicate)
- [ ] Each window displays its own file content correctly
- [ ] Cmd+O dialog still works within each window
- [ ] Window hide-on-close behavior preserved
- [ ] App compiles without warnings related to these changes

## Risks

| Risk | Impact | Mitigation |
|------|--------|------------|
| URL encoding issues with special characters in file paths | File won't load | Use `urlencoding` crate on Rust side, `decodeURIComponent` on JS side |
| Window label collisions | Window creation fails | Use high-precision timestamp (milliseconds) |
| Window destroyed but HashMap not updated | Stale entries, wrong window focused | Add window close event handler to clean up HashMap |
