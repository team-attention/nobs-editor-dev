---
status: approved
updated_at: '2026-01-27T12:00:12.248Z'
---
# Plan: JustViewer - Markdown File Viewer for macOS

## Summary

**Goal:** Build a native macOS markdown viewer app that displays above fullscreen apps with Notion-like prettified view and raw markdown editing capabilities.

**Architecture:** Native Swift/SwiftUI macOS application with panel-style window that floats above all windows including fullscreen apps. Uses a split or toggle view for rendered markdown and raw editing mode. Integrates with macOS file system for Finder "Open With", file scheme URLs from terminal, and in-app Open dialog.

**Tech Stack:** Swift 5.9+, SwiftUI, AppKit (for panel window), WebKit (for markdown rendering), Combine (for state management)

## Codebase Analysis

### Current State

This is a greenfield project with no existing source code. The project contains only the Spets workflow configuration files.

**Relevant Files Found:**
- `.spets/` - Workflow configuration (not application code)
- No existing Swift, Xcode, or application files

**Existing Patterns to Follow:**
- N/A - Greenfield project. Will establish patterns during implementation.

**Similar Features:**
- N/A - New project.

**Testing Approach:**
- Testing library: XCTest (native Apple testing framework)
- Test location: `JustViewerTests/` (Xcode standard)
- UI tests: `JustViewerUITests/` (Xcode standard)

### Key Findings

**File Structure (to be created):**
```
JustViewer/
├── JustViewer.xcodeproj/
├── JustViewer/
│   ├── JustViewerApp.swift          # App entry point
│   ├── AppDelegate.swift            # NSApplicationDelegate for lifecycle
│   ├── Info.plist                   # App configuration, URL schemes, file types
│   ├── JustViewer.entitlements      # Sandbox entitlements
│   ├── Models/
│   │   └── MarkdownDocument.swift   # Document model
│   ├── Views/
│   │   ├── ContentView.swift        # Main container view
│   │   ├── MarkdownRendererView.swift # WebKit-based markdown preview
│   │   ├── MarkdownEditorView.swift   # Raw markdown editor
│   │   └── ToolbarView.swift          # View toggle, file actions
│   ├── ViewModels/
│   │   └── DocumentViewModel.swift   # Document state management
│   ├── Services/
│   │   ├── MarkdownParser.swift      # Markdown to HTML conversion
│   │   └── FileService.swift         # File I/O operations
│   └── Resources/
│       ├── Assets.xcassets           # App icons, colors
│       └── markdown.css              # Styling for rendered markdown
├── JustViewerTests/
│   └── MarkdownParserTests.swift
└── JustViewerUITests/
    └── JustViewerUITests.swift
```

**Dependencies:**
- External: None initially (may add `swift-markdown` or `Ink` for markdown parsing)
- Internal: AppKit, SwiftUI, WebKit, Combine, UniformTypeIdentifiers

**Code Conventions:**
- Naming: Swift standard (camelCase properties/methods, PascalCase types)
- File organization: One primary type per file
- Error handling: Swift `Result` type and `throws` functions

**Integration Points:**
- macOS file system via NSDocument or manual file handling
- URL scheme handling via AppDelegate
- Finder integration via Info.plist document types

## Architecture Decisions

### Chosen Approach: NSPanel-based Floating Window with SwiftUI Content

**How it works:**
1. Use `NSPanel` with `.floating` collection behavior to display above fullscreen apps
2. SwiftUI `ContentView` embedded in the NSPanel for modern UI development
3. WebKit `WKWebView` wrapped in SwiftUI for rendering markdown as HTML
4. Native `TextEditor` for raw markdown editing
5. Toggle between rendered and edit mode via toolbar button
6. File opening via:
   - `NSOpenPanel` for in-app "Open" dialog
   - `application(_:open:)` delegate for file scheme URLs
   - Info.plist document type registration for Finder "Open With"

**Why this approach:**
- NSPanel with floating behavior is the only reliable way to appear above fullscreen apps
- SwiftUI provides modern declarative UI with less boilerplate
- WebKit rendering allows full CSS styling for Notion-like appearance
- Native macOS integration for file handling is seamless

**Pattern References:**
- Apple documentation: NSPanel with `NSWindow.Level.floating`
- SwiftUI + AppKit integration via NSViewRepresentable
- URL scheme handling via NSApplicationDelegate

### Alternatives Considered

**Alternative 1: Pure SwiftUI Window**
- Description: Use SwiftUI's native Window and WindowGroup
- Pros: Simpler code, fully declarative
- Cons: Cannot reliably float above fullscreen apps; limited window customization
- Not chosen because: Cannot achieve the overlay requirement

**Alternative 2: Electron + React**
- Description: Web-based desktop app with Electron
- Pros: Cross-platform, familiar web technologies
- Cons: Large bundle (~150MB+), may not reliably overlay fullscreen, slower startup
- Not chosen because: User chose native macOS, and overlay support is critical

**Alternative 3: Menu Bar App with Popover**
- Description: Lives in menu bar, shows popover for content
- Pros: Non-intrusive, always accessible
- Cons: Popover has limited size, not suitable for document editing, poor UX for file viewer
- Not chosen because: Not suitable for document viewing/editing workflow

### Test Strategy

**Chosen approach:** TDD for core logic (MarkdownParser, FileService), UI tests for integration

**Rationale:** Markdown parsing is pure logic suitable for TDD. UI components are better tested via XCUITest for realistic user flows.

## Task Breakdown

### Phase 1: Project Setup & Infrastructure

#### Task 1: Create Xcode project with SwiftUI App template

**File:** `JustViewer.xcodeproj` (Create)

**Changes:**
- Create new Xcode project: macOS App, SwiftUI interface, Swift language
- Product name: JustViewer
- Bundle identifier: com.yourname.JustViewer
- Set deployment target: macOS 13.0+

**Steps:**
1. Open Xcode, File > New > Project
2. Select macOS > App template
3. Configure: Product Name: JustViewer, Interface: SwiftUI, Language: Swift
4. Save to `/Users/eatnug/Workspace/JustViewer/`
5. Commit: "chore: initialize Xcode project"

**Verification:**
- [ ] Run: Open project in Xcode, Cmd+R to build and run
- [ ] Expected: Empty SwiftUI window appears
- [ ] Check: Project compiles without errors

**Dependencies:** None

**Parallelizable:** NO

**Commit Message:** `chore: initialize Xcode project with SwiftUI template`

---

#### Task 2: Configure Info.plist for file type associations

**File:** `JustViewer/Info.plist` (Modify)

**Changes:**
- Add CFBundleDocumentTypes for markdown files (.md, .markdown, .mdown)
- Add UTI declarations for markdown types
- Add CFBundleURLTypes for custom URL scheme

**Steps:**
1. Open Info.plist in Xcode
2. Add Document Types array with markdown extensions
3. Add Imported Type Identifiers for markdown UTI
4. Add URL Types with scheme "justviewer"
5. Commit: "feat: configure file type and URL scheme associations"

