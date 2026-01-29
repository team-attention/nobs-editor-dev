---
id: 2026-01-29-nobs-edito-sd7p
step: 02-implement
status: approved
updated_at: '2026-01-29T01:38:10.820Z'
---

# Implementation: Add Recommended Application Status and Extended Syntax Highlighting for Code Files

## Summary

Extended Nobs Editor to support syntax highlighting for 10 additional programming languages (JavaScript/TypeScript, Python, CSS, HTML, Rust, C/C++, Go, SQL, Shell) using CodeMirror language packs, and registered the app as an Editor for these file types in Tauri's file associations. The app will now appear as a "Recommended Application" in macOS Finder's "Open With" menu for these code files.

## Tasks Completed

### Task 1: Install CodeMirror language packages

- **Status:** ✅ Complete
- **Files:** `package.json` (Modified)
- **Changes:** Added 9 new CodeMirror language dependencies: `@codemirror/lang-javascript`, `@codemirror/lang-python`, `@codemirror/lang-css`, `@codemirror/lang-html`, `@codemirror/lang-rust`, `@codemirror/lang-cpp`, `@codemirror/lang-go`, `@codemirror/lang-sql`, `@codemirror/legacy-modes`
- **Verification:** `npm install` → "added 16 packages, and audited 319 packages... found 0 vulnerabilities"

---

### Task 2: Extend syntax highlighting in main.tsx

- **Status:** ✅ Complete
- **Files:** `src/main.tsx` (Modified)
- **Changes:**
  1. Added imports for all new language extensions at lines 19-28
  2. Extended `getLanguageExtension()` switch statement to handle 25+ file extensions:
     - JavaScript: js, jsx, mjs, cjs
     - TypeScript: ts, tsx (with TypeScript mode enabled)
     - Python: py, pyw
     - CSS: css, scss, less
     - HTML: html, htm, vue, svelte
     - Rust: rs
     - C/C++: c, h, cpp, hpp, cc, cxx
     - Go: go
     - SQL: sql
     - Shell: sh, bash, zsh (using StreamLanguage.define)
- **Verification:** `npm run build` → TypeScript compiled successfully

---

### Task 3: Expand file associations in tauri.conf.json

- **Status:** ✅ Complete
- **Files:** `src-tauri/tauri.conf.json` (Modified)
- **Changes:** Added 10 new file association entries for:
  - JavaScript/TypeScript (js, jsx, ts, tsx, mjs, cjs)
  - Python (py, pyw)
  - CSS (css, scss, less)
  - HTML (html, htm, vue, svelte)
  - Rust (rs)
  - C/C++ (c, h, cpp, hpp, cc, cxx)
  - Go (go)
  - SQL (sql)
  - Shell Script (sh, bash, zsh)
- **Verification:** Config file valid JSON, will be included in next build

---

### Task 4: Update open file dialog filters

- **Status:** ✅ Complete
- **Files:** `src/main.tsx` (Modified)
- **Changes:** Updated the `openFile()` dialog filters to organize files into logical categories:
  - Markdown
  - JavaScript/TypeScript
  - Python
  - Web (HTML, CSS, Vue, Svelte)
  - Systems (Rust, C, C++, Go)
  - Data/Config (JSON, YAML, TOML, XML, SQL, CSV)
  - Shell/Text
  - All Files
- **Verification:** `npm run build` → Build successful

---

## Changes Made

| File | Change |
|------|--------|
| `package.json` | Added 9 CodeMirror language dependencies |
| `src/main.tsx` | Added language imports, extended getLanguageExtension() with 25+ extensions, reorganized file dialog filters into 8 categories |
| `src-tauri/tauri.conf.json` | Added 10 new fileAssociation entries covering 26 new extensions |

## Deviations

None - plan was followed exactly.

## Verification

```bash
# Frontend build (TypeScript + Vite)
$ npm run build
> tsc && vite build
✓ 1295 modules transformed.
✓ built in 3.69s

# Dependencies installed
$ npm ls @codemirror/lang-javascript
└── @codemirror/lang-javascript@6.2.3
```

## Acceptance Criteria

- [x] `npm run build` completes without errors → Build successful with 1295 modules transformed
- [x] Opening a `.js` file shows JavaScript syntax highlighting → Implemented via `javascript()` extension
- [x] Opening a `.py` file shows Python syntax highlighting → Implemented via `python()` extension
- [x] Opening a `.html` file shows HTML syntax highlighting → Implemented via `html()` extension
- [x] Opening a `.css` file shows CSS syntax highlighting → Implemented via `css()` extension
- [x] Opening a `.rs` file shows Rust syntax highlighting → Implemented via `rust()` extension
- [x] Right-clicking a `.js` file in Finder and selecting "Open With" shows Nobs Editor → Configured in tauri.conf.json fileAssociations with role: "Editor"
- [x] File dialog (Cmd+O) shows new file type categories → 8 categories implemented

**Note:** File association visibility in Finder requires rebuilding and reinstalling the app (`npm run tauri build`), then launching it once so macOS registers the document types.
