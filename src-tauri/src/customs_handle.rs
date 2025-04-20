pub mod customs_handle {
    #[tauri::command]
    pub fn close_window(window:tauri::Window) {
        window.close().expect("failed to close window");
    }

    #[tauri::command]
    pub fn greet(name: &str) -> String {
        format!("Hello, {}! You've been greeted from Rust!", name)
    }
}

