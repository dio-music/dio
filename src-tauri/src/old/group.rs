use crate::plays::PlayItem;
use crate::util;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::{self, Display};

/// PlayGroup is a trait meant to standardize the functions needed for a struct to represent aggregated data
/// for a single piece of music data (Artist, Album, Song). An instance of a struct that implements
/// PlayGroup will represent the aggregated of one artist, album, or song.
pub trait PlayGroup {
    /// Creates a new instance from an instance of PlayedItem
    fn from_track_info(played_item: &PlayItem) -> Self
    where
        Self: Sized;

    /// Extracts a field from played_item to be used as a dictionary key. This key changes based on the
    /// struct that implements SpotifyData. For example, a SongData struct would return the song title,
    /// while an ArtistData struct would return the artist's name.
    /// TODO: Determine if the UID should be included when generating the key
    fn get_key(played_item: &PlayItem) -> String
    where
        Self: Sized;

    /// Adds to the total number of ms of play time the instance has
    fn add_time_to_ms_played(&mut self, new_ms_played: &u64);

    /// Increment the instance's play count by one
    fn increment_play_count(&mut self);

    /// Increment the instance's skip count by one
    fn increment_skip_count(&mut self);

    /// Increment the instance's click count by one
    fn increment_click_count(&mut self);

    /// Increment the instance's shuffle count by one
    fn increment_shuffle_count(&mut self);

    /// Returns the total play time for the instance (in ms)
    fn get_ms_played(&self) -> u64;

    /// Returns the total number of plays for the instance
    fn get_play_count(&self) -> u32;

    /// Returns the overall skip percentage
    fn get_skip_pct(&self) -> f64;

    /// Returns the overall click percentage
    fn get_click_pct(&self) -> f64;

    /// Returns the overall shuffle percentage
    fn get_shuffle_pct(&self) -> f64;

    /// Returns true if the played item should be included in the overall aggregation. This is a bit
    /// confusing, but the general point is to only count plays of songs if the Struct is SongData,
    /// ArtistData, or AlbumData. Similarly, only plays of podcasts are valid for aggregation if
    /// the Struct is EpisodeData or PodcastData.
    fn played_item_is_valid_for_aggregation(played_item: &PlayItem) -> bool
    where
        Self: Sized;
}

pub enum GroupBy {
    Song,
    Album,
    Artist,
    Podcast,
    PodcastEpisode,
}

pub enum SpotifyData {
    Song(SongData),
    Album(AlbumData),
    Artist(ArtistData),
    Podcast(PodcastData),
    PodcastEpisode(EpisodeData),
}

///////////////
// SONG DATA //
///////////////

/// Represents the aggregated data (across all PlayedItem instances in a collection) about a single song
#[derive(Clone, Debug, Serialize)]
pub struct SongData {
    pub album_name: String,
    pub artist_name: String,
    pub track_name: String,
    pub ms_played: u64,
    pub play_count: u32,
    pub skip_count: u32,
    pub click_count: u32,
    pub shuffle_count: u32,
}

impl PlayGroup for SongData {
    fn from_track_info(played_item: &PlayItem) -> Self {
        if let PlayItem {
            master_metadata_album_album_name: Some(album_name),
            master_metadata_album_artist_name: Some(artist_name),
            master_metadata_track_name: Some(track_name),
            .. // Ignore all other fields of played_item
        } = played_item {
            SongData {
                album_name: album_name.to_owned(),
                artist_name: artist_name.to_owned(),
                track_name: track_name.to_owned(),
                ms_played: 0,
                play_count: 0,
                skip_count: 0,
                click_count: 0,
                shuffle_count: 0,
            }
        } else {
            SongData {
                album_name: "".to_owned(),
                artist_name: "".to_owned(),
                track_name: "".to_owned(),
                ms_played: 0,
                play_count: 0,
                skip_count: 0,
                click_count: 0,
                shuffle_count: 0,
            }
        }
    }

    // Returns the track URi as the key
    fn get_key(played_item: &PlayItem) -> String {
        match &played_item.spotify_track_uri {
            Some(track_uri) => track_uri.to_owned(),
            None => "".to_owned(),
        }
    }