**Code sample:**
```xml
<key>CFBundleDocumentTypes</key>
<array>
    <dict>
        <key>CFBundleTypeName</key>
        <string>Markdown Document</string>
        <key>CFBundleTypeRole</key>
        <string>Editor</string>
        <key>LSItemContentTypes</key>
        <array>
            <string>net.daringfireball.markdown</string>
            <string>public.text</string>
        </array>
        <key>LSHandlerRank</key>
        <string>Alternate</string>
    </dict>
</array>
<key>CFBundleURLTypes</key>
<array>
    <dict>
        <key>CFBundleURLName</key>
        <string>com.yourname.JustViewer</string>
        <key>CFBundleURLSchemes</key>
        <array>
            <string>justviewer</string>
        </array>
    </dict>
</array>
```

**Verification:**
- [ ] Run: Build project (Cmd+B)
- [ ] Expected: No build errors
- [ ] Check: Info.plist contains document types and URL schemes

**Dependencies:** Task 1

**Parallelizable:** NO

**Commit Message:** `feat: configure Info.plist for markdown files and URL scheme`

---

#### Task 3: Add App Sandbox entitlements for file access

**File:** `JustViewer/JustViewer.entitlements` (Modify)

**Changes:**
- Enable App Sandbox
- Add read-write access to user-selected files
- Add file access for opened documents

**Steps:**
1. Select JustViewer target > Signing & Capabilities
2. Add "App Sandbox" capability if not present
3. Enable "User Selected File" with Read/Write access
4. Commit: "feat: configure sandbox entitlements for file access"

**Verification:**
- [ ] Run: Build project (Cmd+B)
- [ ] Expected: No build errors
- [ ] Check: Entitlements file contains sandbox keys

**Dependencies:** Task 1

**Parallelizable:** YES (with Task 2)

**Commit Message:** `feat: configure sandbox entitlements for file access`

---

### Phase 2: Core Window & App Architecture

#### Task 4: Create AppDelegate for window management

**File:** `JustViewer/AppDelegate.swift` (Create)

**Changes:**
- Create NSApplicationDelegate class
- Set up floating panel window
- Handle URL open events
- Handle file open events

**Steps:**
1. Create new Swift file: AppDelegate.swift
2. Implement NSApplicationDelegate protocol
3. Create NSPanel in applicationDidFinishLaunching
4. Implement application(_:open:) for URL/file handling
5. Commit: "feat: implement AppDelegate with floating panel window"

**Code sample:**
```swift
import AppKit
import SwiftUI

class AppDelegate: NSObject, NSApplicationDelegate {
    var panel: NSPanel!
    var hostingView: NSHostingView<ContentView>!

    func applicationDidFinishLaunching(_ notification: Notification) {
        let contentView = ContentView()
        hostingView = NSHostingView(rootView: contentView)

        panel = NSPanel(
            contentRect: NSRect(x: 0, y: 0, width: 800, height: 600),
            styleMask: [.titled, .closable, .resizable, .nonactivatingPanel],
            backing: .buffered,
            defer: false
        )
        panel.level = .floating
        panel.collectionBehavior = [.canJoinAllSpaces, .fullScreenAuxiliary]
        panel.contentView = hostingView
        panel.center()
        panel.makeKeyAndOrderFront(nil)
    }

    func application(_ application: NSApplication, open urls: [URL]) {
        // Handle file/URL opens
    }
}
```

**Verification:**
- [ ] Run: Cmd+R to build and run
- [ ] Expected: Floating panel window appears
- [ ] Check: Window stays above other windows

**Dependencies:** Task 1

**Parallelizable:** NO

**Commit Message:** `feat: implement AppDelegate with floating panel window`

---

#### Task 5: Connect AppDelegate to SwiftUI App

**File:** `JustViewer/JustViewerApp.swift` (Modify)

**Changes:**
- Add @NSApplicationDelegateAdaptor to use AppDelegate
- Remove default WindowGroup (panel handles window)

**Steps:**
1. Open JustViewerApp.swift
2. Add NSApplicationDelegateAdaptor property wrapper
3. Replace body with EmptyCommands or minimal setup
4. Commit: "feat: connect AppDelegate to SwiftUI App lifecycle"

**Code sample:**
```swift
import SwiftUI

@main
struct JustViewerApp: App {
    @NSApplicationDelegateAdaptor(AppDelegate.self) var appDelegate

    var body: some Scene {
        Settings {
            EmptyView()
        }
    }
}
```

**Verification:**
- [ ] Run: Cmd+R to build and run
- [ ] Expected: App launches with floating panel
- [ ] Check: No duplicate windows appear

**Dependencies:** Task 4

**Parallelizable:** NO

**Commit Message:** `feat: connect AppDelegate to SwiftUI App lifecycle`

---

#### Task 6: Test overlay behavior above fullscreen apps

**File:** N/A - Manual testing

**Changes:**
- Verify panel appears above fullscreen applications

**Steps:**
1. Build and run JustViewer
2. Open another app (e.g., Safari) and enter fullscreen (Ctrl+Cmd+F)
3. Verify JustViewer panel is still visible above fullscreen app
4. Test resizing and moving the panel
5. Document any issues

**Verification:**
- [ ] Run: Manual test as described
- [ ] Expected: JustViewer panel floats above fullscreen Safari
- [ ] Check: Panel is interactive while above fullscreen app

**Dependencies:** Task 5

**Parallelizable:** NO

**Commit Message:** N/A (manual test, no code changes)

---

### Phase 3: Document Model & File Handling

#### Task 7: Create MarkdownDocument model

**File:** `JustViewer/Models/MarkdownDocument.swift` (Create)

**Changes:**
- Create MarkdownDocument struct/class
- Properties: fileURL, content, title, isModified
- Implement Identifiable, ObservableObject

**Steps:**
1. Create Models folder in JustViewer group
2. Create MarkdownDocument.swift
3. Implement properties and initializers
4. Commit: "feat: add MarkdownDocument model"

**Code sample:**
```swift
import Foundation
import Combine

class MarkdownDocument: ObservableObject, Identifiable {
    let id = UUID()
    var fileURL: URL?
    @Published var content: String
    @Published var isModified: Bool = false

    var title: String {
        fileURL?.lastPathComponent ?? "Untitled"
    }

    init(content: String = "", fileURL: URL? = nil) {
        self.content = content
        self.fileURL = fileURL
    }
}
```

**Verification:**
- [ ] Run: Build project (Cmd+B)
- [ ] Expected: No build errors
- [ ] Check: MarkdownDocument.swift exists in Models group

**Dependencies:** Task 1

**Parallelizable:** YES (with Tasks 4-6)

**Commit Message:** `feat: add MarkdownDocument model`

---

#### Task 8: Create FileService for file I/O

