---
id: 2026-01-29-make-it-op-dgld
step: 02-implement
status: approved
updated_at: '2026-01-28T15:29:42.010Z'
---

# Implementation: Open New Window Per Unique File

## Summary

Implemented per-file window management that creates a new window for each unique file while reusing existing windows when the same file is opened again. The backend now tracks file-to-window mappings using a HashMap, passes file paths to windows via URL query parameters, and the frontend reads the file path from the URL on load.

## Tasks Completed

### Task 1: Add file-to-window tracking state in `lib.rs`

- **Status:** ✅ Complete
- **Files:** `src-tauri/src/lib.rs` (Modified)
- **Changes:** Replaced `OpenedFiles(Mutex<Vec<String>>)` with `OpenWindows(Mutex<HashMap<String, String>>)` to track file path → window label mapping
- **Verification:** `cargo build --manifest-path src-tauri/Cargo.toml` → Compiles successfully

---

### Task 2: Modify `open_window` to accept file path and check for existing windows in `lib.rs`

- **Status:** ✅ Complete
- **Files:** `src-tauri/src/lib.rs` (Modified)
- **Changes:**
  - Renamed function to `open_window_for_file(app: &AppHandle, path: &str, state: &tauri::State<OpenWindows>)`
  - Checks HashMap for existing window for the file path
  - If exists: shows and focuses that window
  - If not: generates unique label using timestamp, creates window with URL query parameter
  - Added cleanup on window destroy event
- **Verification:** `cargo build --manifest-path src-tauri/Cargo.toml` → Compiles successfully

---

### Task 3: Add `urlencoding` dependency to Cargo.toml

- **Status:** ✅ Complete
- **Files:** `src-tauri/Cargo.toml` (Modified)
- **Changes:** Added `urlencoding = "2"` to dependencies
- **Verification:** `cargo build --manifest-path src-tauri/Cargo.toml` → Dependency resolved and downloaded

---

### Task 4: Update `process_urls` to use new `open_window` signature in `lib.rs`

- **Status:** ✅ Complete
- **Files:** `src-tauri/src/lib.rs` (Modified)
- **Changes:**
  - Simplified to loop through URLs and call `open_window_for_file` for each path
  - Removed thread spawn and event emission logic
  - Removed intermediate state storage
- **Verification:** `cargo build --manifest-path src-tauri/Cargo.toml` → Compiles successfully

---

### Task 5: Update frontend to read file path from URL query parameter in `main.tsx`

- **Status:** ✅ Complete
- **Files:** `src/main.tsx` (Modified)
- **Changes:**
  - Removed `listen` import (no longer needed)
  - Replaced event listener and `get_opened_files` check with URL query parameter parsing
  - Uses `URLSearchParams` to read `file` parameter and `decodeURIComponent` to decode path
- **Verification:** `npm run build` → Builds successfully

---

### Task 6: Remove unused commands from `lib.rs`

- **Status:** ✅ Complete
- **Files:** `src-tauri/src/lib.rs` (Modified)
- **Changes:**
  - Removed `get_opened_files` and `clear_opened_files` commands
  - Updated `invoke_handler` to only include `read_file` and `write_file`
- **Verification:** `cargo build --manifest-path src-tauri/Cargo.toml` → Compiles successfully

---

## Changes Made

| File | Change |
|------|--------|
| `src-tauri/Cargo.toml` | Added `urlencoding = "2"` dependency |
| `src-tauri/src/lib.rs` | Replaced single-window pattern with per-file window tracking using HashMap; pass file via URL query param |
| `src/main.tsx` | Read file path from URL query parameter instead of events; removed unused imports |

## Deviations

None. Plan was followed exactly.

## Verification

```bash
# Rust backend build
$ cargo build --manifest-path src-tauri/Cargo.toml
   Compiling justviewer v0.3.0
warning: use of deprecated... (existing warnings from objc crate, not related to changes)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 35.47s

# Frontend build
$ npm run build
> justviewertauri@0.3.0 build
> tsc && vite build
✓ built in 3.13s
```

## Acceptance Criteria

- [x] Opening a file creates a new window for that file - Implemented via `open_window_for_file` which creates new window with unique label
- [x] Opening a second different file creates another new window - Each file gets unique window label based on timestamp
- [x] Opening the same file again focuses the existing window (does not create duplicate) - HashMap lookup returns existing window label, focuses it
- [x] Each window displays its own file content correctly - File path passed via URL query parameter, frontend reads and loads it
- [x] Cmd+O dialog still works within each window - `openFile` function unchanged, calls `loadFile` directly
- [x] Window hide-on-close behavior preserved - `on_window_event` handler still prevents close and hides
- [x] App compiles without warnings related to these changes - Only pre-existing objc deprecation warnings
