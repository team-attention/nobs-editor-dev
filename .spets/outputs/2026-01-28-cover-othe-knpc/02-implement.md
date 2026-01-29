---
id: 2026-01-28-cover-othe-knpc
step: 02-implement
status: approved
updated_at: '2026-01-28T12:46:18.912Z'
---

# Implementation: Support Additional Text File Extensions

## Summary

Expanded JustViewer to support additional text-based file types. Added 10 new file extensions (json, yaml, yml, toml, xml, log, ini, cfg, conf, csv) to both the Tauri file associations and the frontend file dialog filters.

## Changes Made

### Task 1: Add file extensions to Tauri config

**File:** `src-tauri/tauri.conf.json`

**Changes:**
- Extended `bundle.fileAssociations[0].ext` array from 3 to 13 extensions
- Changed association name from "Markdown Document" to "Text Document"
- Added: `json`, `yaml`, `yml`, `toml`, `xml`, `log`, `ini`, `cfg`, `conf`, `csv`

**Verification:**
- [x] JSON is valid (no syntax errors)
- [x] Build succeeds

### Task 2: Update frontend file dialog filter

**File:** `src/main.tsx`

**Changes:**
- Split "Markdown" filter to only include `md`, `markdown` extensions
- Added new "Text Files" filter group with `txt`, `json`, `yaml`, `yml`, `toml`, `xml`, `log`, `ini`, `cfg`, `conf`, `csv`
- Kept "All Files" filter as last option

**Verification:**
- [x] TypeScript compiles: `npm run build` succeeds
- [x] No type errors

### Task 3: Update README documentation

**File:** `README.md`

**Changes:**
- Updated "File Association" feature line to include all supported file types

**Verification:**
- [x] Markdown formatting correct

## Build Verification

```
> tsc && vite build
✓ 1262 modules transformed.
✓ built in 2.76s
```

All tasks completed successfully. Build passes without errors.

## Deviations

None. All changes followed the approved plan exactly.

## Files Modified

| File | Changes |
|------|---------|
| `src-tauri/tauri.conf.json` | Added 10 file extensions, renamed association |
| `src/main.tsx` | Split dialog filters into Markdown + Text Files groups |
| `README.md` | Updated supported file types list |