**File:** `JustViewer/Services/FileService.swift` (Create)

**Changes:**
- Create FileService struct
- Implement loadDocument(from:) throws -> MarkdownDocument
- Implement saveDocument(_:) throws

**Steps:**
1. Create Services folder in JustViewer group
2. Create FileService.swift
3. Implement file reading with String(contentsOf:)
4. Implement file writing with String.write(to:)
5. Commit: "feat: add FileService for file I/O operations"

**Code sample:**
```swift
import Foundation

struct FileService {
    enum FileError: Error {
        case readFailed(URL)
        case writeFailed(URL)
    }

    func loadDocument(from url: URL) throws -> MarkdownDocument {
        let content = try String(contentsOf: url, encoding: .utf8)
        return MarkdownDocument(content: content, fileURL: url)
    }

    func saveDocument(_ document: MarkdownDocument) throws {
        guard let url = document.fileURL else {
            throw FileError.writeFailed(URL(fileURLWithPath: ""))
        }
        try document.content.write(to: url, atomically: true, encoding: .utf8)
        document.isModified = false
    }
}
```

**Verification:**
- [ ] Run: Build project (Cmd+B)
- [ ] Expected: No build errors
- [ ] Check: FileService.swift exists in Services group

**Dependencies:** Task 7

**Parallelizable:** NO

**Commit Message:** `feat: add FileService for file I/O operations`

---

#### Task 9: Write unit tests for FileService

**File:** `JustViewerTests/FileServiceTests.swift` (Create)

**Changes:**
- Create test class FileServiceTests
- Test loadDocument with valid markdown file
- Test loadDocument with non-existent file (throws)
- Test saveDocument

**Steps:**
1. Create FileServiceTests.swift in JustViewerTests target
2. Write test for loading valid file
3. Write test for loading non-existent file
4. Write test for saving document
5. Commit: "test: add FileService unit tests"

**Code sample:**
```swift
import XCTest
@testable import JustViewer

final class FileServiceTests: XCTestCase {
    var fileService: FileService!
    var tempDirectory: URL!

    override func setUp() {
        fileService = FileService()
        tempDirectory = FileManager.default.temporaryDirectory
    }

    func testLoadDocument_ValidFile_ReturnsDocument() throws {
        // Create temp file
        let testURL = tempDirectory.appendingPathComponent("test.md")
        try "# Hello World".write(to: testURL, atomically: true, encoding: .utf8)

        let document = try fileService.loadDocument(from: testURL)

        XCTAssertEqual(document.content, "# Hello World")
        XCTAssertEqual(document.title, "test.md")

        try FileManager.default.removeItem(at: testURL)
    }

    func testLoadDocument_NonExistentFile_Throws() {
        let badURL = tempDirectory.appendingPathComponent("nonexistent.md")

        XCTAssertThrowsError(try fileService.loadDocument(from: badURL))
    }
}
```

**Verification:**
- [ ] Run: Cmd+U to run tests
- [ ] Expected: All tests pass
- [ ] Check: Test coverage includes load and error cases

**Dependencies:** Task 8

**Parallelizable:** NO

**Commit Message:** `test: add FileService unit tests`

---

### Phase 4: Markdown Rendering

#### Task 10: Create MarkdownParser service

**File:** `JustViewer/Services/MarkdownParser.swift` (Create)

**Changes:**
- Create MarkdownParser struct
- Implement parse(_:) -> String (returns HTML)
- Use basic regex-based conversion or AttributedString

**Steps:**
1. Create MarkdownParser.swift in Services folder
2. Implement basic markdown-to-HTML conversion
3. Handle headers, bold, italic, links, code blocks, lists
4. Commit: "feat: add MarkdownParser for markdown-to-HTML conversion"

**Code sample:**
```swift
import Foundation

struct MarkdownParser {
    func parse(_ markdown: String) -> String {
        var html = markdown

        // Headers
        html = html.replacingOccurrences(of: "(?m)^### (.+)$", with: "<h3>$1</h3>", options: .regularExpression)
        html = html.replacingOccurrences(of: "(?m)^## (.+)$", with: "<h2>$1</h2>", options: .regularExpression)
        html = html.replacingOccurrences(of: "(?m)^# (.+)$", with: "<h1>$1</h1>", options: .regularExpression)

        // Bold
        html = html.replacingOccurrences(of: "\\*\\*(.+?)\\*\\*", with: "<strong>$1</strong>", options: .regularExpression)

        // Italic
        html = html.replacingOccurrences(of: "\\*(.+?)\\*", with: "<em>$1</em>", options: .regularExpression)

        // Code blocks
        html = html.replacingOccurrences(of: "```([\\s\\S]*?)```", with: "<pre><code>$1</code></pre>", options: .regularExpression)

        // Inline code
        html = html.replacingOccurrences(of: "`(.+?)`", with: "<code>$1</code>", options: .regularExpression)

        // Links
        html = html.replacingOccurrences(of: "\\[(.+?)\\]\\((.+?)\\)", with: "<a href=\"$2\">$1</a>", options: .regularExpression)

        // Paragraphs (simple approach)
        html = html.components(separatedBy: "\n\n").map { "<p>\($0)</p>" }.joined()

        return html
    }
}
```

**Verification:**
- [ ] Run: Build project (Cmd+B)
- [ ] Expected: No build errors
- [ ] Check: MarkdownParser.swift exists in Services group

**Dependencies:** Task 1

**Parallelizable:** YES (with Tasks 7-9)

**Commit Message:** `feat: add MarkdownParser for markdown-to-HTML conversion`

---

#### Task 11: Write unit tests for MarkdownParser

**File:** `JustViewerTests/MarkdownParserTests.swift` (Create)

**Changes:**
- Create test class MarkdownParserTests
- Test header parsing (h1, h2, h3)
- Test bold and italic
- Test code blocks
- Test links

**Steps:**
1. Create MarkdownParserTests.swift in JustViewerTests target
2. Write tests for each markdown element
3. Commit: "test: add MarkdownParser unit tests"

**Code sample:**
```swift
import XCTest
@testable import JustViewer

final class MarkdownParserTests: XCTestCase {
    var parser: MarkdownParser!

    override func setUp() {
        parser = MarkdownParser()
    }

    func testParse_Header1_ReturnsH1() {
        let result = parser.parse("# Hello")
        XCTAssertTrue(result.contains("<h1>Hello</h1>"))
    }

    func testParse_Bold_ReturnsStrong() {
        let result = parser.parse("**bold text**")
        XCTAssertTrue(result.contains("<strong>bold text</strong>"))
    }

    func testParse_Italic_ReturnsEm() {
        let result = parser.parse("*italic text*")
        XCTAssertTrue(result.contains("<em>italic text</em>"))
    }

    func testParse_Link_ReturnsAnchor() {
        let result = parser.parse("[Click here](https://example.com)")
        XCTAssertTrue(result.contains("<a href=\"https://example.com\">Click here</a>"))
    }
}
```

