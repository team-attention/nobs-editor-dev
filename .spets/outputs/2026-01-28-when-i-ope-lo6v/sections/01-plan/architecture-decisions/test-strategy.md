**Chosen approach:** Manual QA Only

**Rationale:** macOS window behavior and Space management cannot be unit tested - these are OS-level behaviors that require visual verification on a real macOS system. The change is a single flag value modification with well-documented behavior from Apple's AppKit documentation.

**Manual Verification Steps:**
1. Build the app with `cargo tauri build`
2. Create a test markdown file
3. Open Terminal in Space 2 (not Desktop 1)
4. Run `open test.md` or click a file:// link
5. Verify JustViewer window appears on Space 2 (same as Terminal)
6. Repeat test from different Spaces to confirm consistent behavior
