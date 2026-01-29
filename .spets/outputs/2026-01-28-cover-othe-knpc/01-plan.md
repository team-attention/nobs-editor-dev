---
id: 2026-01-28-cover-othe-knpc
step: 01-plan
status: approved
updated_at: '2026-01-28T12:42:37.398Z'
---

# Plan: Support Additional Text File Extensions

## Summary

**Goal:** Expand JustViewer to open and view additional text-based file types beyond markdown (.md, .markdown, .txt).

**Architecture:** Add new file extensions to both the Tauri file association configuration and the frontend file dialog filter. Since BlockNote parses all content as markdown, non-markdown files will display as plain text paragraphs (acceptable for viewing text files).

**Tech Stack:** Tauri 2.x configuration (tauri.conf.json), React frontend (main.tsx)

## Codebase Analysis

### Current State

**Relevant Files Found:**
- `src-tauri/tauri.conf.json:33-39` - File association configuration defining supported extensions
- `src/main.tsx:66-69` - Frontend open file dialog filter configuration
- `README.md:25` - Documentation listing supported file types

**Existing Patterns to Follow:**
- `src-tauri/tauri.conf.json:35` - Array of extensions in fileAssociations
  - WHY: Shows exact format for adding new extensions
- `src/main.tsx:67` - Filter array structure with name and extensions
  - WHY: Shows exact format for open dialog filters

**Similar Features:**
- Current file handling at `src/main.tsx:29-51` - loadFile function handles reading any text file
  - WHY: No changes needed here - it already reads any text file and parses as markdown

**Testing Approach:**
- No existing test infrastructure found (no test files, no test commands in package.json)
- Manual QA will be required

### Key Findings

**File Structure:**
- Configuration in `src-tauri/tauri.conf.json`
- Frontend logic in `src/main.tsx`
- Documentation in `README.md`

**Dependencies:**
- External: @tauri-apps/plugin-dialog (already in use)
- Internal: No new dependencies needed

**Code Conventions:**
- Array formatting in JSON config files
- Inline arrays in TypeScript

**Integration Points:**
- Tauri bundle configuration (builds into app bundle Info.plist)
- Frontend dialog filters

**Existing Utilities to Reuse:**
- `readTextFile` from @tauri-apps/plugin-fs (already handles any text file)
- `editor.tryParseMarkdownToBlocks` (parses content regardless of file extension)

## Architecture Decisions

### Chosen Approach: Simple Extension List Addition

**How it works:**
1. Add new extensions to `tauri.conf.json` fileAssociations array
2. Add new filter group to frontend open dialog
3. Update README documentation

**Why this approach:**
- Follows existing pattern from `src-tauri/tauri.conf.json:35` - simple array addition
- Consistent with existing dialog filter structure at `src/main.tsx:66-69`
- No code logic changes needed - BlockNote already handles any text content
- YAGNI: No need for syntax highlighting or special handling for different file types

**Pattern References:**
- `src-tauri/tauri.conf.json:33-39` - File association config structure
  - **Extract:** Array format for extensions
  - **Apply to:** Adding new extensions to same array
- `src/main.tsx:66-69` - Dialog filter structure
  - **Extract:** Filter object format with name and extensions
  - **Apply to:** Adding new filter group

### Alternatives Considered

**Alternative 1: Separate File Associations per Type**
- Description: Create multiple fileAssociations entries, one per file type category
- Pros: Better Finder "Open With" menu organization
- Cons: More configuration, no functional benefit
- Not chosen because: Single "Text Files" association is simpler and sufficient

**Alternative 2: Add Syntax Highlighting**
- Description: Detect file type and apply syntax highlighting for code files
- Pros: Better viewing experience for code files
- Cons: Significant complexity, requires new dependencies, out of scope
- Not chosen because: Over-engineering - user asked to "cover" files, not enhance viewing

### Test Strategy

**Chosen approach:** Manual QA Only

**Rationale:** No existing test infrastructure exists. Adding tests for configuration changes provides minimal value - manual verification that files open correctly is sufficient.

## Task Breakdown

### Phase 1: Configuration Updates

#### Task 1: Add file extensions to Tauri config

**File:** `src-tauri/tauri.conf.json` (Modify)

