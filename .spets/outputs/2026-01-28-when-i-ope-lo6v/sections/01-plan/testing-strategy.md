## Testing Strategy

### Test Approach
Manual QA only - macOS window/Space behavior cannot be automated through unit or integration tests.

### Unit Tests
N/A - No unit tests possible for OS-level window behavior.

### Integration Tests
N/A - Window management is platform-specific and requires visual verification.

### Manual Verification
1. **Basic Space Test:** Open Terminal in Space 2, run `open /path/to/test.md`, verify JustViewer appears on Space 2
2. **File Link Test:** Click a `file://` link in terminal, verify app appears on same Space
3. **Fullscreen Test:** Enter fullscreen mode in another app, open a markdown file, verify JustViewer appears above fullscreen
4. **Regression Tests:** Verify hide-on-close still works, Dock activation works, manual window movement between Spaces