    fn add_time_to_ms_played(&mut self, new_ms_played: &u64) {
        self.ms_played += new_ms_played;
    }

    fn increment_play_count(&mut self) {
        self.play_count += 1;
    }

    fn increment_skip_count(&mut self) {
        self.skip_count += 1;
    }

    fn increment_click_count(&mut self) {
        self.click_count += 1;
    }

    fn increment_shuffle_count(&mut self) {
        self.shuffle_count += 1;
    }

    fn get_ms_played(&self) -> u64 {
        self.ms_played
    }

    fn get_play_count(&self) -> u32 {
        self.play_count
    }

    fn get_skip_pct(&self) -> f64 {
        if self.play_count == 0 {
            0.
        } else {
            self.skip_count as f64 / self.play_count as f64
        }
    }

    fn get_click_pct(&self) -> f64 {
        if self.play_count == 0 {
            0.
        } else {
            self.click_count as f64 / self.play_count as f64
        }
    }

    fn get_shuffle_pct(&self) -> f64 {
        if self.play_count == 0 {
            0.
        } else {
            self.shuffle_count as f64 / self.play_count as f64
        }
    }

    fn played_item_is_valid_for_aggregation(played_item: &PlayItem) -> bool {
        matches!(
            played_item,
            PlayItem {
                master_metadata_album_album_name: Some(_),
                master_metadata_album_artist_name: Some(_),
                master_metadata_track_name: Some(_),
                .. // Ignore all other fields of played_item
            }
        )
    }
}

impl fmt::Display for SongData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} by {}\nAlbum: {}\nPlay Count: {}\nPlayed For: {}\n",
            self.track_name,
            self.artist_name,
            self.album_name,
            self.play_count,
            util::get_total_listen_time_from_ms(self.ms_played)
        )
    }
}

////////////////
// ALBUM DATA //
////////////////

#[derive(Clone, Debug, Serialize)]
pub struct AlbumData {
    pub album_name: String,
    pub artist_name: String,
    pub ms_played: u64,
    pub play_count: u32,
    pub skip_count: u32,
    pub click_count: u32,
    pub shuffle_count: u32,
}

impl PlayGroup for AlbumData {
    fn from_track_info(played_item: &PlayItem) -> Self {
        if let PlayItem {
            master_metadata_album_album_name: Some(album_name),
            master_metadata_album_artist_name: Some(artist_name),
            .. // Ignore all other fields of played_item
        } = played_item {
            AlbumData {
                album_name: album_name.to_owned(),
                artist_name: artist_name.to_owned(),
                ms_played: 0,
                play_count: 0,
                skip_count: 0,
                click_count: 0,
                shuffle_count: 0,
            }
        } else {
            AlbumData {
                album_name: "".to_owned(),
                artist_name: "".to_owned(),
                ms_played: 0,
                play_count: 0,
                skip_count: 0,
                click_count: 0,
                shuffle_count: 0,
            }
        }
    }

    // Returns a formatted string with the album and artist as the key
    fn get_key(played_item: &PlayItem) -> String {
        if let PlayItem {
            master_metadata_album_album_name: Some(album_name),
            master_metadata_album_artist_name: Some(artist_name),
            ..
        } = played_item
        {
            format!("{} by {}", album_name, artist_name)
        } else {
            "".to_owned()
        }
    }

    fn add_time_to_ms_played(&mut self, new_ms_played: &u64) {
        self.ms_played += new_ms_played;
    }

    fn increment_play_count(&mut self) {
        self.play_count += 1;
    }

    fn increment_skip_count(&mut self) {
        self.skip_count += 1;
    }

    fn increment_click_count(&mut self) {
        self.click_count += 1;
    }

    fn increment_shuffle_count(&mut self) {
        self.shuffle_count += 1;
    }

    fn get_ms_played(&self) -> u64 {
        self.ms_played
    }

    fn get_play_count(&self) -> u32 {
        self.play_count
    }

    fn get_skip_pct(&self) -> f64 {
        if self.play_count == 0 {
            0.
        } else {
            self.skip_count as f64 / self.play_count as f64
        }
    }

    fn get_click_pct(&self) -> f64 {
        if self.play_count == 0 {
            0.
        } else {
            self.click_count as f64 / self.play_count as f64
        }
    }