**Changes:**
- Modify: `bundle.fileAssociations[0].ext` array at line 35
- Add extensions: `json`, `yaml`, `yml`, `toml`, `xml`, `log`, `ini`, `cfg`, `conf`, `csv`
- Update name from "Markdown Document" to "Text Document"

**Steps:**
1. Open `src-tauri/tauri.conf.json`
2. Locate `fileAssociations` array at line 33
3. Replace the `ext` array with expanded list: `["md", "markdown", "txt", "json", "yaml", "yml", "toml", "xml", "log", "ini", "cfg", "conf", "csv"]`
4. Change `name` from "Markdown Document" to "Text Document"
5. Save file

**Verification:**
- [ ] Run: `cat src-tauri/tauri.conf.json | grep -A5 fileAssociations`
- [ ] Expected: See updated ext array with all new extensions
- [ ] Check: JSON is valid (no syntax errors)

**Dependencies:** None

**Parallelizable:** YES (with Task 2)

**Commit Message:** `feat: add text file extensions to Tauri file associations`

---

#### Task 2: Update frontend file dialog filter

**File:** `src/main.tsx` (Modify)

**Changes:**
- Modify: `openFile` function filter array at line 66-69
- Add new filter group for text files
- Keep markdown filter as primary option

**Steps:**
1. Open `src/main.tsx`
2. Locate `filters` array in `openFile` function (around line 66)
3. Update filters to:
   ```typescript
   filters: [
     { name: "Markdown", extensions: ["md", "markdown"] },
     { name: "Text Files", extensions: ["txt", "json", "yaml", "yml", "toml", "xml", "log", "ini", "cfg", "conf", "csv"] },
     { name: "All Files", extensions: ["*"] }
   ]
   ```
4. Save file

**Verification:**
- [ ] Run: `npm run build`
- [ ] Expected: Build succeeds without errors
- [ ] Check: TypeScript compiles successfully

**Dependencies:** None

**Parallelizable:** YES (with Task 1)

**Commit Message:** `feat: add text file filters to open dialog`

---

### Phase 2: Documentation

#### Task 3: Update README with new supported extensions

**File:** `README.md` (Modify)

**Changes:**
- Modify: File Association bullet point at line 25
- Update list of supported file types

**Steps:**
1. Open `README.md`
2. Locate "File Association" feature line (around line 25)
3. Update text to: `**File Association** - Open `.md`, `.markdown`, `.txt`, `.json`, `.yaml`, `.yml`, `.toml`, `.xml`, `.log`, `.ini`, `.cfg`, `.conf`, `.csv` files directly`
4. Save file

**Verification:**
- [ ] Run: `cat README.md | grep "File Association"`
- [ ] Expected: See updated list of file extensions
- [ ] Check: Markdown formatting is correct

**Dependencies:** None

**Parallelizable:** YES (with Task 1, 2)

**Commit Message:** `docs: update README with supported text file extensions`

---

### Phase 3: Verification

#### Task 4: Manual testing of new file associations

**File:** N/A (Testing)

**Changes:**
- Test: Build app and verify file associations work

**Steps:**
1. Run `npm run tauri build` to build production app
2. Create test files with various extensions (.json, .yaml, .toml, etc.)
3. Right-click test files in Finder → "Open With" should show JustViewer
4. Double-click test files should open in JustViewer
5. Use Cmd+O in JustViewer to verify new filter groups appear

**Verification:**
- [ ] Run: `npm run tauri build`
- [ ] Expected: Build completes successfully
- [ ] Check: Files with new extensions can be opened

**Dependencies:** Requires Task 1, 2

**Parallelizable:** NO

**Commit Message:** N/A (verification only)

## Files to Modify

| File | Action | Description | Phase | Task # |
|------|--------|-------------|-------|--------|
| `src-tauri/tauri.conf.json` | Modify | Add file extensions to fileAssociations | 1 | 1 |
| `src/main.tsx` | Modify | Update open dialog filter | 1 | 2 |
| `README.md` | Modify | Update documentation | 2 | 3 |

## Testing Strategy

### Test Approach

**Strategy:** Manual QA Only

**Rationale:** Configuration changes don't benefit from automated tests. Manual verification ensures macOS file associations work correctly.

### Manual Verification

