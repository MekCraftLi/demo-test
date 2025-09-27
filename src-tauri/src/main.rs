// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
extern crate core;

/******* import ***********************************************************************************/
use tokio::sync::Mutex;

mod command;



fn main() {
    run();
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(None::<command::serial::TaskState>))
        .invoke_handler(tauri::generate_handler![
            /* 设置对前端开放的接口函数 */
            command::window::create_window,
            command::window::close_window,
            command::window::maximize_window,
            command::window::minimize_window,
            command::window::reset_window,
            command::serial::start_serial_simulation,
            command::serial::start_receive_comm,
            command::serial::stop_receive_comm,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}