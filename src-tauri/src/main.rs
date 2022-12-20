#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod data_loading;

use rfd::FileDialog;
use std::sync::Mutex;

struct Dio(Mutex<DioState>);

struct DioState {
    spotify_plays_data: Vec<data_loading::PlayedItem>,
}

impl Default for DioState {
    fn default() -> Self {
        DioState {
            spotify_plays_data: Vec::new(),
        }
    }
}

#[tauri::command]
fn load_spotify_data(unlocked_state: tauri::State<Dio>) -> Result<(), String> {
    // Attempt to grab the mutex on the state struct managed by Tauri
    match unlocked_state.0.lock() {
        Err(_) => Err("Unable to acquire lock on global state managed by Tauri.".to_owned()),
        // Prompt the user with a folder picking dialog
        Ok(mut state) => match FileDialog::new().pick_folder() {
            None => Err("Error while choosing a folder containing Spotify data.".to_owned()),
            // Extract the folder path from the Option that was returned
            Some(folder_path) => match data_loading::extract_plays_from_path(&folder_path) {
                Err(_) => Err("Error while attempting to load Spotify data.".to_owned()),
                // Extract the vec of PlayedItems from the Option that was returned
                Ok(spotify_plays_data) => {
                    state.spotify_plays_data = spotify_plays_data;
                    Ok(())
                }
            },
        },
    }
}

fn main() {
    tauri::Builder::default()
        .manage(Dio(Mutex::new(DioState::default())))
        .invoke_handler(tauri::generate_handler![load_spotify_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