**Verification:**
- [ ] Run: Cmd+U to run tests
- [ ] Expected: All tests pass
- [ ] Check: Tests cover headers, bold, italic, links

**Dependencies:** Task 10

**Parallelizable:** NO

**Commit Message:** `test: add MarkdownParser unit tests`

---

#### Task 12: Add CSS stylesheet for Notion-like rendering

**File:** `JustViewer/Resources/markdown.css` (Create)

**Changes:**
- Create CSS file for markdown styling
- Style headers, paragraphs, code blocks
- Use Notion-inspired typography and colors

**Steps:**
1. Create Resources folder in JustViewer group
2. Create markdown.css file
3. Add styling for all markdown elements
4. Commit: "feat: add Notion-inspired CSS for markdown rendering"

**Code sample:**
```css
:root {
    --text-color: #37352f;
    --background-color: #ffffff;
    --code-background: #f7f6f3;
    --link-color: #2eaadc;
}

@media (prefers-color-scheme: dark) {
    :root {
        --text-color: #e6e6e6;
        --background-color: #191919;
        --code-background: #2f2f2f;
        --link-color: #529cca;
    }
}

body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 1.7;
    color: var(--text-color);
    background-color: var(--background-color);
    padding: 32px 48px;
    max-width: 900px;
    margin: 0 auto;
}

h1 { font-size: 2.5em; font-weight: 700; margin: 1.5em 0 0.5em; }
h2 { font-size: 1.875em; font-weight: 600; margin: 1.25em 0 0.5em; }
h3 { font-size: 1.5em; font-weight: 600; margin: 1em 0 0.5em; }

p { margin: 0.5em 0; }

a { color: var(--link-color); text-decoration: none; }
a:hover { text-decoration: underline; }

code {
    font-family: "SF Mono", SFMono-Regular, Menlo, Consolas, monospace;
    font-size: 0.9em;
    background-color: var(--code-background);
    padding: 2px 6px;
    border-radius: 4px;
}

pre {
    background-color: var(--code-background);
    padding: 16px;
    border-radius: 4px;
    overflow-x: auto;
}

pre code {
    background: none;
    padding: 0;
}
```

**Verification:**
- [ ] Run: Build project (Cmd+B)
- [ ] Expected: No build errors
- [ ] Check: markdown.css is included in bundle (Build Phases > Copy Bundle Resources)

**Dependencies:** Task 1

**Parallelizable:** YES (with Tasks 10-11)

**Commit Message:** `feat: add Notion-inspired CSS for markdown rendering`

---

### Phase 5: UI Components

#### Task 13: Create DocumentViewModel

**File:** `JustViewer/ViewModels/DocumentViewModel.swift` (Create)

**Changes:**
- Create DocumentViewModel class
- Manage current document state
- Provide computed HTML for preview
- Handle view mode toggle (preview/edit)

**Steps:**
1. Create ViewModels folder in JustViewer group
2. Create DocumentViewModel.swift
3. Implement ObservableObject with published properties
4. Add computed HTML property using MarkdownParser
5. Commit: "feat: add DocumentViewModel for document state management"

**Code sample:**
```swift
import Foundation
import Combine

class DocumentViewModel: ObservableObject {
    @Published var document: MarkdownDocument
    @Published var isEditMode: Bool = false

    private let markdownParser = MarkdownParser()
    private let fileService = FileService()

    var renderedHTML: String {
        markdownParser.parse(document.content)
    }

    init(document: MarkdownDocument = MarkdownDocument()) {
        self.document = document
    }

    func loadFile(from url: URL) {
        do {
            document = try fileService.loadDocument(from: url)
        } catch {
            print("Failed to load file: \(error)")
        }
    }

    func saveFile() {
        do {
            try fileService.saveDocument(document)
        } catch {
            print("Failed to save file: \(error)")
        }
    }

    func toggleEditMode() {
        isEditMode.toggle()
    }
}
```

**Verification:**
- [ ] Run: Build project (Cmd+B)
- [ ] Expected: No build errors
- [ ] Check: DocumentViewModel.swift exists in ViewModels group

**Dependencies:** Tasks 7, 8, 10

**Parallelizable:** NO

**Commit Message:** `feat: add DocumentViewModel for document state management`

---

#### Task 14: Create MarkdownRendererView with WebKit

**File:** `JustViewer/Views/MarkdownRendererView.swift` (Create)

**Changes:**
- Create SwiftUI view wrapping WKWebView
- Implement NSViewRepresentable protocol
- Load HTML with CSS styling

**Steps:**
1. Create Views folder in JustViewer group
2. Create MarkdownRendererView.swift
3. Implement NSViewRepresentable with WKWebView
4. Load HTML string with embedded CSS
5. Commit: "feat: add MarkdownRendererView using WebKit"

**Code sample:**
```swift
import SwiftUI
import WebKit

struct MarkdownRendererView: NSViewRepresentable {
    let html: String

    func makeNSView(context: Context) -> WKWebView {
        let webView = WKWebView()
        webView.setValue(false, forKey: "drawsBackground")
        return webView
    }

    func updateNSView(_ webView: WKWebView, context: Context) {
        let css = loadCSS()
        let fullHTML = """
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <style>\(css)</style>
        </head>
        <body>\(html)</body>
        </html>
        """
        webView.loadHTMLString(fullHTML, baseURL: nil)
    }

    private func loadCSS() -> String {
        guard let url = Bundle.main.url(forResource: "markdown", withExtension: "css"),
              let css = try? String(contentsOf: url) else {
            return ""
        }
        return css
    }
}
```

**Verification:**
- [ ] Run: Build project (Cmd+B)
- [ ] Expected: No build errors
- [ ] Check: MarkdownRendererView.swift exists in Views group

**Dependencies:** Task 12

**Parallelizable:** NO

**Commit Message:** `feat: add MarkdownRendererView using WebKit`

---

#### Task 15: Create MarkdownEditorView

**File:** `JustViewer/Views/MarkdownEditorView.swift` (Create)

**Changes:**
- Create SwiftUI view for raw markdown editing
- Use TextEditor with monospace font
- Bind to document content

**Steps:**
1. Create MarkdownEditorView.swift in Views folder
2. Use SwiftUI TextEditor component
3. Style with monospace font
4. Commit: "feat: add MarkdownEditorView for raw markdown editing"

**Code sample:**
```swift
import SwiftUI

struct MarkdownEditorView: View {
    @Binding var content: String

    var body: some View {
        TextEditor(text: $content)
            .font(.system(.body, design: .monospaced))
            .padding(16)
            .background(Color(NSColor.textBackgroundColor))
    }
}
```

**Verification:**
- [ ] Run: Build project (Cmd+B)
- [ ] Expected: No build errors
- [ ] Check: MarkdownEditorView.swift exists in Views group

**Dependencies:** Task 1

