---
id: 2026-01-29-nobs-edito-sd7p
step: 01-plan
status: approved
updated_at: '2026-01-29T01:34:42.083Z'
---

# Plan: Add Recommended Application Status and Extended Syntax Highlighting for Code Files

## Summary

This plan extends Nobs Editor to be shown as a "Recommended Application" for common code files by expanding file associations in tauri.conf.json, and adds syntax highlighting support for popular programming languages (JavaScript, TypeScript, Python, CSS, HTML, Rust, C/C++, Go, SQL, Shell) using CodeMirror language packs. The app already has basic infrastructure in place; this is an incremental extension.

## Key Patterns Found

- `src/main.tsx:30-38` — `getLanguageExtension()` function maps file extensions to CodeMirror language extensions; needs extension for new languages
- `src/main.tsx:16-18` — Language imports follow pattern: `import { langName } from "@codemirror/lang-langname"`
- `src-tauri/tauri.conf.json:33-39` — File associations declared with `ext` array, `name`, and `role: "Editor"` for Recommended Application status
- `package.json:17-20` — CodeMirror language dependencies follow pattern `@codemirror/lang-*`

## Approach

Incrementally extend existing patterns: add new CodeMirror language packages to dependencies, expand the `getLanguageExtension()` switch statement, and add more file extensions to `fileAssociations` in tauri.conf.json. This maintains consistency with current architecture.

**Test strategy:** Manual QA - open files of each type and verify syntax highlighting works, verify app appears in "Open With" menu for new file types after rebuild.

## Tasks

### Task 1: Install CodeMirror language packages

- **File:** `package.json` (Modify)
- **What:** Add npm dependencies for CodeMirror language packs: `@codemirror/lang-javascript`, `@codemirror/lang-python`, `@codemirror/lang-css`, `@codemirror/lang-html`, `@codemirror/lang-rust`, `@codemirror/lang-cpp`, `@codemirror/lang-go`, `@codemirror/lang-sql`, `@codemirror/legacy-modes` (for shell)
- **Pattern:** `package.json:17-20` — existing CodeMirror lang dependencies
- **Verify:** `npm install` → no errors, `npm ls @codemirror/lang-javascript` → shows version
- **Depends:** None

---

### Task 2: Extend syntax highlighting in main.tsx

- **File:** `src/main.tsx` (Modify)
- **What:**
  1. Add imports for new language extensions
  2. Extend `getLanguageExtension()` switch statement to handle: js, jsx, ts, tsx, mjs, cjs (JavaScript), py, pyw (Python), css, scss, less (CSS), html, htm, vue, svelte (HTML), rs (Rust), c, h, cpp, hpp, cc, cxx (C/C++), go (Go), sql (SQL), sh, bash, zsh (Shell via StreamLanguage)
- **Pattern:** `src/main.tsx:30-38` — existing switch statement pattern
- **Verify:** Open a `.js` file → JavaScript syntax highlighting visible; Open a `.py` file → Python syntax highlighting visible
- **Depends:** Task 1

---

### Task 3: Expand file associations in tauri.conf.json

- **File:** `src-tauri/tauri.conf.json` (Modify)
- **What:** Add common code file extensions to the `fileAssociations` array: js, jsx, ts, tsx, mjs, cjs, py, pyw, css, scss, less, html, htm, vue, svelte, rs, c, h, cpp, hpp, cc, cxx, go, sql, sh, bash, zsh
- **Pattern:** `src-tauri/tauri.conf.json:33-39` — existing fileAssociations structure
- **Verify:** `npm run tauri build` → check generated Info.plist in app bundle contains CFBundleDocumentTypes for new extensions
- **Depends:** None

---

### Task 4: Update open file dialog filters

- **File:** `src/main.tsx` (Modify)
- **What:** Update the `openFile()` dialog filters to include new file types in appropriate categories (Code Files, Web Files, etc.)
- **Pattern:** `src/main.tsx:150-154` — existing dialog filter structure
- **Verify:** Cmd+O → File dialog shows new file types in filters
- **Depends:** None

---

## Files to Modify

| File | Action | Description |
|------|--------|-------------|
| `package.json` | Modify | Add CodeMirror language package dependencies |
| `src/main.tsx` | Modify | Add language imports, extend getLanguageExtension() switch, update dialog filters |
| `src-tauri/tauri.conf.json` | Modify | Add code file extensions to fileAssociations |

## Acceptance Criteria

- [ ] `npm run tauri build` completes without errors
- [ ] Opening a `.js` file shows JavaScript syntax highlighting (keywords colored, strings highlighted)
- [ ] Opening a `.py` file shows Python syntax highlighting
- [ ] Opening a `.html` file shows HTML syntax highlighting
- [ ] Opening a `.css` file shows CSS syntax highlighting
- [ ] Opening a `.rs` file shows Rust syntax highlighting
- [ ] Right-clicking a `.js` file in Finder and selecting "Open With" shows Nobs Editor as an option
- [ ] File dialog (Cmd+O) shows new file type categories

## Risks

- **Bundle size increase**: Adding multiple language packs will increase the app bundle size. Mitigation: Use tree-shaking, consider lazy loading if bundle becomes too large.
- **Shell syntax highlighting**: The `@codemirror/legacy-modes` package uses different API (StreamLanguage). Mitigation: Follow CodeMirror documentation for legacy mode integration.
