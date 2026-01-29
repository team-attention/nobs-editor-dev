## Implementation Notes

### Code Patterns to Follow
- Keep existing `unsafe` block structure
- Maintain `msg_send!` macro usage pattern
- Preserve the `#[cfg(target_os = "macos")]` conditional compilation

### Gotchas & Pitfalls
- **Two locations:** The behavior flag is set in TWO places - both must be updated
  - `bring_window_to_front()` function (line 35)
  - `.setup()` handler (line 104)
- **Flag values:** Use `1 << 1` not the raw value `2` for clarity

### Common Mistakes to Avoid
- Don't remove the FullScreenAuxiliary flag (1 << 8)
- Don't forget to update the comment explaining the flag
- Don't change other parts of the window activation code

### Debugging Tips
- If behavior doesn't change, ensure app was fully restarted (not just hidden)
- Use `cargo tauri dev` for quick iteration
- Check Console.app for any AppKit warnings
