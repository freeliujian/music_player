pub mod customs_handle {
    use common_const::window_control_common_const::ControlWindowType;

    #[tauri::command]
    pub fn control_window(window:tauri::Window, args:ControlWindowType) -> Result<(), String> {
        match args {
            ControlWindowType::Hide => window.minimize().map_err(|e| e.to_string()),
            ControlWindowType::Maximize => {
                window.maximize().map_err(|e| e.to_string())
            },
            ControlWindowType::Normal => {
                window.set_size(tauri::Size::Logical(tauri::LogicalSize {
                    width: 800.0,
                    height: 600.0,
                })).map_err(|e| e.to_string())
            }
            ControlWindowType::Close => {
                window.close().map_err(|e| e.to_string())
            }
        }
    }

    #[tauri::command]
    pub fn greet(name: &str) -> String {
        format!("Hello, {}! You've been greeted from Rust!", name)
    }
}

