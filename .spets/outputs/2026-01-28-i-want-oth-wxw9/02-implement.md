---
id: 2026-01-28-i-want-oth-wxw9
step: 02-implement
status: approved
updated_at: '2026-01-28T13:05:09.655Z'
---

# Implementation: Add CodeMirror editor for non-markdown file prettification

## Summary

Added CodeMirror 6 as an editable code editor with syntax highlighting for non-markdown files (JSON, YAML, XML, and plain text types). Markdown files continue to use BlockNote. File type is detected by extension, and the UI conditionally renders the appropriate editor.

## Changes Made

### Task 1: Install CodeMirror packages
- Installed 12 CodeMirror packages: `codemirror`, `@codemirror/view`, `@codemirror/state`, `@codemirror/lang-json`, `@codemirror/lang-xml`, `@codemirror/lang-yaml`, `@codemirror/language`, `@codemirror/theme-one-dark`, `@codemirror/commands`, `@codemirror/autocomplete`, `@codemirror/search`, `@codemirror/lint`
- 20 packages added total (including transitive deps)

### Task 2: Add file type detection and state management
- Added `getFileType(path)` helper function that returns `"markdown"` for `.md`/`.markdown` files, `"code"` for everything else
- Added `getLanguageExtension(filename)` helper that maps file extensions to CodeMirror language support (JSON, YAML, XML)
- Added `fileType` and `codeContent` state variables
- Modified `loadFile()` to branch: markdown files go through BlockNote, code files store raw content in state

### Task 3: Add CodeMirror editor rendering
- Added CodeMirror imports (EditorView, EditorState, language extensions, theme)
- Added `cmContainerRef` and `cmViewRef` refs
- Added useEffect to create/destroy CodeMirror EditorView instances
- CodeMirror configured with: line numbers, active line gutter, fold gutter, bracket matching, selection highlighting, syntax highlighting, line wrapping, keyboard shortcuts (indent with tab)
- Dark mode: uses `oneDark` theme when `prefers-color-scheme: dark` matches
- Updated JSX to three-way conditional: empty state / BlockNote / CodeMirror

### Task 4: Update save logic
- Modified `saveFile()` to branch based on `fileType`
- Code files: extracts text via `cmViewRef.current.state.doc.toString()`
- Markdown files: unchanged `blocksToMarkdownLossy` flow

### Task 5: Add CodeMirror styles
- Added `#codemirror-container` styles matching editor-container layout (max-width 900px, centered)
- Added `.cm-editor` styles: monospace font (SF Mono/Fira Code/Menlo), 14px, rounded corners
- Added gutter styling using CSS variables for theme consistency
- Added dark mode override for CodeMirror background

### Task 6: Polish and cleanup
- Updated empty state text: "Open a file to get started" (was "Open a markdown file to get started")
- CodeMirror cleanup handled in useEffect return (destroy on unmount/re-render)
- setTimeout(0) used to ensure container is rendered before EditorView creation

## Files Modified

| File | Changes |
|------|---------|
| `package.json` | Added 12 CodeMirror dependencies |
| `src/main.tsx` | Added file type detection, CodeMirror rendering, save branching, updated empty state text |
| `src/styles.css` | Added CodeMirror container and theme styles |

## Verification

- [x] TypeScript compiles: `npx tsc --noEmit` - no errors
- [x] Vite build succeeds: `npx vite build` - built in 2.92s
- [x] No linting errors
- [x] Existing markdown functionality preserved (BlockNote rendering unchanged)

## Deviations from Plan

- Used `setTimeout(0)` in CodeMirror useEffect to ensure the DOM container is rendered before creating EditorView. This is a common React pattern for imperative APIs that need a mounted DOM element.
- TOML files render as plain text (no syntax highlighting) since there's no official CodeMirror TOML package. The plan noted this as a known limitation.

## Key Decisions

1. **Three-way conditional rendering** instead of two separate toggles - cleaner state management
2. **`EditorView.lineWrapping`** enabled for better readability of long lines in config files
3. **Font stack**: SF Mono > Fira Code > Fira Mono > Menlo > Consolas - prioritizes macOS native monospace font
