use servo::{Servo, ServoCommand};
use shared::Error;

pub mod shared;
pub mod servo;

#[tauri::command]
async fn servo_up() -> Result<String,Error> {
    Servo::command(ServoCommand::Up).await
}

#[tauri::command]
async fn servo_down() -> Result<String,Error> {
    Servo::command(ServoCommand::Down).await
}

#[tauri::command]
async fn servo_reset() -> Result<String,Error> {
    Servo::command(ServoCommand::Reset).await
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
