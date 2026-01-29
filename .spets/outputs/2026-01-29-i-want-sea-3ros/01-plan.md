---
id: 2026-01-29-i-want-sea-3ros
step: 01-plan
status: approved
updated_at: '2026-01-29T08:53:23.653Z'
---

# Plan: Toolbar-Integrated Search Feature

## Summary

Add a find-only search feature integrated into the toolbar for searching within the currently open document. The implementation will handle both editing modes (BlockNote for Markdown, CodeMirror for code files) with keyboard shortcut support (Cmd+F). The search will highlight matches and allow navigation between them.

## Key Patterns Found

- `src/main.tsx:146-158` — StyleControl component pattern for toolbar inline controls
- `src/main.tsx:448-462` — Inline style bar pattern showing how controls are added to toolbar
- `src/main.tsx:314-330` — Keyboard shortcut handling pattern with meta/ctrl key detection
- `src/main.tsx:31` — `highlightSelectionMatches` from @codemirror/search already imported (can leverage more search features)
- `src/main.tsx:232-246` — CodeMirror extensions setup pattern

## Approach

Use a unified search state managed in React while leveraging editor-specific search implementations:
- **CodeMirror**: Use `@codemirror/search` built-in search functionality via programmatic API
- **BlockNote**: Convert blocks to text, find matches, and use BlockNote's selection/highlighting API

**Alternative considered**: Floating search panel (rejected per user preference for toolbar integration).

**Test strategy:** Manual QA - verify search works in both Markdown and code files, test match navigation, test Cmd+F trigger.

## Tasks

### Task 1: Add search state and UI to toolbar in main.tsx

- **File:** `src/main.tsx` (Modify)
- **What:**
  - Add `searchQuery` and `searchMatches` state
  - Add search input to toolbar after filename
  - Add match count display and prev/next navigation buttons
  - Show/hide search bar based on `showSearch` state
- **Pattern:** `src/main.tsx:448-462` — inline style bar placement pattern
- **Verify:** `npm run dev` → Open file → Search input visible in toolbar
- **Depends:** None

---

### Task 2: Implement CodeMirror search integration

- **File:** `src/main.tsx` (Modify)
- **What:**
  - Import `SearchQuery`, `search`, `searchPanelOpen`, `findNext`, `findPrevious` from @codemirror/search
  - Add search extension to CodeMirror extensions array
  - Create `searchInCodeMirror(query)` function that uses SearchQuery to find and highlight matches
  - Wire search input to CodeMirror search state
- **Pattern:** `src/main.tsx:232-246` — CodeMirror extensions setup
- **Verify:** `npm run dev` → Open .json file → Type in search → Matches highlighted
- **Depends:** Task 1

---

### Task 3: Implement BlockNote search functionality

- **File:** `src/main.tsx` (Modify)
- **What:**
  - Create `searchInBlockNote(query)` function that:
    - Gets text content from all blocks
    - Finds match positions
    - Highlights matches using BlockNote's text cursor/selection API
  - Track current match index for navigation
- **Pattern:** Uses existing `editor` from `useCreateBlockNote()`
- **Verify:** `npm run dev` → Open .md file → Type in search → Matches highlighted
- **Depends:** Task 1

---

### Task 4: Add keyboard shortcut (Cmd+F) for search

- **File:** `src/main.tsx` (Modify)
- **What:**
  - Add `e.key === "f"` case to existing keyboard handler
  - Toggle search visibility and focus search input
  - Add Escape key to close search
- **Pattern:** `src/main.tsx:314-330` — keyboard shortcut pattern
- **Verify:** `npm run dev` → Press Cmd+F → Search bar appears/focuses → Press Escape → Search closes
- **Depends:** Task 1

---

### Task 5: Add search UI styles

- **File:** `src/styles.css` (Modify)
- **What:**
  - Add `.search-bar` container styles (flex, gap, alignment)
  - Add `.search-input` styles (width, padding, border)
  - Add `.search-nav-btn` styles for prev/next buttons
  - Add `.search-count` styles for match count display
  - Add dark mode variants
- **Pattern:** Existing toolbar and style control CSS patterns in styles.css
- **Verify:** Visual inspection in light/dark modes
- **Depends:** Task 1

---

## Files to Modify

| File | Action | Description |
|------|--------|-------------|
| `src/main.tsx` | Modify | Add search state, UI, keyboard shortcuts, and editor-specific search logic |
| `src/styles.css` | Modify | Add search bar styling for toolbar integration |

## Acceptance Criteria

- [ ] Search input visible in toolbar when a file is open
- [ ] Cmd+F toggles search bar visibility and focuses input
- [ ] Escape key closes search bar
- [ ] Typing in search input highlights matches in CodeMirror (code files)
- [ ] Typing in search input highlights matches in BlockNote (markdown files)
- [ ] Match count displayed (e.g., "3 matches" or "1/3")
- [ ] Prev/Next buttons navigate between matches
- [ ] Search UI styled consistently in light and dark modes

## Risks

| Risk | Impact | Mitigation |
|------|--------|------------|
| BlockNote search API limitations | May not support native text highlighting | Fall back to scrolling to match position and selecting text range |
| Performance on large files | Search could be slow | Debounce search input, limit highlight rendering |
