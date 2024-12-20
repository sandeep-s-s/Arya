// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
mod cmd;
mod db;
mod models;
mod schema;

mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            cmd::create_collection,
            cmd::get_collections,
            cmd::create_request,
            cmd::get_request,
            cmd::save_request,
            cmd::send_request,
            cmd::toggle_collection
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