**Parallelizable:** YES (with Task 14)

**Commit Message:** `feat: add MarkdownEditorView for raw markdown editing`

---

#### Task 16: Create ToolbarView with view toggle

**File:** `JustViewer/Views/ToolbarView.swift` (Create)

**Changes:**
- Create toolbar with Open, Save, and view toggle buttons
- Use SF Symbols for icons
- Bind to ViewModel actions

**Steps:**
1. Create ToolbarView.swift in Views folder
2. Add Open, Save, Toggle buttons
3. Use SF Symbols (doc.fill, square.and.arrow.down, eye, pencil)
4. Commit: "feat: add ToolbarView with file and view toggle actions"

**Code sample:**
```swift
import SwiftUI

struct ToolbarView: View {
    @ObservedObject var viewModel: DocumentViewModel
    let onOpen: () -> Void

    var body: some View {
        HStack(spacing: 12) {
            Button(action: onOpen) {
                Image(systemName: "doc.fill")
            }
            .help("Open File")

            Button(action: viewModel.saveFile) {
                Image(systemName: "square.and.arrow.down")
            }
            .help("Save File")
            .disabled(viewModel.document.fileURL == nil)

            Spacer()

            Text(viewModel.document.title)
                .font(.headline)

            Spacer()

            Button(action: viewModel.toggleEditMode) {
                Image(systemName: viewModel.isEditMode ? "eye" : "pencil")
            }
            .help(viewModel.isEditMode ? "Preview Mode" : "Edit Mode")
        }
        .padding(.horizontal, 16)
        .padding(.vertical, 8)
        .background(Color(NSColor.windowBackgroundColor))
    }
}
```

**Verification:**
- [ ] Run: Build project (Cmd+B)
- [ ] Expected: No build errors
- [ ] Check: ToolbarView.swift exists in Views group

**Dependencies:** Task 13

**Parallelizable:** NO

**Commit Message:** `feat: add ToolbarView with file and view toggle actions`

---

#### Task 17: Create ContentView assembling all components

**File:** `JustViewer/Views/ContentView.swift` (Modify)

**Changes:**
- Combine ToolbarView, MarkdownRendererView, MarkdownEditorView
- Toggle between preview and edit modes
- Add Open dialog functionality

**Steps:**
1. Open ContentView.swift (created by Xcode template)
2. Add StateObject for DocumentViewModel
3. Add toolbar, conditional renderer/editor views
4. Implement openFile() with NSOpenPanel
5. Commit: "feat: implement ContentView with all UI components"

**Code sample:**
```swift
import SwiftUI

struct ContentView: View {
    @StateObject private var viewModel = DocumentViewModel()

    var body: some View {
        VStack(spacing: 0) {
            ToolbarView(viewModel: viewModel, onOpen: openFile)

            Divider()

            if viewModel.isEditMode {
                MarkdownEditorView(content: $viewModel.document.content)
            } else {
                MarkdownRendererView(html: viewModel.renderedHTML)
            }
        }
        .frame(minWidth: 600, minHeight: 400)
    }

    private func openFile() {
        let panel = NSOpenPanel()
        panel.allowedContentTypes = [.plainText]
        panel.allowsMultipleSelection = false

        if panel.runModal() == .OK, let url = panel.url {
            viewModel.loadFile(from: url)
        }
    }
}
```

**Verification:**
- [ ] Run: Cmd+R to build and run
- [ ] Expected: App shows toolbar and empty content area
- [ ] Check: Open button triggers file dialog

**Dependencies:** Tasks 13, 14, 15, 16

**Parallelizable:** NO

**Commit Message:** `feat: implement ContentView with all UI components`

---

### Phase 6: File Opening Integration

#### Task 18: Implement file open in AppDelegate

**File:** `JustViewer/AppDelegate.swift` (Modify)

**Changes:**
- Implement application(_:open:) for file URLs
- Pass opened file to ContentView/ViewModel
- Support both file:// URLs and justviewer:// scheme

**Steps:**
1. Open AppDelegate.swift
2. Add reference to DocumentViewModel
3. Implement URL handling in application(_:open:)
4. Parse justviewer://open?path= scheme
5. Commit: "feat: implement file opening via URL schemes in AppDelegate"

**Code sample (additions):**
```swift
// In AppDelegate class
var viewModel: DocumentViewModel!

func applicationDidFinishLaunching(_ notification: Notification) {
    viewModel = DocumentViewModel()
    let contentView = ContentView().environmentObject(viewModel)
    // ... rest of panel setup
}

func application(_ application: NSApplication, open urls: [URL]) {
    for url in urls {
        if url.scheme == "file" {
            viewModel.loadFile(from: url)
        } else if url.scheme == "justviewer" {
            if let components = URLComponents(url: url, resolvingAgainstBaseURL: false),
               let pathItem = components.queryItems?.first(where: { $0.name == "path" }),
               let path = pathItem.value {
                let fileURL = URL(fileURLWithPath: path)
                viewModel.loadFile(from: fileURL)
            }
        }
    }
    panel.makeKeyAndOrderFront(nil)
}
```

**Verification:**
- [ ] Run: Build and run, then open terminal
- [ ] Run: `open "justviewer://open?path=/path/to/test.md"`
- [ ] Expected: JustViewer opens and loads the file
- [ ] Check: File content appears in the viewer

**Dependencies:** Tasks 4, 13, 17

**Parallelizable:** NO

**Commit Message:** `feat: implement file opening via URL schemes in AppDelegate`

---

#### Task 19: Test Finder "Open With" integration

**File:** N/A - Manual testing

**Changes:**
- Build and archive the app
- Test right-click "Open With" on .md files

**Steps:**
1. Build the app (Cmd+B)
2. Right-click JustViewer.app in Products, select "Show in Finder"
3. Copy app to /Applications (optional for testing)
4. Find a .md file in Finder
5. Right-click > Open With > JustViewer
6. Verify file opens in JustViewer

**Verification:**
- [ ] Run: Manual test as described
- [ ] Expected: .md file opens in JustViewer
- [ ] Check: Content renders correctly

**Dependencies:** Task 18

**Parallelizable:** NO

**Commit Message:** N/A (manual test)

---

#### Task 20: Test terminal file:// scheme handling

**File:** N/A - Manual testing

**Changes:**
- Test clicking file:// links in terminal opens JustViewer

**Steps:**
1. Build and run JustViewer
2. In Terminal, use `open file:///path/to/test.md`
3. Verify JustViewer opens the file
4. Test iTerm2 or other terminals if available

**Verification:**
- [ ] Run: `open file:///Users/eatnug/Workspace/JustViewer/.spets/steps/01-plan/instruction.md`
- [ ] Expected: JustViewer opens and displays the file
- [ ] Check: Rendered markdown appears

**Dependencies:** Task 18

**Parallelizable:** YES (with Task 19)

**Commit Message:** N/A (manual test)

---

### Phase 7: Polish & Edge Cases

