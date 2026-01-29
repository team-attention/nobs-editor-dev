---
id: 2026-01-29-improve-st-n81o
step: 01-plan
status: approved
updated_at: '2026-01-29T06:25:19.971Z'
---

# Plan: Inline Style Toolbar with File-Type Specific Controls

## Summary

Transform the current dropdown-based style panel into an inline horizontal toolbar displayed directly next to the filename. The toolbar will show different style controls based on file type: markdown files will have per-block controls (H1, H2, H3, Paragraph, Code), while code files will have a simpler code-focused control. This provides faster access to style settings without opening a dropdown.

## Key Patterns Found

- `src/main.tsx:327-401` — Toolbar structure with filename, buttons, and style panel in flex layout
- `src/main.tsx:36-50` — BlockStyles interface defining the customizable style properties
- `src/main.tsx:313-323` — CSS variable injection pattern for dynamic styling
- `src/main.tsx:52-56` — File type detection (markdown vs code) based on extension
- `src/styles.css:42-52` — Toolbar flexbox layout with gap and -webkit-app-region for window dragging
- `src/styles.css:86-171` — Current style panel dropdown styling

## Approach

Replace the dropdown panel with an inline horizontal style bar that appears after the filename, using compact +/- buttons or small increment controls for each style category. The toolbar will conditionally render different control sets based on `fileType` state. This approach maintains the single-file architecture while providing immediate access to style controls.

**Test strategy:** Manual QA - visually verify toolbar layout in both markdown and code files, test style adjustments, and confirm macOS window dragging still works.

## Tasks

### Task 1: Create inline style control components in main.tsx

- **File:** `src/main.tsx` (Modify)
- **What:**
  - Remove the dropdown-based style panel (lines 334-400)
  - Create inline `MarkdownStyleBar` component with compact controls for H1, H2, H3, P, Code sizes
  - Create inline `CodeStyleBar` component with just Code size control
  - Each control shows a label (e.g., "H1") with +/- buttons or a compact number input
  - Conditionally render based on `fileType` state
- **Pattern:** `src/main.tsx:327-401` — existing toolbar structure
- **Verify:** `npm run tauri dev` → Open markdown file, see H1/H2/H3/P/Code controls inline; Open code file, see only Code control
- **Depends:** None

---

### Task 2: Update toolbar CSS for inline style controls

- **File:** `src/styles.css` (Modify)
- **What:**
  - Remove `.style-panel` dropdown styles (lines 111-171)
  - Add `.inline-style-bar` class for horizontal layout with small gap
  - Add `.style-control` class for individual control groups (label + buttons)
  - Add `.style-btn` class for compact +/- buttons
  - Ensure `-webkit-app-region: no-drag` on interactive elements
  - Keep controls visually subtle until hovered
- **Pattern:** `src/styles.css:42-52` — toolbar flexbox layout
- **Verify:** `npm run tauri dev` → Controls display inline, hover effects work, window still draggable by toolbar background
- **Depends:** Task 1

---

### Task 3: Add visual separators and polish

- **File:** `src/styles.css` (Modify)
- **What:**
  - Add a subtle vertical separator between filename and style controls
  - Add visual feedback (color change) when adjusting values
  - Ensure dark mode compatibility for new styles
- **Pattern:** `src/styles.css:167-171` — dark mode style overrides
- **Verify:** `npm run tauri dev` → Toggle system dark mode, verify controls look correct in both themes
- **Depends:** Task 2

---

### Task 4: Remove click-outside handler and related code

- **File:** `src/main.tsx` (Modify)
- **What:**
  - Remove `showStylePanel` state (no longer needed)
  - Remove `stylePanelRef` ref (no longer needed)
  - Remove click-outside effect (lines 299-311)
  - Remove gear icon button (`#style-settings-btn`)
- **Pattern:** Clean up unused code
- **Verify:** `npm run tauri build` → No TypeScript errors, app builds successfully
- **Depends:** Task 1

---

## Files to Modify

| File | Action | Description |
|------|--------|-------------|
| `src/main.tsx` | Modify | Replace dropdown panel with inline style controls, add file-type conditional rendering |
| `src/styles.css` | Modify | Replace dropdown styles with inline control styles |

## Acceptance Criteria

- [ ] Style controls appear inline next to filename (not as dropdown)
- [ ] Markdown files show: H1, H2, H3, Paragraph, Code size controls
- [ ] Code files show: only Code size control
- [ ] Clicking +/- buttons adjusts the corresponding font size
- [ ] Window title bar remains draggable
- [ ] Dark mode styling works correctly
- [ ] `npm run tauri build` completes without errors

## Risks

- **Toolbar width overflow on narrow windows**: Controls might overflow on very narrow windows. Mitigation: Test with minimum window width; consider collapsing to icon-only on narrow views if needed.
- **Touch/click target size**: Compact buttons might be hard to click. Mitigation: Ensure minimum 24x24px touch targets on buttons.
