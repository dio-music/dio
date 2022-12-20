#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;

use rfd::FileDialog;

struct Dio(Mutex<DioState>);

struct DioState {
    spotify_data_file_path: String,
}

impl Default for DioState {
    fn default() -> Self {
        DioState {
            spotify_data_file_path: "".to_owned(),
        }
    }
}

#[tauri::command]
fn select_spotify_data_folder(unlocked_state: tauri::State<Dio>) -> Option<String> {
    // Attempt to grab the mutex on the state struct managed by Tauri
    if let Ok(mut state) = unlocked_state.0.lock() {
        // Create a file dialog for the user to select a folder path
        state.spotify_data_file_path = FileDialog::new().pick_folder()?.to_str()?.to_owned();
        // Return the folder path as an owned string
        Some(state.spotify_data_file_path.to_owned())
    } else {
        // If the mutex for the app state isn't able to be locked, return None
        None
    }
}

fn main() {
    tauri::Builder::default()
        .manage(Dio(Mutex::new(DioState::default())))
        .invoke_handler(tauri::generate_handler![select_spotify_data_folder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