    fn get_shuffle_pct(&self) -> f64 {
        if self.play_count == 0 {
            0.
        } else {
            self.shuffle_count as f64 / self.play_count as f64
        }
    }

    fn played_item_is_valid_for_aggregation(played_item: &PlayItem) -> bool {
        matches!(
            played_item,
            PlayItem {
                master_metadata_album_album_name: Some(_),
                master_metadata_album_artist_name: Some(_),
                master_metadata_track_name: Some(_),
                .. // Ignore all other fields of played_item
            }
        )
    }
}

impl fmt::Display for AlbumData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} by {}\nPlay Count: {}\nPlayed For: {}\n",
            self.album_name,
            self.artist_name,
            self.play_count,
            util::get_total_listen_time_from_ms(self.ms_played)
        )
    }
}

/////////////////
// ARTIST DATA //
/////////////////

#[derive(Clone, Debug, Serialize)]
pub struct ArtistData {
    pub artist_name: String,
    pub ms_played: u64,
    pub play_count: u32,
    pub skip_count: u32,
    pub click_count: u32,
    pub shuffle_count: u32,
}

impl PlayGroup for ArtistData {
    fn from_track_info(played_item: &PlayItem) -> Self {
        if let PlayItem {
            master_metadata_album_artist_name: Some(artist_name),
            .. // Ignore all other fields of played_item
        } = played_item {
            ArtistData {
                artist_name: artist_name.to_owned(),
                ms_played: 0,
                play_count: 0,
                skip_count: 0,
                click_count: 0,
                shuffle_count: 0,
            }
        } else {
            ArtistData {
                artist_name: "".to_owned(),
                ms_played: 0,
                play_count: 0,
                skip_count: 0,
                click_count: 0,
                shuffle_count: 0,
            }
        }
    }

    // Returns the artist name as the key
    fn get_key(played_item: &PlayItem) -> String {
        match &played_item.master_metadata_album_artist_name {
            Some(artist_name) => artist_name.to_owned(),
            None => "".to_owned(),
        }
    }

    fn add_time_to_ms_played(&mut self, new_ms_played: &u64) {
        self.ms_played += new_ms_played;
    }

    fn increment_play_count(&mut self) {
        self.play_count += 1;
    }

    fn increment_skip_count(&mut self) {
        self.skip_count += 1;
    }

    fn increment_click_count(&mut self) {
        self.click_count += 1;
    }

    fn increment_shuffle_count(&mut self) {
        self.shuffle_count += 1;
    }

    fn get_ms_played(&self) -> u64 {
        self.ms_played
    }

    fn get_play_count(&self) -> u32 {
        self.play_count
    }

    fn get_skip_pct(&self) -> f64 {
        if self.play_count == 0 {
            0.
        } else {
            self.skip_count as f64 / self.play_count as f64
        }
    }

    fn get_click_pct(&self) -> f64 {
        if self.play_count == 0 {
            0.
        } else {
            self.click_count as f64 / self.play_count as f64
        }
    }

    fn get_shuffle_pct(&self) -> f64 {
        if self.play_count == 0 {
            0.
        } else {
            self.shuffle_count as f64 / self.play_count as f64
        }
    }

    fn played_item_is_valid_for_aggregation(played_item: &PlayItem) -> bool {
        matches!(
            played_item,
            PlayItem {
                master_metadata_album_album_name: Some(_),
                master_metadata_album_artist_name: Some(_),
                master_metadata_track_name: Some(_),
                .. // Ignore all other fields of played_item
            }
        )
    }
}

impl fmt::Display for ArtistData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\nPlay Count: {}\nPlayed For: {}\n",
            self.artist_name,
            self.play_count,
            util::get_total_listen_time_from_ms(self.ms_played)
        )
    }
}

//////////////////////////
// PODCAST EPISODE DATA //
//////////////////////////

#[derive(Clone, Debug, Serialize)]
pub struct EpisodeData {
    pub episode_name: String,
    pub podcast_name: String,
    pub ms_played: u64,
    pub play_count: u32,
    pub skip_count: u32,
    pub click_count: u32,
    pub shuffle_count: u32,
}

