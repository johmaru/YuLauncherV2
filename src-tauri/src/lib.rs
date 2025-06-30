use tauri::{Emitter, Manager, Size};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod libs;
#[tauri::command]
fn greet(name: &str) -> String {
    format!("こんにちは, {}!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let settings =crate::libs::settings_control::load_settings()?;
            if let Some(window) = app.get_webview_window("main") {
                window.set_size(Size::Logical(tauri::LogicalSize {
                    width: settings.resolution.width as f64,
                    height: settings.resolution.height as f64,
                }))?;

                window.set_theme(Some(if settings.theme == "dark" {
                    tauri::Theme::Dark
                } else {
                    tauri::Theme::Light
                }))?;

                window.emit("theme-apply", settings.theme.clone())?;
            }

            app.set_theme(Some(if settings.theme == "dark" {
                tauri::Theme::Dark
            } else {
                tauri::Theme::Light
            }));

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
