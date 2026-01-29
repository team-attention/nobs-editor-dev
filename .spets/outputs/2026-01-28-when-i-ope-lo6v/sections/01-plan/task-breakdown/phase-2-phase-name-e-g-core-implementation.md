### Phase 2: Build & Manual Verification

#### Task 3: Build and Test on macOS

**File:** N/A (Build verification)

**Steps:**
1. Run `cd src-tauri && cargo build`
2. If build succeeds, run `cargo tauri dev` to test locally
3. Create a test markdown file in a known location
4. Open Terminal, switch to Space 2 (or any non-Desktop Space)
5. Run `open /path/to/test.md`
6. Observe: JustViewer should appear on Space 2, not switch to Desktop 1

**Verification:**
- [ ] Run: `cd src-tauri && cargo build`
- [ ] Expected: Build completes without errors
- [ ] Manual: App window appears on same Space as terminal

**Dependencies:** Task 1, Task 2

**Parallelizable:** NO

---