impl PlayGroup for EpisodeData {
    fn from_track_info(played_item: &PlayItem) -> Self {
        if let PlayItem {
            episode_show_name: Some(podcast_name),
            episode_name: Some(episode_name),
            ..
        } = played_item
        {
            EpisodeData {
                episode_name: episode_name.to_owned(),
                podcast_name: podcast_name.to_owned(),
                ms_played: 0,
                play_count: 0,
                skip_count: 0,
                click_count: 0,
                shuffle_count: 0,
            }
        } else {
            EpisodeData {
                episode_name: "".to_owned(),
                podcast_name: "".to_owned(),
                ms_played: 0,
                play_count: 0,
                skip_count: 0,
                click_count: 0,
                shuffle_count: 0,
            }
        }
    }

    // Returns the episode URi as the key
    fn get_key(played_item: &PlayItem) -> String {
        match &played_item.spotify_episode_uri {
            Some(episode_uri) => episode_uri.to_owned(),
            None => "".to_owned(),
        }
    }

    fn add_time_to_ms_played(&mut self, new_ms_played: &u64) {
        self.ms_played += new_ms_played;
    }

    fn increment_play_count(&mut self) {
        self.play_count += 1;
    }

    fn increment_skip_count(&mut self) {
        self.skip_count += 1;
    }

    fn increment_click_count(&mut self) {
        self.click_count += 1;
    }

    fn increment_shuffle_count(&mut self) {
        self.shuffle_count += 1;
    }

    fn get_ms_played(&self) -> u64 {
        self.ms_played
    }

    fn get_play_count(&self) -> u32 {
        self.play_count
    }

    fn get_skip_pct(&self) -> f64 {
        if self.play_count == 0 {
            0.
        } else {
            self.skip_count as f64 / self.play_count as f64
        }
    }

    fn get_click_pct(&self) -> f64 {
        if self.play_count == 0 {
            0.
        } else {
            self.click_count as f64 / self.play_count as f64
        }
    }

    fn get_shuffle_pct(&self) -> f64 {
        if self.play_count == 0 {
            0.
        } else {
            self.shuffle_count as f64 / self.play_count as f64
        }
    }

    fn played_item_is_valid_for_aggregation(played_item: &PlayItem) -> bool {
        matches!(
            played_item,
            PlayItem {
                episode_show_name: Some(_),
                episode_name: Some(_),
                .. // Ignore all other fields of played_item
            }
        )
    }
}

impl Display for EpisodeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Episode: {}\nPodcast: {}\nPlay Count: {}\nPlayed For: {}\n",
            self.episode_name,
            self.podcast_name,
            self.play_count,
            util::get_total_listen_time_from_ms(self.ms_played)
        )
    }
}

//////////////////
// PODCAST DATA //
//////////////////

#[derive(Clone, Debug, Serialize)]
pub struct PodcastData {
    pub podcast_name: String,
    pub ms_played: u64,
    pub play_count: u32,
    pub skip_count: u32,
    pub click_count: u32,
    pub shuffle_count: u32,
}

impl PlayGroup for PodcastData {
    fn from_track_info(played_item: &PlayItem) -> Self {
        if let PlayItem {
            episode_show_name: Some(podcast_name),
            ..
        } = played_item
        {
            PodcastData {
                podcast_name: podcast_name.to_owned(),
                ms_played: 0,
                play_count: 0,
                skip_count: 0,
                click_count: 0,
                shuffle_count: 0,
            }
        } else {
            PodcastData {
                podcast_name: "".to_owned(),
                ms_played: 0,
                play_count: 0,
                skip_count: 0,
                click_count: 0,
                shuffle_count: 0,
            }
        }
    }

    // Returns the name of the podcast name as the key
    fn get_key(played_item: &PlayItem) -> String {
        match &played_item.episode_show_name {
            Some(podcast_name) => podcast_name.to_owned(),
            None => "".to_owned(),
        }
    }

    fn add_time_to_ms_played(&mut self, new_ms_played: &u64) {
        self.ms_played += new_ms_played;
    }

    fn increment_play_count(&mut self) {
        self.play_count += 1;
    }

    fn increment_skip_count(&mut self) {
        self.skip_count += 1;
    }

    fn increment_click_count(&mut self) {
        self.click_count += 1;
    }

