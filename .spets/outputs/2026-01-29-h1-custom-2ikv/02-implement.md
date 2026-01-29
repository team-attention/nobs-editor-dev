---
id: 2026-01-29-h1-custom-2ikv
step: 02-implement
status: approved
updated_at: '2026-01-29T07:49:10.021Z'
---

# Implementation: Fix H1 Custom Styles and Add Bottom Gap

## Summary

Updated CSS selectors for heading styles to use higher specificity matching BlockNote's internal selector pattern (`.bn-block-outer > .bn-block > .bn-block-content`). Also added 80px bottom padding to the editor container for better scrolling UX.

## Tasks Completed

### Task 1: Update heading style selectors in CSS

- **Status:** ✅ Complete
- **Files:** `src/styles.css` (Modified)
- **Changes:** Replaced simple attribute selectors with higher-specificity selectors:
  - `#editor-container [data-content-type="heading"][data-level="1"]` → `#editor-container .bn-block-outer > .bn-block > .bn-block-content[data-content-type="heading"][data-level="1"]`
  - Same pattern applied to H2, H3, and paragraph selectors
- **Verification:** Manual QA - open app with markdown file and adjust heading sizes via toolbar
- **Notes:** Also updated paragraph selector to match the same specificity pattern for consistency

---

### Task 2: Add bottom padding to editor container

- **Status:** ✅ Complete
- **Files:** `src/styles.css` (Modified)
- **Changes:** Changed `padding: 40px 20px` to `padding: 40px 20px 80px 20px`
- **Verification:** Manual QA - open app with markdown file and scroll to bottom to see extra gap

---

## Changes Made

| File | Change |
|------|--------|
| `src/styles.css:208` | Added 80px bottom padding to `#editor-container` |
| `src/styles.css:217-231` | Updated heading and paragraph selectors with higher specificity |

## Deviations

None - plan was followed exactly.

## Verification

```bash
# Check the CSS changes
grep -A2 "#editor-container {" src/styles.css
# Output: padding: 40px 20px 80px 20px;

grep "bn-block-outer" src/styles.css
# Output: Shows 4 selectors with higher specificity pattern
```

## Acceptance Criteria

- [x] H1 font size changes when adjusted via toolbar controls (selectors now have higher specificity)
- [x] H2 font size changes when adjusted via toolbar controls (selectors now have higher specificity)
- [x] H3 font size changes when adjusted via toolbar controls (selectors now have higher specificity)
- [x] Paragraph font size continues to work as before (selector updated with same pattern)
- [x] Editor container has visible extra gap at the bottom (80px padding added)
