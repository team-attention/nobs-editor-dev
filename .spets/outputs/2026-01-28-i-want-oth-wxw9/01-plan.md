---
id: 2026-01-28-i-want-oth-wxw9
step: 01-plan
status: approved
updated_at: '2026-01-28T13:00:26.298Z'
---

# Plan: Add CodeMirror editor for non-markdown file prettification

## Summary

**Goal:** Add syntax-highlighted, editable CodeMirror editor for non-markdown file types (JSON, YAML, TOML, XML, CSV, logs, config files) while keeping BlockNote for markdown files.

**Architecture:** Detect file extension in `loadFile()`, branch rendering to either BlockNote (markdown) or CodeMirror (everything else). CodeMirror provides syntax highlighting, line numbers, and editing. Save logic branches similarly based on file type.

**Tech Stack:** CodeMirror 6 (`@codemirror/view`, `@codemirror/state`, language packages), existing React/Tauri stack.

## Codebase Analysis

### Current State

**Relevant Files:**
- `src/main.tsx` - Single React component handling all UI. `loadFile()` at line 29 always calls `editor.tryParseMarkdownToBlocks(content)` (line 44). `saveFile()` at line 78 always converts via `editor.blocksToMarkdownLossy()` (line 82). Rendering at line 176 always shows `<BlockNoteView>`.
- `src/styles.css` - Theme variables with dark mode support via `prefers-color-scheme`. Editor container at line 130 with `max-width: 900px`.
- `package.json` - No testing framework configured. No CodeMirror dependency yet.

**Existing Patterns to Follow:**
- `src/main.tsx:29-52` - File loading pattern: `readTextFile` -> parse -> set state -> `setShowEditor(true)`
  - WHY: New CodeMirror loading must follow same async pattern with error handling
- `src/main.tsx:78-88` - File saving pattern: convert content -> `writeTextFile`
  - WHY: CodeMirror save must use same `writeTextFile` API
- `src/main.tsx:150-182` - UI structure: toolbar + content area with conditional rendering
  - WHY: CodeMirror view must slot into same content area pattern
- `src/styles.css:7-25` - CSS variable system for theming
  - WHY: CodeMirror theme must use same CSS variables for consistency

**Testing Approach:**
- No test framework exists in the project. Manual QA only.

### Key Findings

**File Structure:**
- Single-file React app (`src/main.tsx`), all logic inline
- No component extraction, no separate files for editors
- CSS in `src/styles.css` with CSS variables

**Dependencies:**
- External needed: `codemirror`, `@codemirror/view`, `@codemirror/state`, `@codemirror/lang-json`, `@codemirror/lang-xml`, `@codemirror/lang-yaml`, `@codemirror/language`, `@codemirror/theme-one-dark`
- Internal: existing `readTextFile`/`writeTextFile` from `@tauri-apps/plugin-fs`