#### Task 21: Add empty state for no document

**File:** `JustViewer/Views/ContentView.swift` (Modify)

**Changes:**
- Show welcome/empty state when no document is loaded
- Display "Open a file to get started" message

**Steps:**
1. Open ContentView.swift
2. Add conditional for empty document
3. Show centered placeholder text
4. Commit: "feat: add empty state for no document"

**Code sample:**
```swift
// In ContentView body, replace content area:
if viewModel.document.content.isEmpty && viewModel.document.fileURL == nil {
    VStack {
        Image(systemName: "doc.text")
            .font(.system(size: 48))
            .foregroundColor(.secondary)
        Text("Open a markdown file to get started")
            .font(.headline)
            .foregroundColor(.secondary)
        Button("Open File", action: openFile)
            .padding(.top, 8)
    }
    .frame(maxWidth: .infinity, maxHeight: .infinity)
} else if viewModel.isEditMode {
    // ... editor view
} else {
    // ... renderer view
}
```

**Verification:**
- [ ] Run: Cmd+R, launch app without opening file
- [ ] Expected: Empty state with "Open a markdown file" message appears
- [ ] Check: Open button in empty state works

**Dependencies:** Task 17

**Parallelizable:** NO

**Commit Message:** `feat: add empty state for no document`

---

#### Task 22: Handle unsaved changes warning

**File:** `JustViewer/AppDelegate.swift` (Modify)

**Changes:**
- Show alert when closing with unsaved changes
- Prompt to save, discard, or cancel

**Steps:**
1. Open AppDelegate.swift
2. Implement windowShouldClose delegate
3. Check isModified flag
4. Show NSAlert with save options
5. Commit: "feat: add unsaved changes warning on close"

**Code sample:**
```swift
// Make AppDelegate conform to NSWindowDelegate
extension AppDelegate: NSWindowDelegate {
    func windowShouldClose(_ sender: NSWindow) -> Bool {
        guard viewModel.document.isModified else { return true }

        let alert = NSAlert()
        alert.messageText = "Do you want to save changes?"
        alert.informativeText = "Your changes will be lost if you don't save them."
        alert.addButton(withTitle: "Save")
        alert.addButton(withTitle: "Don't Save")
        alert.addButton(withTitle: "Cancel")

        let response = alert.runModal()
        switch response {
        case .alertFirstButtonReturn:
            viewModel.saveFile()
            return true
        case .alertSecondButtonReturn:
            return true
        default:
            return false
        }
    }
}

// In applicationDidFinishLaunching, add:
panel.delegate = self
```

**Verification:**
- [ ] Run: Open file, edit content, try to close window
- [ ] Expected: Alert appears asking to save
- [ ] Check: All three button options work correctly

**Dependencies:** Tasks 13, 18

**Parallelizable:** NO

**Commit Message:** `feat: add unsaved changes warning on close`

---

#### Task 23: Track document modifications

**File:** `JustViewer/ViewModels/DocumentViewModel.swift` (Modify)

**Changes:**
- Set isModified = true when content changes
- Use Combine to observe content changes

**Steps:**
1. Open DocumentViewModel.swift
2. Add cancellables set for Combine
3. Subscribe to document.content changes
4. Set isModified on change (after initial load)
5. Commit: "feat: track document modifications"

**Code sample:**
```swift
// In DocumentViewModel
private var cancellables = Set<AnyCancellable>()
private var initialContent: String = ""

init(document: MarkdownDocument = MarkdownDocument()) {
    self.document = document
    setupContentObserver()
}

private func setupContentObserver() {
    document.$content
        .dropFirst()
        .sink { [weak self] _ in
            self?.document.isModified = true
        }
        .store(in: &cancellables)
}

func loadFile(from url: URL) {
    do {
        document = try fileService.loadDocument(from: url)
        initialContent = document.content
        setupContentObserver()
    } catch {
        print("Failed to load file: \(error)")
    }
}
```

**Verification:**
- [ ] Run: Open file, edit content
- [ ] Expected: isModified becomes true after editing
- [ ] Check: Saving resets isModified to false

**Dependencies:** Task 13

**Parallelizable:** YES (with Task 22)

**Commit Message:** `feat: track document modifications`

---

#### Task 24: Add keyboard shortcuts

**File:** `JustViewer/JustViewerApp.swift` (Modify)

**Changes:**
- Add Cmd+O for Open
- Add Cmd+S for Save
- Add Cmd+E for toggle Edit mode

**Steps:**
1. Open JustViewerApp.swift
2. Add .commands modifier with CommandGroup
3. Add keyboard shortcuts for common actions
4. Commit: "feat: add keyboard shortcuts for common actions"

**Code sample:**
```swift
@main
struct JustViewerApp: App {
    @NSApplicationDelegateAdaptor(AppDelegate.self) var appDelegate

    var body: some Scene {
        Settings {
            EmptyView()
        }
        .commands {
            CommandGroup(replacing: .newItem) {
                Button("Open...") {
                    appDelegate.openFile()
                }
                .keyboardShortcut("o")
            }
            CommandGroup(replacing: .saveItem) {
                Button("Save") {
                    appDelegate.viewModel.saveFile()
                }
                .keyboardShortcut("s")
            }
            CommandGroup(after: .textEditing) {
                Button("Toggle Edit Mode") {
                    appDelegate.viewModel.toggleEditMode()
                }
                .keyboardShortcut("e")
            }
        }
    }
}
```

**Verification:**
- [ ] Run: Cmd+R, test Cmd+O, Cmd+S, Cmd+E
- [ ] Expected: Shortcuts trigger corresponding actions
- [ ] Check: Shortcuts appear in menu bar

**Dependencies:** Tasks 5, 13

**Parallelizable:** NO

**Commit Message:** `feat: add keyboard shortcuts for common actions`

---

### Phase 8: UI Testing

#### Task 25: Create UI test for file opening flow

**File:** `JustViewerUITests/JustViewerUITests.swift` (Modify)

**Changes:**
- Test opening a file via menu
- Verify content appears in window
- Test view toggle

**Steps:**
1. Open JustViewerUITests.swift
2. Add test for open file flow
3. Add test for view toggle
4. Commit: "test: add UI tests for file opening and view toggle"

**Code sample:**
```swift
import XCTest

final class JustViewerUITests: XCTestCase {
    var app: XCUIApplication!

    override func setUpWithError() throws {
        continueAfterFailure = false
        app = XCUIApplication()
        app.launch()
    }

    func testToggleEditMode() throws {
        // Find and click the toggle button
        let toggleButton = app.buttons["Toggle Edit Mode"]
        XCTAssertTrue(toggleButton.waitForExistence(timeout: 5))

        toggleButton.click()

        // Verify editor appears (TextEditor becomes visible)
        let textEditor = app.textViews.firstMatch
        XCTAssertTrue(textEditor.waitForExistence(timeout: 2))
    }

    func testEmptyStateShowsOpenPrompt() throws {
        let openPrompt = app.staticTexts["Open a markdown file to get started"]
        XCTAssertTrue(openPrompt.waitForExistence(timeout: 5))
    }
}
```

