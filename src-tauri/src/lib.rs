use servo::Servo;
use shared::{Globals, Result};
use tauri::{async_runtime::Mutex, State};

pub mod shared;
pub mod servo;

#[tauri::command]
async fn servo_up(value: i32, state: State<'_, Mutex<Globals>>) -> Result<String> {
    Servo::up(value, &*state.lock().await).await
}

#[tauri::command]
async fn servo_down(value: i32, state: State<'_, Mutex<Globals>>) -> Result<String> {
    Servo::down(value, &*state.lock().await).await
}

#[tauri::command]
async fn servo_reset(state: State<'_, Mutex<Globals>>) -> Result<()> {
    Servo::reset(&*state.lock().await).await
}

#[tauri::command]
async fn servo_geterin(state: State<'_, Mutex<Globals>>) -> Result<()> {
    Servo::geterin(&*state.lock().await).await
}

#[tauri::command]
async fn set_url(url: String, state: State<'_, Mutex<Globals>>) -> Result<()> {
    let mut glob = state.lock().await;
    glob.url = url;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(Globals::default()))
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            servo_up,
            servo_down,
            servo_reset,
            servo_geterin,
            set_url,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
