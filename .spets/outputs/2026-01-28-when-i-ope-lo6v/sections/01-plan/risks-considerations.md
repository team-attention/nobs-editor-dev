## Risks & Considerations

### Technical Risks
- **Low:** Flag change is well-documented in Apple's AppKit documentation
- **Low:** cocoa crate has stable support for these APIs

### Edge Cases
- **App already running on different Space:** MoveToActiveSpace will move the existing window
- **Multiple displays:** Behavior should work across displays (same Space concept applies)
- **Stage Manager enabled:** May need separate testing with Stage Manager on macOS Ventura+

### Dependencies
- macOS 10.15+ (already required minimum)
- No new dependencies needed

### Performance Considerations
- No performance impact - flag is set once during window setup
- Window movement is handled by macOS, not application code