**Manual testing checklist:**
- [ ] Build app: `npm run tauri build`
- [ ] Create test.json file with sample JSON content
- [ ] Create test.yaml file with sample YAML content
- [ ] Create test.log file with sample log content
- [ ] Right-click test.json → Open With → JustViewer appears
- [ ] Double-click test.yaml → Opens in JustViewer
- [ ] Open JustViewer → Cmd+O → See "Text Files" filter option
- [ ] Select a .toml file → Opens and displays content
- [ ] Check console: No errors during file loading

**Environment:** macOS Development

## Acceptance Criteria

### Implementation Complete
- [ ] Task 1: `src-tauri/tauri.conf.json` updated with new extensions
- [ ] Task 2: `src/main.tsx` updated with new filter groups
- [ ] Task 3: `README.md` updated with documentation
- [ ] All files committed with proper messages

### Functionality
- [ ] Files with new extensions appear in Finder's "Open With" menu
- [ ] Files with new extensions can be double-clicked to open
- [ ] Open dialog shows new filter groups
- [ ] Content displays correctly in viewer (as plain text paragraphs)
- [ ] No console errors when opening new file types

### Non-Regression
- [ ] Existing .md files still open correctly
- [ ] Existing .txt files still open correctly
- [ ] Markdown rendering still works for .md files

**Status:** All criteria must be checked before considering feature complete

## Risks & Considerations

### Technical Risks

- **Risk 1**: macOS may cache file associations
  - **Impact:** Changes may not appear immediately after rebuild
  - **Mitigation:** Delete old app bundle before testing new build
  - **Contingency:** Run `lsregister -kill -r -domain local -domain system -domain user` to reset LaunchServices

- **Risk 2**: Some file extensions may conflict with system handlers
  - **Impact:** JustViewer may not appear as option for .json files if VS Code is default
  - **Mitigation:** This is expected behavior - user can manually select JustViewer
  - **Contingency:** N/A - this is macOS limitation, not a bug

### Edge Cases

- **Binary files with text extensions**
  - Scenario: User opens a .log file that's actually binary
  - Handling: Will display garbled content (acceptable - viewer makes no guarantees)
  - Test coverage: Manual testing

- **Large files**
  - Scenario: User opens a 100MB log file
  - Handling: May be slow to load (BlockNote limitation)
  - Test coverage: Out of scope - existing limitation

### Dependencies

**External Dependencies:**
- None new

**Internal Dependencies:**
- None new

**Breaking Changes:**
- None - purely additive change

### Performance Considerations

- **Performance Impact:** None
- No new code logic, just configuration changes

## Implementation Notes

### Code Patterns to Follow

**Pattern 1: Tauri fileAssociations format**
```json
// Reference: src-tauri/tauri.conf.json:33-39
"fileAssociations": [
  {
    "ext": ["md", "markdown", "txt", ...new extensions...],
    "name": "Text Document",
    "role": "Viewer"
  }
]
```

**Pattern 2: Dialog filter format**
```typescript
// Reference: src/main.tsx:66-69
filters: [
  { name: "Category Name", extensions: ["ext1", "ext2"] },
  { name: "All Files", extensions: ["*"] }
]
```

### Gotchas & Pitfalls

- **Gotcha 1**: File associations require app rebuild
  - Why it matters: Changes won't take effect until `npm run tauri build`
  - How to avoid: Always rebuild after modifying tauri.conf.json

- **Gotcha 2**: macOS caches file type associations
  - Why it matters: Old associations may persist after rebuild
  - How to avoid: Delete `/Applications/JustViewer.app` before installing new build

### Common Mistakes to Avoid

- ❌ Don't use dots in extension names (use "json" not ".json")
- ❌ Don't forget to rebuild app after config changes
- ✅ Do keep "All Files" option as last filter for flexibility

---

## Task Flow Visualization

```
Phase 1 (Configuration)
  Task 1 (tauri.conf.json)  ─┬─→ Phase 3
  Task 2 (main.tsx)         ─┤      ↓
                             │   Task 4 (verification)
Phase 2 (Documentation)      │
  Task 3 (README.md)        ─┘
```

## Estimated Effort

- **Total phases:** 3
- **Total tasks:** 4
- **Complexity:** Low
- **Parallelization opportunities:** Tasks 1, 2, 3 can run in parallel
