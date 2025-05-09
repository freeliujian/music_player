// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod customs_handle;
use tauri::{LogicalSize, Manager};
use tauri_plugin_log::{Target, TargetKind};
use customs_handle::customs_handle::{control_window, greet};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.set_min_size(Some(LogicalSize::new(1020, 670)))?;
            window.set_size(LogicalSize::new(1020, 670))?;
            window.set_decorations(false)?;
            Ok(())
        })
        .invoke_handler(
            tauri::generate_handler![
                control_window,
                greet
            ])
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),  // 输出到终端
                    Target::new(TargetKind::Webview), // 输出到 Webview 控制台
                    Target::new(TargetKind::LogDir {
                        file_name: Some("logs".to_string()),
                    }), // 输出到日志文件)
                ])
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
