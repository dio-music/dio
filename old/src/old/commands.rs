use crate::group::{Group, GroupBy};
use crate::sort::SortSpotifyDataBy;
use crate::Dio;
use crate::{dates, filter, group, plays, sort};
use rfd::FileDialog;

#[tauri::command]
pub async fn load_spotify_data(unlocked_state: tauri::State<'_, Dio>) -> Result<(), String> {
    let Some(folder_path) = FileDialog::new().pick_folder() else {
        return Err("Error while choosing a folder containing Spotify data.".to_owned());
    };

    let Ok(spotify_plays_data) = plays::extract_plays_from_path(&folder_path).await else {
        return Err("Error while attempting to load Spotify data.".to_owned());
    };

    let Ok(date_range_boundaries) = dates::get_date_bounds_from_play_items(&spotify_plays_data) else {
        return Err("Unable to find the earliest and latest dates from the Spotify data.".to_owned());
    };

    let Ok(mut state) = unlocked_state.0.lock() else {
        return Err("Unable to acquire lock on global state managed by Tauri.".to_owned());
    };

    state.spotify_data_folder_path = Some(folder_path);
    state.spotify_plays_data = spotify_plays_data;
    state.filter.date_range_boundaries = date_range_boundaries;

    // JAKE: Testing
    state.group_by = GroupBy::Song;

    let mut grouped_data =
        group::get_grouped_data(&state.group_by, state.spotify_plays_data.clone());
    sort::sort_grouped_data(&mut grouped_data, sort::SortSpotifyDataBy::PlayCount, true);

    println!("");
    for (i, group) in grouped_data.iter().enumerate() {
        if i == 25 {
            break;
        }
        println!("{}. {}", i + 1, group);
    }

    Ok(())
}

#[tauri::command]
pub fn get_processed_data(unlocked_state: tauri::State<Dio>) -> Result<Vec<Group>, String> {
    let Ok(state) = unlocked_state.0.lock() else {
        return Err("Unable to acquire lock on global state managed by Tauri.".to_owned());
    };

    Ok(state.processed_data.clone())
}

#[tauri::command]
pub fn set_group_by(
    unlocked_state: tauri::State<Dio>,
    new_filter_group: String,
) -> Result<(), String> {
    let Ok(mut state) = unlocked_state.0.lock() else {
        return Err("Unable to acquire lock on global state managed by Tauri.".to_owned());
    };

    state.group_by = match new_filter_group.as_str() {
        "songs" => GroupBy::Song,
        "artists" => GroupBy::Artist,
        "albums" => GroupBy::Album,
        "podcasts" => GroupBy::Podcast,
        "podcast_episodes" => GroupBy::PodcastEpisode,
        _ => return Err("Invalid filter group string passed.".to_owned()),
    };

    Ok(())
}

// TODO: filter commands, maybe put them in filter.rs, or a new commands.rs
// fn set_filter

#[tauri::command]
pub fn apply_filters_and_group(unlocked_state: tauri::State<Dio>) -> Result<(), String> {
    let Ok(mut state) = unlocked_state.0.lock() else {
        return Err("Unable to acquire lock on global state managed by Tauri.".to_owned());
    };

    let play_data_within_filter_dates =
        filter::get_play_items_between_dates(&state.spotify_plays_data, &state.filter);

    let grouped_data = group::get_grouped_data(&state.group_by, play_data_within_filter_dates);

    // TODO: Add more filters to the data
    state.processed_data = grouped_data;

    Ok(())
}

#[tauri::command]
pub fn set_sort(
    unlocked_state: tauri::State<Dio>,
    new_sort: String,
    descending: bool,
) -> Result<(), String> {
    let Ok(mut state) = unlocked_state.0.lock() else {
        return Err("Unable to acquire lock on global state managed by Tauri.".to_owned());
    };

    state.sort_by = match new_sort.as_str() {
        "auto_play_pct" => SortSpotifyDataBy::AutoPlayPct,
        "click_pct" => SortSpotifyDataBy::ClickPct,
        "play_count" => SortSpotifyDataBy::PlayCount,
        "shuffle_pct" => SortSpotifyDataBy::ShufflePct,
        "skip_pct" => SortSpotifyDataBy::SkipPct,
        "total_listening_time" => SortSpotifyDataBy::TotalListenTime,
        _ => return Err("Invalid sort_by string passed into set_sort()".to_owned()),
    };

    state.sort_order_descending = descending;

    Ok(())
}

#[tauri::command]
pub fn apply_sort(unlocked_state: tauri::State<Dio>) -> Result<(), String> {
    let Ok(mut state) = unlocked_state.0.lock() else {
        return Err("Unable to acquire lock on global state managed by Tauri.".to_owned());
    };

    let sort_by = state.sort_by.clone();
    let sort_order_descending = state.sort_order_descending.clone();

    sort::sort_grouped_data(&mut state.processed_data, sort_by, sort_order_descending);

    Ok(())
}

#[tauri::command]
pub fn reset_filter(unlocked_state: tauri::State<Dio>) -> Result<(), String> {
    let Ok(mut state) = unlocked_state.0.lock() else {
        return Err("Unable to acquire lock on global state managed by Tauri.".to_owned());
    };

    state.filter.date_range = None;
    state.group_by = GroupBy::Song;
    state.sort_by = SortSpotifyDataBy::TotalListenTime;
    Ok(())
}
