#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod dates;
mod filter;
mod group;
mod plays;
mod sort;
mod util;

use filter::Filter;
use group::{Group, GroupBy};
use sort::SortSpotifyDataBy;
use std::{path::PathBuf, sync::Mutex};

pub struct Dio(Mutex<DioState>);

pub struct DioState {
    spotify_data_folder_path: Option<PathBuf>,
    spotify_plays_data: Vec<plays::PlayItem>,
    filter: filter::Filter,
    processed_data: Vec<Group>,
    group_by: group::GroupBy,
    sort_by: sort::SortSpotifyDataBy,
    sort_order_descending: bool,
}

impl Default for DioState {
    fn default() -> Self {
        DioState {
            spotify_data_folder_path: None,
            spotify_plays_data: Vec::new(),
            filter: Filter::default(),
            processed_data: Vec::new(),
            group_by: GroupBy::Song,
            sort_by: SortSpotifyDataBy::TotalListenTime,
            sort_order_descending: true,
        }
    }
}

fn main() {
    tauri::Builder::default()
        .manage(Dio(Mutex::new(DioState::default())))
        .invoke_handler(tauri::generate_handler![
            commands::load_spotify_data,
            commands::get_processed_data,
            commands::set_group_by,
            commands::apply_filters_and_group,
            commands::reset_filter,
            commands::set_sort,
            commands::apply_sort
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
