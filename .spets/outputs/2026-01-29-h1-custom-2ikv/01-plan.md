---
id: 2026-01-29-h1-custom-2ikv
step: 01-plan
status: approved
updated_at: '2026-01-29T07:47:25.106Z'
---

# Plan: Fix H1 Custom Styles and Add Bottom Gap

## Summary

The H1 heading custom font sizes are not being applied because BlockNote's internal CSS has higher selector specificity than the app's custom styles. Additionally, the editor container needs more padding at the bottom for better UX. This fix involves updating the CSS selectors to match or exceed BlockNote's specificity and adding bottom padding.

## Key Patterns Found

- `src/styles.css:217` — Current H1 style using simple attribute selector that gets overridden
- `node_modules/@blocknote/core/src/editor/Block.css:167-177` — BlockNote's complex selector pattern: `.bn-block-outer:not([data-prev-type]) > .bn-block > .bn-block-content[data-content-type="heading"]`
- `src/styles.css:204` — Editor container padding definition: `padding: 40px 20px`

## Approach

Update CSS selectors to use the same complex selector pattern as BlockNote (`.bn-block-outer > .bn-block > .bn-block-content`) combined with `#editor-container` for sufficient specificity to override the library styles. Add `padding-bottom` to the editor container.

**Test strategy:** Manual QA - verify heading styles change when toolbar controls are adjusted, and confirm extra space at bottom

## Tasks

### Task 1: Update heading style selectors in CSS

- **File:** `src/styles.css` (Modify)
- **What:** Replace the current simple attribute selectors with higher-specificity selectors that match BlockNote's internal structure:
  - Change `#editor-container [data-content-type="heading"][data-level="1"]` to `#editor-container .bn-block-outer > .bn-block > .bn-block-content[data-content-type="heading"][data-level="1"]`
  - Apply same pattern for H2 and H3
- **Pattern:** `node_modules/@blocknote/core/src/editor/Block.css:167` — BlockNote's heading selector pattern
- **Verify:** Open app, load a markdown file with headings, adjust H1 size in toolbar → heading size changes visually
- **Depends:** None

---

### Task 2: Add bottom padding to editor container

- **File:** `src/styles.css` (Modify)
- **What:** Change `#editor-container` padding from `40px 20px` to `40px 20px 80px 20px` (or use explicit `padding-bottom: 80px`)
- **Verify:** Open app with markdown file → visible extra space at bottom of editor content
- **Depends:** None

---

## Files to Modify

| File | Action | Description |
|------|--------|-------------|
| `src/styles.css` | Modify | Update heading selectors for higher specificity, add bottom padding |

## Acceptance Criteria

- [ ] H1 font size changes when adjusted via toolbar controls
- [ ] H2 font size changes when adjusted via toolbar controls
- [ ] H3 font size changes when adjusted via toolbar controls
- [ ] Paragraph font size continues to work as before
- [ ] Editor container has visible extra gap at the bottom (~80px or more)

## Risks

None - this is a CSS-only change with no functional code modifications.