**Code Conventions:**
- Inline React component with hooks (useState, useCallback, useEffect, useRef)
- No TypeScript interfaces extracted, all inline
- Simple CSS with BEM-ish naming (#editor-container, #toolbar)

**Integration Points:**
- `loadFile()` function needs branching logic based on file extension
- `saveFile()` function needs to handle CodeMirror content extraction
- Render section needs conditional CodeMirror vs BlockNote display
- Keyboard shortcut handler (Cmd+S) already wired to `saveFile()`

## Architecture Decisions

### Chosen Approach: Conditional Editor Rendering

**How it works:**
1. Extract file extension from path in `loadFile()`
2. Store `fileType` state: `"markdown"` or `"code"`
3. For markdown files: keep existing BlockNote flow unchanged
4. For non-markdown files: store raw content in state, render CodeMirror
5. CodeMirror mounts via `useRef` + `useEffect` pattern (imperative API)
6. Save branches: BlockNote -> markdown export, CodeMirror -> get doc text

**Why this approach:**
- Minimal changes to existing code (branching, not replacing)
- CodeMirror 6 has native React-compatible imperative API via `EditorView`
- Follows existing pattern of state-driven conditional rendering (`src/main.tsx:163-179`)
- Keeps markdown experience completely unchanged

**Pattern References:**
- `src/main.tsx:163-179` - Conditional rendering pattern (empty state vs editor)
  - **Extract:** Ternary-based conditional rendering in JSX
  - **Apply to:** Three-way conditional: empty state / BlockNote / CodeMirror
- `src/main.tsx:29-52` - Async file loading with error handling
  - **Extract:** try/catch pattern with state updates
  - **Apply to:** CodeMirror content loading

### Alternatives Considered

**Alternative 1: Replace BlockNote entirely with CodeMirror**
- Pros: Single editor, simpler code
- Cons: Loses Notion-style markdown editing experience
- Not chosen because: User explicitly wants to keep markdown experience

**Alternative 2: Use BlockNote code blocks for all non-markdown**
- Pros: No new dependency
- Cons: BlockNote code blocks lack line numbers, proper syntax highlighting, and editing UX for code
- Not chosen because: User chose CodeMirror for better editing experience

### Test Strategy

**Chosen approach:** Manual QA Only

**Rationale:** No test framework exists in the project. This is a UI-heavy change best verified visually.

## Task Breakdown

### Phase 1: Dependencies

#### Task 1: Install CodeMirror packages

**File:** `package.json`

**Changes:**
- Add CodeMirror 6 core and language packages

**Steps:**
1. Run: `npm install codemirror @codemirror/view @codemirror/state @codemirror/lang-json @codemirror/lang-xml @codemirror/lang-yaml @codemirror/language @codemirror/theme-one-dark @codemirror/commands @codemirror/autocomplete @codemirror/search @codemirror/lint`
2. Verify `package.json` updated

**Verification:**
- [ ] Run: `npm ls codemirror`
- [ ] Expected: codemirror package listed without errors
- [ ] Check: No peer dependency warnings

**Dependencies:** None

**Commit Message:** `feat: add CodeMirror 6 dependencies for code editing`

---

### Phase 2: Core Implementation

#### Task 2: Add file type detection and state management in `src/main.tsx`

**File:** `src/main.tsx` (Modify)

**Changes:**
- Add `fileType` state: `useState<"markdown" | "code">("markdown")`
- Add `codeContent` state: `useState<string>("")`
- Add helper function `getFileType(path: string): "markdown" | "code"` that checks extension
- Add helper function `getLanguageFromExt(ext: string)` that returns CodeMirror language extension

**Steps:**
1. Add new state variables after line 20
2. Add `getFileType()` helper before the App component (checks if extension is `.md` or `.markdown`)
3. Add `getLanguageFromExt()` helper that maps extensions to CodeMirror language support
4. Modify `loadFile()` to branch: if markdown, use existing BlockNote flow; if code, set `codeContent` and `fileType`

**Verification:**
- [ ] Run: `npm run build`
- [ ] Expected: No TypeScript errors
- [ ] Verify: `loadFile` handles both markdown and non-markdown paths

**Dependencies:** Requires Task 1

**Commit Message:** `feat: add file type detection and branching in loadFile`

---

#### Task 3: Add CodeMirror editor component rendering in `src/main.tsx`

**File:** `src/main.tsx` (Modify)

**Changes:**
- Add CodeMirror imports
- Add `editorRef` (useRef for DOM container) and `cmViewRef` (useRef for EditorView instance)
- Add useEffect to create/update CodeMirror EditorView when `fileType === "code"`
- Add conditional rendering: show BlockNote for markdown, CodeMirror div for code files

**Steps:**
1. Add CodeMirror imports at top of file
2. Add refs: `const cmContainerRef = useRef<HTMLDivElement>(null)` and `const cmViewRef = useRef<EditorView | null>(null)`
3. Add useEffect that creates EditorView when fileType is "code" and cmContainerRef is available
4. Configure EditorView with: line numbers, syntax highlighting, appropriate language, dark theme detection
5. Update JSX to render CodeMirror container div when `fileType === "code"`

**Verification:**
- [ ] Run: `npm run build`
- [ ] Expected: No TypeScript errors
- [ ] Verify: Opening a .json file shows CodeMirror with syntax highlighting

**Dependencies:** Requires Task 2

**Commit Message:** `feat: add CodeMirror rendering for non-markdown files`

---

#### Task 4: Update save logic for CodeMirror files in `src/main.tsx`

**File:** `src/main.tsx` (Modify)

**Changes:**
- Modify `saveFile()` to branch based on `fileType`
- For "code" files: extract text from CodeMirror's EditorView via `cmViewRef.current.state.doc.toString()`
- For "markdown" files: keep existing `blocksToMarkdownLossy` flow

**Steps:**
1. Update `saveFile()` callback to check `fileType`
2. If "code": get content from `cmViewRef.current?.state.doc.toString()`, write with `writeTextFile`
3. If "markdown": keep existing logic unchanged

**Verification:**
- [ ] Run: `npm run build`
- [ ] Expected: No TypeScript errors
- [ ] Verify: Edit a JSON file, press Cmd+S, reopen - changes persisted

**Dependencies:** Requires Task 3

**Commit Message:** `feat: add save support for CodeMirror-edited files`

---

### Phase 3: Styling

#### Task 5: Add CodeMirror styles in `src/styles.css`

**File:** `src/styles.css` (Modify)

**Changes:**
- Add `#codemirror-container` styles matching `#editor-container` layout
- Add dark mode CodeMirror theme integration using CSS variables
- Ensure CodeMirror fills available height

**Steps:**
1. Add `#codemirror-container` styles: full height, max-width 900px, margin auto, padding
2. Add `.cm-editor` overrides: height 100%, font-family monospace
3. Add dark mode media query for CodeMirror background/text colors

**Verification:**
- [ ] Run: `npm run dev` and open a JSON file
- [ ] Expected: CodeMirror matches app theme in both light and dark modes
- [ ] Verify: Editor fills content area properly

**Dependencies:** Requires Task 3

**Parallelizable:** YES (with Task 4)

**Commit Message:** `style: add CodeMirror container and theme styles`

---

### Phase 4: Polish

#### Task 6: Update empty state text and cleanup

**File:** `src/main.tsx` (Modify)

**Changes:**
- Update empty state text from "Open a markdown file" to "Open a file to get started" (line 172)
- Ensure CodeMirror EditorView is properly destroyed on cleanup (useEffect return)
- Ensure switching between files (markdown -> code -> markdown) works cleanly

**Steps:**
1. Update empty state text at line 172
2. Add cleanup in CodeMirror useEffect: `return () => { cmViewRef.current?.destroy(); }`
3. When switching from code to markdown, destroy CodeMirror view
4. When switching from markdown to code, ensure BlockNote is hidden

**Verification:**
- [ ] Run: `npm run dev`
- [ ] Expected: Open .md file (BlockNote), then .json file (CodeMirror), then another .md file (BlockNote) - all transitions clean
- [ ] Verify: No console errors during transitions

**Dependencies:** Requires Task 4, Task 5

**Commit Message:** `feat: polish editor transitions and update empty state text`

---

## Files to Modify

| File | Action | Description | Phase | Task # |
|------|--------|-------------|-------|--------|
| `package.json` | Modify | Add CodeMirror dependencies | 1 | 1 |
| `src/main.tsx` | Modify | Add file type detection, CodeMirror rendering, save branching | 2 | 2, 3, 4, 6 |
| `src/styles.css` | Modify | Add CodeMirror container and theme styles | 3 | 5 |

## Testing Strategy

### Manual Verification

**Test checklist:**
- [ ] Open `.md` file: BlockNote editor renders with Notion-style editing
- [ ] Open `.json` file: CodeMirror renders with JSON syntax highlighting and line numbers
- [ ] Open `.yaml` file: CodeMirror renders with YAML syntax highlighting
- [ ] Open `.xml` file: CodeMirror renders with XML syntax highlighting
- [ ] Open `.toml` file: CodeMirror renders with appropriate highlighting
- [ ] Open `.csv` file: CodeMirror renders with text content
- [ ] Open `.log` file: CodeMirror renders with text content
- [ ] Open `.ini`/`.cfg`/`.conf` file: CodeMirror renders
- [ ] Open `.txt` file: CodeMirror renders as plain text
- [ ] Edit a JSON file in CodeMirror, save with Cmd+S, reopen - changes persist
- [ ] Edit a markdown file in BlockNote, save with Cmd+S, reopen - changes persist
- [ ] Switch from .md to .json to .md - no visual glitches or console errors
- [ ] Dark mode: both editors match app theme
- [ ] Light mode: both editors match app theme
- [ ] Open file via toolbar button (all file types)
- [ ] Open file via right-click "Open With" in Finder
- [ ] Window resize: CodeMirror resizes properly

## Acceptance Criteria

- [ ] Non-markdown files render in CodeMirror with syntax highlighting
- [ ] Markdown files continue to render in BlockNote (unchanged)
- [ ] All supported file types (json, yaml, yml, toml, xml, csv, log, ini, cfg, conf, txt) open in CodeMirror
- [ ] CodeMirror shows line numbers
- [ ] Files are editable in CodeMirror
- [ ] Cmd+S saves CodeMirror content correctly
- [ ] Dark mode works for CodeMirror
- [ ] Switching between markdown and non-markdown files works cleanly
- [ ] Build succeeds: `npm run build`
- [ ] No console errors during normal usage

## Risks & Considerations

### Technical Risks

- **Risk 1**: CodeMirror bundle size increases app size
  - **Impact:** ~150KB additional JS
  - **Mitigation:** Use tree-shaking (Vite handles this). Only import needed language packages.
  - **Contingency:** Acceptable tradeoff for full editor functionality.

- **Risk 2**: CodeMirror dark mode may not perfectly match existing theme
  - **Impact:** Visual inconsistency between BlockNote and CodeMirror in dark mode
  - **Mitigation:** Use CSS variable overrides to match CodeMirror to app theme colors
  - **Contingency:** Use `@codemirror/theme-one-dark` as base, customize colors

### Edge Cases

- **Empty files**: CodeMirror handles empty documents natively - no special handling needed
- **Very large files**: CodeMirror virtualizes rendering, should handle large files well
- **Binary files**: Not in file association list, so won't be opened
- **Files without extension**: Will default to "code" type with plain text (no highlighting)

### Dependencies

**External Dependencies:**
- `codemirror` (v6) - Core editor
- `@codemirror/lang-json`, `@codemirror/lang-xml`, `@codemirror/lang-yaml` - Language support
- `@codemirror/theme-one-dark` - Dark theme base

## Implementation Notes

### Code Pattern for CodeMirror in React

```typescript
// Pattern: Imperative CodeMirror in React via useRef + useEffect
const cmContainerRef = useRef<HTMLDivElement>(null);
const cmViewRef = useRef<EditorView | null>(null);

useEffect(() => {
  if (fileType !== "code" || !cmContainerRef.current) return;

  // Destroy previous instance
  cmViewRef.current?.destroy();

  const view = new EditorView({
    state: EditorState.create({
      doc: codeContent,
      extensions: [
        lineNumbers(),
        highlightActiveLineGutter(),
        syntaxHighlighting(defaultHighlightStyle, { fallback: true }),
        getLanguageExtension(filename),
        // dark mode conditional
        ...(isDarkMode ? [oneDark] : []),
        keymap.of([...defaultKeymap, indentWithTab]),
      ],
    }),
    parent: cmContainerRef.current,
  });

  cmViewRef.current = view;

  return () => view.destroy();
}, [fileType, codeContent]);
```

### Extension to Language Mapping

```typescript
function getLanguageFromExt(ext: string) {
  switch (ext) {
    case "json": return json();
    case "yaml": case "yml": return yaml();
    case "xml": case "toml": return xml(); // TOML has no official CM package, use plain
    case "csv": case "log": case "txt": case "ini": case "cfg": case "conf":
    default: return [];
  }
}
```

### Gotchas

- CodeMirror EditorView must be destroyed before creating a new one (memory leak)
- Dark mode detection: use `window.matchMedia('(prefers-color-scheme: dark)').matches`
- CodeMirror's `doc` is immutable - get content via `view.state.doc.toString()`

## Task Flow Visualization

```
Task 1 (Install deps)
  ↓
Task 2 (File type detection + state)
  ↓
Task 3 (CodeMirror rendering)
  ↓
Task 4 (Save logic)    Task 5 (Styles) [parallel]
  ↓                      ↓
Task 6 (Polish + cleanup)
```

## Estimated Effort

- **Total phases:** 4
- **Total tasks:** 6
- **Complexity:** Medium
- **Parallelization:** Tasks 4 and 5 can run in parallel
