use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager, RunEvent, WebviewUrl, WebviewWindowBuilder};

// Store opened file paths for later retrieval
struct OpenedFiles(Mutex<Vec<String>>);

#[tauri::command]
fn get_opened_files(state: tauri::State<OpenedFiles>) -> Vec<String> {
    let files = state.0.lock().unwrap();
    files.clone()
}

#[tauri::command]
fn clear_opened_files(state: tauri::State<OpenedFiles>) {
    let mut files = state.0.lock().unwrap();
    files.clear();
}

/// Set window collection behavior to move to active space
#[cfg(target_os = "macos")]
fn set_move_to_active_space(window: &tauri::WebviewWindow) {
    use cocoa::appkit::{NSWindow, NSWindowCollectionBehavior};
    use cocoa::base::id;

    if let Ok(ns_window) = window.ns_window() {
        unsafe {
            let ns_window = ns_window as id;
            ns_window.setCollectionBehavior_(
                NSWindowCollectionBehavior::NSWindowCollectionBehaviorMoveToActiveSpace,
            );
        }
    }
}

/// Creates or shows window
fn open_window(app: &AppHandle) {
    // Check if window exists (warm start - app was already running)
    if let Some(window) = app.get_webview_window("main") {
        // Set behavior to move to active space before showing
        #[cfg(target_os = "macos")]
        set_move_to_active_space(&window);

        // Window exists - show it (will move to current space)
        let _ = window.show();
        let _ = window.set_focus();
        return;
    }

    // Cold start - window doesn't exist yet
    // Wait for macOS space switch to complete, then create window on Desktop
    std::thread::sleep(std::time::Duration::from_millis(300));

    // Create new window (will appear on Desktop after space switch)
    if let Ok(window) = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
        .title("JustViewer")
        .inner_size(800.0, 600.0)
        .resizable(true)
        .visible(true)
        .build()
    {
        // Setup close handler - hide instead of close
        let w = window.clone();
        window.on_window_event(move |event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = w.hide();
            }
        });

        let _ = window.set_focus();
    }
}


fn process_urls(urls: Vec<url::Url>, state: &tauri::State<OpenedFiles>, app_handle: &AppHandle) {
    // Collect paths first
    let mut paths: Vec<String> = Vec::new();

    for url in urls {
        let path = if url.scheme() == "file" {
            url.to_file_path().ok().map(|p| p.to_string_lossy().to_string())
        } else if url.scheme() == "justviewer" {
            url.query_pairs()
                .find(|(key, _)| key == "path")
                .map(|(_, value)| value.to_string())
        } else {
            None
        };

        if let Some(path) = path {
            paths.push(path);
        }
    }

    if paths.is_empty() {
        return;
    }

    // Store in state
    {
        let mut files = state.0.lock().unwrap();
        for path in &paths {
            files.push(path.clone());
        }
    }

    // Open window (creates new on cold start, shows existing on warm start)
    open_window(app_handle);

    // Emit file-opened events after window is ready
    let app_clone = app_handle.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(150));
        for path in paths {
            let _ = app_clone.emit("file-opened", path);
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(OpenedFiles(Mutex::new(Vec::new())))
        .invoke_handler(tauri::generate_handler![get_opened_files, clear_opened_files])
        .setup(|app| {
            // LSUIElement in Info.plist makes this a background app (no dock icon, no space switching)
            // Windows are created on demand

            // Check for URLs passed at startup (cold start)
            #[cfg(any(target_os = "macos", target_os = "ios"))]
            {
                use tauri_plugin_deep_link::DeepLinkExt;
                if let Ok(urls) = app.deep_link().get_current() {
                    if let Some(urls) = urls {
                        let state = app.state::<OpenedFiles>();
                        let mut files = state.0.lock().unwrap();
                        for url in urls {
                            let path = if url.scheme() == "file" {
                                url.to_file_path().ok().map(|p| p.to_string_lossy().to_string())
                            } else {
                                None
                            };
                            if let Some(path) = path {
                                files.push(path);
                            }
                        }
                    }
                }
            }

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|app_handle, event| {
        match event {
            RunEvent::Opened { urls } => {
                let state = app_handle.state::<OpenedFiles>();
                process_urls(urls, &state, app_handle);
            }
            // Don't create window on Ready - only when file is opened
            RunEvent::Ready => {}
            // Keep app running even when all windows are closed
            RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        }
    });
}