**Verification:**
- [ ] Run: Cmd+U to run UI tests
- [ ] Expected: UI tests pass
- [ ] Check: Tests cover basic user flows

**Dependencies:** Tasks 17, 21, 24

**Parallelizable:** NO

**Commit Message:** `test: add UI tests for file opening and view toggle`

---

### Phase 9: Final Integration & Documentation

#### Task 26: Add app icon

**File:** `JustViewer/Resources/Assets.xcassets/AppIcon.appiconset` (Modify)

**Changes:**
- Add app icon images at required sizes
- Use simple markdown-related icon (M in document)

**Steps:**
1. Create app icon image (1024x1024 PNG)
2. Open Assets.xcassets in Xcode
3. Drag icon to AppIcon set
4. Let Xcode generate required sizes
5. Commit: "feat: add app icon"

**Verification:**
- [ ] Run: Build and run, check Dock icon
- [ ] Expected: Custom app icon appears in Dock
- [ ] Check: Icon appears in Finder as well

**Dependencies:** Task 1

**Parallelizable:** YES (can be done anytime)

**Commit Message:** `feat: add app icon`

---

#### Task 27: Final integration test

**File:** N/A - Manual testing

**Changes:**
- Complete end-to-end test of all features

**Steps:**
1. Build release version (Cmd+Shift+B with Release scheme)
2. Test: Launch app, verify empty state
3. Test: Open file via Cmd+O
4. Test: Toggle edit mode via Cmd+E
5. Test: Make changes, save via Cmd+S
6. Test: Close with unsaved changes, verify warning
7. Test: Open via Finder "Open With"
8. Test: Open via terminal `open file://...`
9. Test: Verify overlay above fullscreen app
10. Document any issues

**Verification:**
- [ ] Run: All manual tests above
- [ ] Expected: All features work correctly
- [ ] Check: No crashes or unexpected behavior

**Dependencies:** All previous tasks

**Parallelizable:** NO

**Commit Message:** N/A (manual test)

---

## Files to Modify

| File | Action | Description | Phase | Task # |
|------|--------|-------------|-------|--------|
| `JustViewer.xcodeproj` | Create | Xcode project | 1 | 1 |
| `JustViewer/Info.plist` | Modify | File types, URL schemes | 1 | 2 |
| `JustViewer/JustViewer.entitlements` | Modify | Sandbox file access | 1 | 3 |
| `JustViewer/AppDelegate.swift` | Create | Window management, URL handling | 2 | 4, 18, 22 |
| `JustViewer/JustViewerApp.swift` | Modify | App lifecycle, shortcuts | 2, 7 | 5, 24 |
| `JustViewer/Models/MarkdownDocument.swift` | Create | Document model | 3 | 7 |
| `JustViewer/Services/FileService.swift` | Create | File I/O | 3 | 8 |
| `JustViewer/Services/MarkdownParser.swift` | Create | Markdown to HTML | 4 | 10 |
| `JustViewer/Resources/markdown.css` | Create | Markdown styling | 4 | 12 |
| `JustViewer/ViewModels/DocumentViewModel.swift` | Create | State management | 5 | 13, 23 |
| `JustViewer/Views/MarkdownRendererView.swift` | Create | WebKit preview | 5 | 14 |
| `JustViewer/Views/MarkdownEditorView.swift` | Create | Raw editor | 5 | 15 |
| `JustViewer/Views/ToolbarView.swift` | Create | Toolbar UI | 5 | 16 |
| `JustViewer/Views/ContentView.swift` | Modify | Main container | 5, 7 | 17, 21 |
| `JustViewerTests/FileServiceTests.swift` | Create | File service tests | 3 | 9 |
| `JustViewerTests/MarkdownParserTests.swift` | Create | Parser tests | 4 | 11 |
| `JustViewerUITests/JustViewerUITests.swift` | Modify | UI tests | 8 | 25 |
| `Assets.xcassets/AppIcon.appiconset` | Modify | App icon | 9 | 26 |

## Testing Strategy

### Test Approach

**Strategy:** TDD for services (FileService, MarkdownParser), UI tests for integration

**Rationale:** Services contain pure logic ideal for unit testing. UI flows are better verified via XCUITest for realistic user interaction.

### Unit Tests

**Test file:** `JustViewerTests/*.swift`

**Testing library:** XCTest (native Apple)

**Test cases to implement:**

**Happy Path:**
- [ ] FileService.loadDocument loads valid markdown file
- [ ] FileService.saveDocument writes content correctly
- [ ] MarkdownParser.parse converts headers to HTML
- [ ] MarkdownParser.parse converts bold/italic
- [ ] MarkdownParser.parse converts links
- [ ] MarkdownParser.parse converts code blocks

**Edge Cases:**
- [ ] FileService.loadDocument with empty file returns empty content
- [ ] MarkdownParser.parse with empty string returns empty string
- [ ] MarkdownParser.parse with nested formatting (bold in header)

**Error Cases:**
- [ ] FileService.loadDocument with non-existent file throws error
- [ ] FileService.saveDocument with nil URL throws error

**Verification Commands:**
```bash
# Run all tests
xcodebuild test -project JustViewer.xcodeproj -scheme JustViewer -destination 'platform=macOS'

# Run specific test file
xcodebuild test -project JustViewer.xcodeproj -scheme JustViewer -destination 'platform=macOS' -only-testing:JustViewerTests/FileServiceTests
```

### Integration Tests (UI Tests)

**Integration scenarios:**
- [ ] Empty state shows open prompt: Launch app, verify welcome message
- [ ] Toggle edit mode: Click toggle, verify editor appears
- [ ] Open file flow: Use menu to open file, verify content loads

### Manual Verification

**Manual testing checklist:**
- [ ] Start application: Build and run in Xcode
- [ ] Verify empty state appears on fresh launch
- [ ] Open file via Cmd+O, navigate to .md file
- [ ] Verify markdown renders with Notion-like styling
- [ ] Toggle to edit mode via Cmd+E
- [ ] Make changes to markdown
- [ ] Save via Cmd+S
- [ ] Close window, verify unsaved changes warning
- [ ] Test Finder "Open With" context menu
- [ ] Test terminal `open file://...` command
- [ ] Enter fullscreen in another app, verify JustViewer floats above

**Environment:** macOS 13.0+ (Ventura or later)

## Acceptance Criteria

### Implementation Complete
- [ ] All 27 tasks in all phases completed as specified
- [ ] All files created/modified as planned
- [ ] All verification commands pass
- [ ] Code committed with proper messages

### Tests Passing
- [ ] All unit tests passing: `xcodebuild test`
- [ ] All UI tests passing
- [ ] No flaky tests

