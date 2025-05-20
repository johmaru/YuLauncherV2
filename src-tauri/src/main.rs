
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use yulauncherv2_lib::libs::settings_control;

fn main() {
    if let Err(e) = settings_control::check_settings_file() {
        eprintln!("settings error: {e}");
    }
    yulauncherv2_lib::run()
}
