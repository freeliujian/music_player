// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri_plugin_log::{Target, TargetKind};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().targets([
            Target::new(TargetKind::Stdout),  // 输出到终端
            Target::new(TargetKind::Webview), // 输出到 Webview 控制台
            Target::new(TargetKind::LogDir {
                file_name: Some("logs".to_string()),
            }),  // 输出到日志文件)
        ])
            .build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
