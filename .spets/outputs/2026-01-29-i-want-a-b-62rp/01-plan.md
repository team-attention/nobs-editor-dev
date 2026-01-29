---
id: 2026-01-29-i-want-a-b-62rp
step: 01-plan
status: approved
updated_at: '2026-01-29T06:25:39.118Z'
---

# Plan: Collapsible Editable Frontmatter Panel

## Summary

Add a collapsible frontmatter panel above the BlockNote editor that displays and allows editing of YAML frontmatter from markdown files. The panel will parse frontmatter on file load, display key-value pairs in an editable form UI, and reconstruct the frontmatter when saving. This approach keeps frontmatter separate from the document body while providing an Obsidian-like properties experience.

## Key Patterns Found

- `src/main.tsx:88-90` — Block styles state management pattern for UI customization
- `src/main.tsx:299-311` — Click-outside-to-close pattern for style panel
- `src/main.tsx:345-398` — Collapsible panel UI structure with form controls
- `src/styles.css:111-171` — Style panel CSS (positioning, form styling, dark mode)
- `src/main.tsx:116-122` — File load flow where frontmatter extraction should occur
- `src/main.tsx:212-219` — File save flow where frontmatter reconstruction should occur

## Approach

Use a manual YAML frontmatter parser (regex-based extraction) rather than adding a dependency like `gray-matter`. Extract frontmatter before passing content to BlockNote, store it in React state, render as an editable key-value form in a collapsible panel, and prepend it back when saving. This keeps the solution simple and avoids npm dependency overhead.

**Test strategy:** Manual QA - open markdown files with/without frontmatter, edit values, save, verify preservation

## Tasks

### Task 1: Add frontmatter state and parsing utility in main.tsx

- **File:** `src/main.tsx` (Modify)
- **What:** Add `frontmatter` state (Map or object), `showFrontmatter` toggle state, and utility functions `parseFrontmatter(content) → { frontmatter, body }` and `serializeFrontmatter(frontmatter) → string`
- **Pattern:** `src/main.tsx:88` — useState pattern for UI state
- **Verify:** Manual - load a file with frontmatter, check console.log for parsed result
- **Depends:** None

---

### Task 2: Modify loadFile to extract and store frontmatter

- **File:** `src/main.tsx` (Modify)
- **What:** In the `loadFile` callback, call `parseFrontmatter(content)` before passing to BlockNote. Store frontmatter in state, pass only `body` to `tryParseMarkdownToBlocks()`
- **Pattern:** `src/main.tsx:116-122` — existing file load flow
- **Verify:** Manual - open file with frontmatter, confirm BlockNote doesn't show raw YAML
- **Depends:** Task 1

---

### Task 3: Modify saveFile to reconstruct frontmatter

- **File:** `src/main.tsx` (Modify)
- **What:** In the `saveFile` callback, prepend `serializeFrontmatter(frontmatter)` to the markdown content before writing to disk
- **Pattern:** `src/main.tsx:212-219` — existing save flow
- **Verify:** Manual - edit document, save, reopen, confirm frontmatter preserved
- **Depends:** Task 1

---

### Task 4: Create FrontmatterPanel component in main.tsx

- **File:** `src/main.tsx` (Modify)
- **What:** Add inline `FrontmatterPanel` component that renders a collapsible panel showing frontmatter key-value pairs. Include: expand/collapse toggle, list of editable key-value rows, add/remove property buttons
- **Pattern:** `src/main.tsx:345-398` — existing style panel structure
- **Verify:** Manual - panel renders above editor, can expand/collapse
- **Depends:** Task 1

---

### Task 5: Add editable key-value inputs in FrontmatterPanel

- **File:** `src/main.tsx` (Modify)
- **What:** Each frontmatter property renders as a row with key input (disabled for existing, editable for new) and value input. On change, update frontmatter state. Include delete button per row and "Add Property" button
- **Pattern:** `src/main.tsx:348-364` — input controls pattern
- **Verify:** Manual - edit a frontmatter value, verify state updates
- **Depends:** Task 4

---

### Task 6: Add frontmatter panel CSS styles

- **File:** `src/styles.css` (Modify)
- **What:** Add `.frontmatter-panel` styles: collapsible header, property rows layout, input styling matching existing style panel, dark mode support
- **Pattern:** `src/styles.css:111-171` — style panel CSS structure
- **Verify:** Visual - panel looks native to the app, works in dark mode
- **Depends:** Task 4

---

### Task 7: Integrate panel into editor layout

- **File:** `src/main.tsx` (Modify)
- **What:** Render `FrontmatterPanel` inside `#editor-container` above `BlockNoteView`, only when `fileType === "markdown"` and frontmatter exists (or allow creating new)
- **Pattern:** `src/main.tsx:417-424` — conditional rendering in editor area
- **Verify:** Manual - panel appears above editor for markdown files only
- **Depends:** Task 4, Task 6

## Files to Modify

| File | Action | Description |
|------|--------|-------------|
| `src/main.tsx` | Modify | Add frontmatter parsing, state, panel component, integration |
| `src/styles.css` | Modify | Add frontmatter panel styles |

## Acceptance Criteria

- [ ] Opening a markdown file with YAML frontmatter shows panel with parsed properties
- [ ] Opening a markdown file without frontmatter shows collapsed/empty panel
- [ ] Editing a frontmatter value and saving preserves the change
- [ ] Adding a new property via "Add" button works and saves correctly
- [ ] Removing a property via delete button removes it from saved file
- [ ] Panel can be collapsed/expanded
- [ ] Code files do not show frontmatter panel
- [ ] Dark mode styling matches existing app theme

## Risks

- **Complex YAML values (arrays, nested objects):** Impact - may not render correctly in simple key-value UI. Mitigation - detect complex values and show as raw YAML string in textarea, or show "Edit as YAML" fallback.
- **Frontmatter parsing edge cases:** Impact - malformed YAML or edge cases could break parsing. Mitigation - wrap in try-catch, preserve raw frontmatter string as fallback if parsing fails.
