#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod dates;
mod filter;
mod group;
mod plays;
mod sort;
mod util;

use filter::Filter;
use rfd::FileDialog;
use std::{path::PathBuf, sync::Mutex};

struct Dio(Mutex<DioState>);

struct DioState {
    spotify_data_folder_path: Option<PathBuf>,
    spotify_plays_data: Vec<plays::PlayItem>,
    filter: filter::Filter,
    // filtered_data: Mutex<Vec<group1::SpotifyData>>,
}

impl Default for DioState {
    fn default() -> Self {
        DioState {
            spotify_data_folder_path: None,
            spotify_plays_data: Vec::new(),
            filter: Filter::default(),
            // filtered_data: Mutex::new(Vec::new()),
        }
    }
}

#[tauri::command]
async fn load_spotify_data(unlocked_state: tauri::State<'_, Dio>) -> Result<(), String> {
    let Some(folder_path) = FileDialog::new().pick_folder() else {
        return Err("Error while choosing a folder containing Spotify data.".to_owned());
    };

    let Ok(spotify_plays_data) = plays::extract_plays_from_path(&folder_path).await else {
        return Err("Error while attempting to load Spotify data.".to_owned());
    };

    let Ok(date_range_boundaries) = dates::get_min_and_max_dates_from_play_items(&spotify_plays_data) else {
        return Err("Unable to find the earliest and latest dates from the Spotify data.".to_owned());
    };

    let Ok(mut state) = unlocked_state.0.lock() else {
        return Err("Unable to acquire lock on global state managed by Tauri.".to_owned());
    };

    state.spotify_data_folder_path = Some(folder_path);
    state.spotify_plays_data = spotify_plays_data;
    state.filter.date_range_boundaries = date_range_boundaries;

    let mut temp = group::get_grouped_data(&state.filter.group_by, &state.spotify_plays_data);
    sort::sort_grouped_data(&mut temp, sort::SortSpotifyDataBy::TotalListenTime, true);

    println!("");

    for (i, group) in temp.iter().enumerate() {
        if i == 100 {
            break;
        }

        println!("{}. {}", i + 1, group);
    }

    Ok(())
}

#[tauri::command]
fn apply_filter(unlocked_state: tauri::State<Dio>) -> Result<(), String> {
    let Ok(state) = unlocked_state.0.lock() else {
        return Err("Unable to acquire lock on global state managed by Tauri.".to_owned());
    };

    // let Ok(mut filtered_data) = state.filtered_data.lock() else {
    //     return Err("Unable to acquire lock on filtered data vec.".to_owned());
    // };

    // *filtered_data = match state.filter.group_by {
    //     GroupBy::Song => {}
    //     _ => todo!(),
    // };

    // let grouped_data = match state.filter.group_by {
    //     GroupBy::Song => group::get_aggregated_data::<group::SongData>(&state.spotify_plays_data),
    //     _ => todo!(),
    // };

    // *filtered_data = grouped_data.into_iter().map(|a| Box::<dyn groups::PlayGroup + Send>::from(a)).collect();

    Ok(())
}

// #[tauri::command]
// fn reset_filter(unlocked_state: tauri::State<Dio>) -> Result<(), String> {
//     let Ok(mut state) = unlocked_state.0.lock() else {
//         return Err("Unable to acquire lock on global state managed by Tauri.".to_owned());
//     };

//     state.filter.date_range = None;
//     state.filter.group_by = GroupBy::Song;

//     Ok(())
// }

// #[tauri::command]
// fn set_filter_group(
//     unlocked_state: tauri::State<Dio>,
//     new_filter_group: String,
// ) -> Result<(), String> {
//     let Ok(mut state) = unlocked_state.0.lock() else {
//         return Err("Unable to acquire lock on global state managed by Tauri.".to_owned());
//     };

//     match new_filter_group.as_str() {
//         "songs" => state.filter.group_by = GroupBy::Song,
//         "artists" => state.filter.group_by = GroupBy::Artist,
//         "albums" => state.filter.group_by = GroupBy::Album,
//         "podcasts" => state.filter.group_by = GroupBy::Podcast,
//         "podcast-episodes" => state.filter.group_by = GroupBy::PodcastEpisode,
//         _ => return Err("Invalid filter group string passed.".to_owned()),
//     };

//     todo!()
// }

fn main() {
    tauri::Builder::default()
        .manage(Dio(Mutex::new(DioState::default())))
        .invoke_handler(tauri::generate_handler![
            load_spotify_data,
            // apply_filter,
            // reset_filter,
            // set_filter_group
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
