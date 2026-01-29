---
id: 2026-01-29-add-a-styl-ktzi
step: 01-plan
status: approved
updated_at: '2026-01-29T05:55:39.479Z'
---

# Plan: Add Block Style Customization Panel

## Summary

Add a style customization panel accessible from a button next to the filename in the toolbar. The panel allows users to individually customize font sizes for each block type (H1, H2, H3, paragraph, code). Settings are stored in state and applied via CSS custom properties to the editor container.

## Key Patterns Found

- `src/main.tsx:256-264` — Toolbar header structure with button, filename span, and spacer
- `src/main.tsx:60-70` — State management using React useState hooks
- `src/styles.css:7-24` — CSS variables pattern with light/dark mode support
- `src/styles.css:54-67` — Toolbar button styling patterns
- BlockNote uses `[data-content-type="heading"][data-level="1"]` selectors for targeting block types

## Approach

Add a settings button in the toolbar that opens a popover/panel with sliders or number inputs for each block type's font size. Store the custom values in React state and apply them as inline CSS custom properties on the editor container. CSS rules then use these variables to style specific block types via `[data-content-type]` selectors.

**Alternative considered:** Modal dialog - rejected for UX; popover allows quick adjustments while viewing the document.

**Test strategy:** Manual QA - verify panel opens, sliders change values, and font sizes update in real-time.

## Tasks

### Task 1: Define BlockStyles type and default values

- **File:** `src/main.tsx` (Modify)
- **What:**
  - Add `BlockStyles` interface with properties: `h1Size`, `h2Size`, `h3Size`, `paragraphSize`, `codeSize` (all numbers)
  - Add `DEFAULT_BLOCK_STYLES` constant with sensible defaults (e.g., H1: 28, H2: 22, H3: 18, paragraph: 15, code: 14)
  - Add `useState<BlockStyles>` hook for `blockStyles`
- **Pattern:** `src/main.tsx:60-70` — existing state declarations
- **Verify:** `npm run build` → no TypeScript errors
- **Depends:** None

---

### Task 2: Add settings button to toolbar

- **File:** `src/main.tsx` (Modify)
- **What:**
  - Add settings/gear icon button after `#filename` span
  - Add `useState<boolean>` for `showStylePanel`
  - Wire button `onClick` to toggle `showStylePanel`
- **Pattern:** `src/main.tsx:257-261` — existing open button structure
- **Verify:** `npm run dev` → settings button visible in toolbar, toggles state
- **Depends:** Task 1

---

### Task 3: Create StylePanel component with inputs

- **File:** `src/main.tsx` (Modify)
- **What:**
  - Create `StylePanel` component that renders when `showStylePanel` is true
  - Include labeled number inputs for each block type: H1, H2, H3, Paragraph, Code
  - Each input updates the corresponding `blockStyles` property via `onChange`
  - Position as a dropdown/popover below the settings button
- **Pattern:** React controlled inputs pattern
- **Verify:** Panel shows inputs, values update state on change
- **Depends:** Task 2

---

### Task 4: Apply block styles to editor container as CSS variables

- **File:** `src/main.tsx` (Modify)
- **What:**
  - Add inline `style` prop to `#editor-container` div
  - Map `blockStyles` state to CSS custom properties: `--h1-size`, `--h2-size`, etc.
  - Example: `style={{ '--h1-size': `${blockStyles.h1Size}px`, ... }}`
- **Pattern:** React inline styles with CSS variables
- **Verify:** DevTools shows CSS variables on container
- **Depends:** Task 3

---

### Task 5: Add CSS rules using block style variables

- **File:** `src/styles.css` (Modify)
- **What:**
  - Add rules targeting BlockNote blocks using `[data-content-type]` selectors
  - `[data-content-type="heading"][data-level="1"] { font-size: var(--h1-size, 28px) }`
  - `[data-content-type="heading"][data-level="2"] { font-size: var(--h2-size, 22px) }`
  - `[data-content-type="heading"][data-level="3"] { font-size: var(--h3-size, 18px) }`
  - `[data-content-type="paragraph"] { font-size: var(--p-size, 15px) }`
  - `.cm-editor { font-size: var(--code-size, 14px) }` for CodeMirror
- **Pattern:** `src/styles.css:7-24` — CSS variables pattern
- **Verify:** Changing values in panel updates editor typography
- **Depends:** Task 4

---

### Task 6: Style the settings button and panel

- **File:** `src/styles.css` (Modify)
- **What:**
  - Add `#style-settings-btn` styles matching existing toolbar buttons
  - Add `.style-panel` styles: positioned below toolbar, background, border, shadow, padding
  - Style inputs with labels in a clean vertical layout
  - Support both light and dark modes
- **Pattern:** `src/styles.css:54-72` — toolbar button styles
- **Verify:** Panel looks polished in both themes
- **Depends:** Task 3

---

## Files to Modify

| File | Action | Description |
|------|--------|-------------|
| `src/main.tsx` | Modify | Add BlockStyles state, settings button, StylePanel component, CSS variable binding |
| `src/styles.css` | Modify | Add block-type CSS rules with variables, panel and button styling |

## Acceptance Criteria

- [ ] Settings button (gear icon) appears in toolbar next to filename
- [ ] Clicking button opens a style customization panel
- [ ] Panel has inputs for: H1, H2, H3, Paragraph, Code font sizes
- [ ] Changing input values updates editor block font sizes in real-time
- [ ] Works for both markdown (BlockNote) and code (CodeMirror) files
- [ ] Panel styling works in light and dark mode
- [ ] No build errors

## Risks

- **CSS specificity**: BlockNote may have high-specificity default styles. Mitigation: use `!important` if needed or increase specificity with container selector.