### Code Quality
- [ ] No build errors or warnings
- [ ] Code follows Swift conventions
- [ ] No commented-out code
- [ ] No debug print statements

### Functionality
- [ ] App launches with floating panel
- [ ] Panel floats above fullscreen apps
- [ ] Markdown files open from Finder "Open With"
- [ ] Files open from terminal file:// URLs
- [ ] Files open from justviewer:// URLs
- [ ] In-app Open dialog works
- [ ] Markdown renders with Notion-like styling
- [ ] Toggle between preview and edit mode works
- [ ] Save functionality works
- [ ] Unsaved changes warning appears
- [ ] Keyboard shortcuts work (Cmd+O, Cmd+S, Cmd+E)

**Status:** All criteria must be checked before considering feature complete

## Risks & Considerations

### Technical Risks

- **Risk 1**: NSPanel may not work on all macOS versions
  - **Impact:** App may not float above fullscreen on older macOS
  - **Mitigation:** Set minimum deployment target to macOS 13.0
  - **Contingency:** Fall back to regular window if panel fails

- **Risk 2**: WebKit rendering may have performance issues with large files
  - **Impact:** Slow rendering for very large markdown files
  - **Mitigation:** Consider lazy rendering for files > 100KB
  - **Contingency:** Show warning for large files

- **Risk 3**: File sandbox may restrict access
  - **Impact:** Can't open files from certain locations
  - **Mitigation:** Use security-scoped bookmarks if needed
  - **Contingency:** Request broader entitlements

### Edge Cases

- **Empty File**
  - Scenario: User opens empty .md file
  - Handling: Show empty content area, allow editing
  - Test coverage: Task #9 (FileService test)

- **Very Large File**
  - Scenario: File > 10MB markdown
  - Handling: Load and render normally (may be slow)
  - Test coverage: Manual test

- **Non-UTF8 Encoding**
  - Scenario: File with different encoding
  - Handling: UTF-8 decode fails, show error
  - Test coverage: Could add test for this

- **File Deleted While Open**
  - Scenario: External process deletes file
  - Handling: Save will fail, show error alert
  - Test coverage: Manual test

### Dependencies

**External Dependencies:**
- None (using only Apple frameworks)

**Internal Dependencies:**
- WebKit: For markdown rendering
- SwiftUI: For UI components
- AppKit: For NSPanel window management

**Breaking Changes:**
- None - new project

### Performance Considerations

- **Performance Impact:** Low
- **Specific concerns:**
  - WebKit reloads entire HTML on content change
    - Mitigation: Debounce updates in edit mode preview
  - Large CSS may slow initial render
    - Mitigation: Keep CSS minimal, inline critical styles

## Implementation Notes

### Code Patterns to Follow

**Pattern 1: SwiftUI + AppKit Integration**
```swift
// Use NSViewRepresentable for AppKit views in SwiftUI
struct WebViewWrapper: NSViewRepresentable {
    func makeNSView(context: Context) -> WKWebView { ... }
    func updateNSView(_ nsView: WKWebView, context: Context) { ... }
}
```

**Pattern 2: ViewModel with Combine**
```swift
// Use ObservableObject + @Published for reactive state
class DocumentViewModel: ObservableObject {
    @Published var document: MarkdownDocument
    @Published var isEditMode: Bool = false
}
```

**Pattern 3: File URL Handling**
```swift
// Handle both file:// and custom scheme URLs
func application(_ app: NSApplication, open urls: [URL]) {
    for url in urls {
        switch url.scheme {
        case "file": loadFile(from: url)
        case "justviewer": parseCustomURL(url)
        default: break
        }
    }
}
```

### Gotchas & Pitfalls

- **Gotcha 1**: NSPanel needs specific styleMask for floating behavior
  - Why it matters: Wrong styleMask = no floating above fullscreen
  - How to avoid: Use `[.titled, .closable, .resizable, .nonactivatingPanel]`

- **Gotcha 2**: WebKit drawsBackground must be false for transparency
  - Why it matters: White background will show behind content
  - How to avoid: `webView.setValue(false, forKey: "drawsBackground")`

- **Gotcha 3**: File sandbox requires explicit entitlements
  - Why it matters: App can't read files without proper permissions
  - How to avoid: Enable "User Selected File" read/write in entitlements

### Common Mistakes to Avoid

- Do not use WindowGroup for the main window - it won't float properly
- Do not forget to set panel.collectionBehavior for fullscreen auxiliary
- Do not hardcode file paths - always use user-selected URLs
- Do use NSViewRepresentable for WebKit, not UIViewRepresentable

### Debugging Tips

- If panel doesn't float: Check panel.level and collectionBehavior settings
- If file won't open: Check sandbox entitlements and console for errors
- If WebKit shows blank: Check if HTML is valid and CSS is loading

---

## Task Flow Visualization

```
Phase 1 (Setup)
  Task 1 (Create project)
         ↓
  Task 2 (Info.plist) ←→ Task 3 (Entitlements) [parallel]

Phase 2 (Window)
  Task 4 (AppDelegate) → Task 5 (Connect to App) → Task 6 (Test overlay)

Phase 3 (Model)                    Phase 4 (Rendering)
  Task 7 (Document) [parallel] ←→   Task 10 (Parser) [parallel]
         ↓                                  ↓
  Task 8 (FileService)              Task 11 (Parser tests)
         ↓                                  ↓
  Task 9 (FileService tests)        Task 12 (CSS) [parallel]

Phase 5 (UI)
  Task 13 (ViewModel) → Task 14 (Renderer) ←→ Task 15 (Editor) [parallel]
                                ↓
                        Task 16 (Toolbar)
                                ↓
                        Task 17 (ContentView)

Phase 6 (Integration)
  Task 18 (URL handling) → Task 19 (Finder test) ←→ Task 20 (Terminal test) [parallel]

Phase 7 (Polish)
  Task 21 (Empty state) → Task 22 (Unsaved warning) ←→ Task 23 (Track changes) [parallel]
                                        ↓
                                Task 24 (Shortcuts)

Phase 8 (Testing)
  Task 25 (UI tests)

Phase 9 (Final)
  Task 26 (App icon) → Task 27 (Final test)
```

## Estimated Effort

- **Total phases:** 9
- **Total tasks:** 27
- **Complexity:** Medium
- **Parallelization opportunities:** Tasks 2-3, 7-10-12, 14-15, 19-20, 22-23 can run in parallel

---

## Plan Completion Checklist

Before submitting this plan, verify:
- [x] ALL tasks are 2-5 minutes with exact file paths
- [x] Tasks reference patterns and provide code samples
- [x] EVERY task has concrete verification (command + expected output)
- [x] Zero assumptions about business logic (greenfield project)
- [x] Test strategy decided: TDD for services, UI tests for integration
- [x] Dependencies and parallelization marked
- [x] Phase organization logical and complete
- [x] One complete plan covering entire feature