    fn increment_shuffle_count(&mut self) {
        self.shuffle_count += 1;
    }

    fn get_ms_played(&self) -> u64 {
        self.ms_played
    }

    fn get_play_count(&self) -> u32 {
        self.play_count
    }

    fn get_skip_pct(&self) -> f64 {
        if self.play_count == 0 {
            0.
        } else {
            self.skip_count as f64 / self.play_count as f64
        }
    }

    fn get_click_pct(&self) -> f64 {
        if self.play_count == 0 {
            0.
        } else {
            self.click_count as f64 / self.play_count as f64
        }
    }

    fn get_shuffle_pct(&self) -> f64 {
        if self.play_count == 0 {
            0.
        } else {
            self.shuffle_count as f64 / self.play_count as f64
        }
    }

    fn played_item_is_valid_for_aggregation(played_item: &PlayItem) -> bool {
        matches!(
            played_item,
            PlayItem {
                episode_show_name: Some(_),
                episode_name: Some(_),
                .. // Ignore all other fields of played_item
            }
        )
    }
}

impl Display for PodcastData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Podcast: {}\nPlay Count: {}\nPlayed For: {}\n",
            self.podcast_name,
            self.play_count,
            util::get_total_listen_time_from_ms(self.ms_played)
        )
    }
}

//////////////////////////////////
// GENERATE THE AGGREGATED DATA //
//////////////////////////////////

// Returns aggregated data about the PlayedItems in all_played_items. For instance, this function can return
// the top artists by play count, and it can also get the bottom songs by total listen time.
// pub fn get_aggregated_data(
//     all_played_items: &Vec<PlayItem>,
//     group_by: GroupBy,
// ) -> Vec<SpotifyData> {
//     // Define a hashmap to collect all of the data, with one entry per song
//     let mut aggregated_data: HashMap<String, SpotifyData> = HashMap::new();

//     // For all of the times a song was played
//     for played_item in all_played_items.iter() {
//         // If the pla
//         if T::played_item_is_valid_for_aggregation(played_item) {
//             if let Some(ms_played) = played_item.ms_played {
//                 let entry = aggregated_data
//                     .entry(T::get_key(played_item))
//                     .or_insert_with(|| T::from_track_info(played_item));

//                 entry.add_time_to_ms_played(&ms_played);
//                 entry.increment_play_count();

//                 // TODO: implement checks to increment the other fields of the entry struct
//             }
//         }
//     }

//     let sorted_aggregated_data: Vec<T> = aggregated_data
//         .into_iter()
//         .map(|(_uri, entry)| entry)
//         .collect::<Vec<T>>();

//     // // The sort must happen in-place so the sort call must happen outside of a let statement since the sort
//     // // does not return a Vec<SongEntry>
//     // match sory_by {
//     //     SortSpotifyDataBy::TotalListenTime => {
//     //         sorted_aggregated_data.sort_by(|a, b| a.get_ms_played().cmp(&b.get_ms_played()));
//     //     }
//     //     SortSpotifyDataBy::PlayCount => {
//     //         sorted_aggregated_data.sort_by(|a, b| a.get_play_count().cmp(&b.get_play_count()));
//     //     }
//     //     SortSpotifyDataBy::ClickPct => {
//     //         sorted_aggregated_data.sort_by(|a, b| a.get_click_pct().total_cmp(&b.get_click_pct()));
//     //     }
//     //     SortSpotifyDataBy::ShufflePct => {
//     //         sorted_aggregated_data
//     //             .sort_by(|a, b| a.get_shuffle_pct().total_cmp(&b.get_shuffle_pct()));
//     //     }
//     //     SortSpotifyDataBy::SkipPct => {
//     //         sorted_aggregated_data.sort_by(|a, b| a.get_skip_pct().total_cmp(&b.get_skip_pct()));
//     //     }
//     // }

//     // // The sort by default is in ascending order (i.e. starting at a listen time of 0ms), so reverse it
//     // if sort_descending {
//     //     sorted_aggregated_data.reverse();
//     // }

//     // // Wrap all elements of this in Boxes and return
//     // sorted_aggregated_data
//     //     .into_iter()
//     //     .map(Box::<T>::new)
//     //     .collect()

//     todo!()
// }
