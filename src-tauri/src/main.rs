// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let mut app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .build(tauri::generate_context!())
        .expect("should be able to build the Tauri app");

    let start_time = std::time::Instant::now();

    loop {
        let iteration = app.run_iteration();
        println!("Ran Tauri iteration at {:?}", start_time.elapsed());
        if iteration.window_count == 0 {
            break;
        }
    }
}
