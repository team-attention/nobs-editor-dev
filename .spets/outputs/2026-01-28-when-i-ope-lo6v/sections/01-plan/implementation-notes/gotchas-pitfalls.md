### Gotchas & Pitfalls
- **Two locations:** The behavior flag is set in TWO places - both must be updated
  - `bring_window_to_front()` function (line 35)
  - `.setup()` handler (line 104)
- **Flag values:** Use `1 << 1` not the raw value `2` for clarity
