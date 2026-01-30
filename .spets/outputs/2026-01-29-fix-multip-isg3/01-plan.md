---
id: 2026-01-29-fix-multip-isg3
step: 01-plan
status: approved
updated_at: '2026-01-29T09:40:02.146Z'
---

# Plan: Fix Search & Editor Issues in Nobs Editor

## Summary

This plan addresses 6 issues in Nobs Editor: search hotkey conflict, broken tray quit menu, incorrect search result ordering, insufficient match highlighting, missing markdown search highlighting, and missing scrollbar match indicators. The fixes involve investigating the Cmd+F/fullscreen conflict, debugging the tray menu event handler, fixing the search result ordering logic, adding CSS for match visibility, implementing ProseMirror decorations for BlockNote, and building a custom scrollbar overlay for match positions.

## Key Patterns Found

- `src/main.tsx:541-564` — Keyboard event handler pattern using document.addEventListener for Cmd+O/S/F shortcuts
- `src/main.tsx:446-493` — Search implementation using CodeMirror's SearchQuery and cursor iteration
- `src/main.tsx:256-267` — CodeMirror extensions array pattern for adding editor features
- `src-tauri/src/lib.rs:165-178` — Tray menu setup using TrayIconBuilder with on_menu_event handler
- `src/styles.css:410-468` — Search UI styling using CSS custom properties for theming

## Approach

Fix each issue independently in order of dependency: (1) tray quit menu, (2) hotkey investigation, (3) search result ordering, (4) match highlight CSS, (5) markdown search highlighting with ProseMirror decorations, (6) custom scrollbar overlay. This order allows early issues to be verified before dependent features are built.

**Test strategy:** Manual QA - test each fix in the running app

## Tasks

### Task 1: Fix tray quit menu event handler in lib.rs

- **File:** `src-tauri/src/lib.rs` (Modify)
- **What:** The current tray menu setup at lines 165-178 may not be triggering correctly. Debug by adding logging and verify the menu event handler is receiving events. Check if `show_menu_on_left_click(true)` is interfering with menu item selection.
- **Pattern:** `src-tauri/src/lib.rs:173-177` — existing on_menu_event pattern
- **Verify:** Build app → click tray icon → click "Quit" → app should exit
- **Depends:** None

---

### Task 2: Investigate and fix search hotkey conflict with fullscreen

- **File:** `src/main.tsx` (Modify)
- **What:** The issue is that Cmd+F may be triggering when Ctrl+Cmd+F (macOS fullscreen) is pressed. Check if the keyboard handler at line 548 is checking for the wrong modifier combination. The condition `(e.metaKey || e.ctrlKey)` treats Ctrl+Cmd the same as just Cmd. Fix by requiring ONLY metaKey for Cmd+F without ctrlKey.
- **Pattern:** `src/main.tsx:548-559` — keyboard handler condition
- **Verify:** Press Ctrl+Cmd+F → should trigger fullscreen, not search. Press Cmd+F → should trigger search.
- **Depends:** None

---

### Task 3: Fix search result ordering in CodeMirror

- **File:** `src/main.tsx` (Modify)
- **What:** The search cursor at lines 469-473 iterates document order but the first `findNext()` call at line 477 jumps to a match position that may not be the first. Need to move cursor to document start before first search, or collect all matches with positions and sort by line number.
- **Pattern:** `src/main.tsx:461-478` — CodeMirror search dispatch pattern
- **Verify:** Open tauri.conf.json → search "ext" → first match should be at line 35 (earliest occurrence)
- **Depends:** None

---

### Task 4: Add visible background for matched text in CSS

- **File:** `src/styles.css` (Modify)
- **What:** Add CSS rules for `.cm-searchMatch` and `.cm-searchMatch-selected` to provide more visible highlighting. Use accent color with higher opacity for better visibility in both light and dark modes.
- **Pattern:** `src/styles.css:266-286` — CodeMirror dark mode styling pattern
- **Verify:** Open code file → search for term → matched text should have clearly visible yellow/orange background
- **Depends:** None

---

### Task 5: Implement markdown search highlighting with ProseMirror decorations

- **File:** `src/main.tsx` (Modify)
- **What:** The current markdown search at lines 479-492 only counts matches without visual highlighting. Need to:
  1. Access the underlying ProseMirror editor view from BlockNote
  2. Create a decoration plugin that marks search matches
  3. Apply the plugin when search query changes
  4. Navigate to matches using ProseMirror selection
- **Pattern:** BlockNote exposes `editor._tiptapEditor` for Tiptap/ProseMirror access
- **Verify:** Open markdown file → search for term → matched text should be highlighted with background color → Enter/Shift+Enter should navigate between matches
- **Depends:** Task 4 (CSS for highlight styling)

---

### Task 6: Add scrollbar match overlay component

- **File:** `src/main.tsx` (Modify), `src/styles.css` (Modify)
- **What:** Create a `<SearchMatchOverlay>` component that:
  1. Receives match positions as props (line numbers)
  2. Renders a thin vertical strip on the right side of the editor
  3. Shows colored markers at proportional positions for each match
  4. Updates when search query or document changes
  For CodeMirror: calculate positions from SearchQuery cursor
  For BlockNote: calculate from ProseMirror text positions
- **Pattern:** `src/main.tsx:576-627` — search bar conditional rendering pattern
- **Verify:** Search for term → thin overlay appears on right → colored dots indicate match positions in document
- **Depends:** Task 3, Task 5

---

## Files to Modify

| File | Action | Description |
|------|--------|-------------|
| `src-tauri/src/lib.rs` | Modify | Debug and fix tray menu quit event handler |
| `src/main.tsx` | Modify | Fix hotkey conflict, search ordering, add markdown highlighting, add scrollbar overlay |
| `src/styles.css` | Modify | Add search match highlight styles and scrollbar overlay styles |

## Acceptance Criteria

- [ ] Clicking "Quit Nobs Editor" in tray menu exits the application
- [ ] Cmd+F opens search, Ctrl+Cmd+F triggers macOS fullscreen (no conflict)
- [ ] Search results are ordered by line number (earliest match shown first)
- [ ] Matched text in code editor has clearly visible background highlight
- [ ] Matched text in markdown editor has visible background highlight
- [ ] Navigation (Enter/Shift+Enter) works in markdown editor to jump between matches
- [ ] Scrollbar overlay shows colored markers at match positions

## Risks

1. **BlockNote/ProseMirror internal API**: Accessing `_tiptapEditor` may break in future BlockNote versions. Mitigation: document the dependency and check BlockNote changelogs before updates.

2. **Scrollbar overlay performance**: Many matches could create performance issues. Mitigation: debounce updates and limit rendered markers to visible viewport + buffer.

3. **Dark mode compatibility**: New CSS may not look good in dark mode. Mitigation: test both modes and use CSS custom properties for colors.
