**Goal:** Make JustViewer window appear on the same macOS Space as the terminal when opening files via command line.

**Architecture:** Change `NSWindowCollectionBehavior` from `CanJoinAllSpaces` to `MoveToActiveSpace` in the `bring_window_to_front()` function. This single-flag change aligns with how Arc browser handles window placement.

**Tech Stack:** Rust, Tauri, macOS AppKit (cocoa crate), NSWindow APIs
