#[allow(dead_code, unused_variables)]
use std::{collections::HashMap, fmt::Display};

use crate::{plays::PlayItem, util};

// use erased_serde;
use serde::Serialize;

// pub trait Group: Display + Send {
//     fn from_play_item(play_item: &PlayItem) -> Result<Self, ()>
//     where
//         Self: Sized;

//     fn get_aggregated_data(&self) -> &AggregatedData;
//     fn get_aggregated_data_mut(&mut self) -> &mut AggregatedData;
//     fn get_metadata(&self) -> Vec<MetaDataEntry>;

//     fn generate_key(play_item: &PlayItem) -> Result<String, ()>
//     where
//         Self: Sized;
// }

pub enum GroupBy {
    Album,
    Artist,
    Song,
    Podcast,
    PodcastEpisode,
}

pub struct GroupData {
    meta_data: MetaData,
    aggregated_data: AggregatedData,
}

pub enum Group {
    Album(GroupData),
    Artist(GroupData),
    Song(GroupData),
    Podcast(GroupData),
    PodcastEpisode(GroupData),
}
//
impl Group {
    fn new_album(play_item: &PlayItem) -> Result<Self, ()> {
        let Some(album_name) = play_item.master_metadata_album_album_name.to_owned() else {
            return Err(());
        };

        let Some(artist_name) = play_item.master_metadata_album_artist_name.to_owned() else {
            return Err(());
        };

        let meta_data = MetaData::Album {
            album_name,
            artist_name,
        };

        let aggregated_data = AggregatedData::default();

        Ok(Self::Album(GroupData {
            meta_data,
            aggregated_data,
        }))
    }

    fn new_artist(play_item: &PlayItem) -> Result<Self, ()> {
        let Some(artist_name) = play_item.master_metadata_album_artist_name.to_owned() else {
            return Err(());
        };

        let meta_data = MetaData::Artist { artist_name };

        let aggregated_data = AggregatedData::default();

        Ok(Self::Artist(GroupData {
            meta_data,
            aggregated_data,
        }))
    }

    fn new_song(play_item: &PlayItem) -> Result<Self, ()> {
        let Some(track_name) = play_item.master_metadata_track_name.to_owned() else {
            return Err(());
        };

        let Some(album_name) = play_item.master_metadata_album_album_name.to_owned() else {
            return Err(());
        };

        let Some(artist_name) = play_item.master_metadata_album_artist_name.to_owned() else {
            return Err(());
        };

        let meta_data = MetaData::Song {
            track_name,
            album_name,
            artist_name,
        };

        let aggregated_data = AggregatedData::default();

        Ok(Self::Song(GroupData {
            meta_data,
            aggregated_data,
        }))
    }

    fn new_podcast(play_item: &PlayItem) -> Result<Self, ()> {
        let Some(podcast_name) = play_item.episode_show_name.to_owned() else {
            return Err(());
        };

        let meta_data = MetaData::Podcast { podcast_name };

        let aggregated_data = AggregatedData::default();

        Ok(Self::Podcast(GroupData {
            meta_data,
            aggregated_data,
        }))
    }

    fn new_podcast_episode(play_item: &PlayItem) -> Result<Self, ()> {
        let Some(episode_name) = play_item.episode_name.to_owned() else {
            return Err(());
        };

        let Some(podcast_name) = play_item.episode_show_name.to_owned() else {
            return Err(());
        };

        let meta_data = MetaData::PodcastEpisode {
            episode_name,
            podcast_name,
        };

        let aggregated_data = AggregatedData::default();

        Ok(Self::PodcastEpisode(GroupData {
            meta_data,
            aggregated_data,
        }))
    }

    fn get_aggregated_data(&self) -> &AggregatedData {
        match self {
            Self::Album(group_data) => &group_data.aggregated_data,
            Self::Artist(group_data) => &group_data.aggregated_data,
            Self::Song(group_data) => &group_data.aggregated_data,
            Self::Podcast(group_data) => &group_data.aggregated_data,
            Self::PodcastEpisode(group_data) => &group_data.aggregated_data,
        }
    }

    fn get_aggregated_data_mut(&mut self) -> &mut AggregatedData {
        match self {
            Self::Album(group_data) => &mut group_data.aggregated_data,
            Self::Artist(group_data) => &mut group_data.aggregated_data,
            Self::Song(group_data) => &mut group_data.aggregated_data,
            Self::Podcast(group_data) => &mut group_data.aggregated_data,
            Self::PodcastEpisode(group_data) => &mut group_data.aggregated_data,
        }
    }

    fn get_metadata(&self) -> &MetaData {
        match self {
            Self::Album(group_data) => &group_data.meta_data,
            Self::Artist(group_data) => &group_data.meta_data,
            Self::Song(group_data) => &group_data.meta_data,
            Self::Podcast(group_data) => &group_data.meta_data,
            Self::PodcastEpisode(group_data) => &group_data.meta_data,
        }
    }

    fn generate_key(play_item: &PlayItem) -> Result<String, ()> {
        todo!()
    }
}

impl Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

/////////////////////
// METADATA STRUCT //
/////////////////////

pub enum MetaData {
    Album {
        album_name: String,
        artist_name: String,
    },
    Artist {
        artist_name: String,
    },
    Song {
        track_name: String,
        album_name: String,
        artist_name: String,
    },
    Podcast {
        podcast_name: String,
    },
    PodcastEpisode {
        episode_name: String,
        podcast_name: String,
    },
}

impl MetaData {
    fn get_key(&self) -> String {
        match self {
            Self::Album {
                album_name,
                artist_name,
            } => format!("{} by {}", album_name, artist_name),
            Self::Artist { artist_name } => format!("{}", artist_name),
            Self::Song {
                track_name,
                album_name,
                artist_name,
            } => format!("{} on {} by {}", track_name, album_name, artist_name),
            Self::Podcast { podcast_name } => format!("{}", podcast_name),
            Self::PodcastEpisode {
                episode_name,
                podcast_name,
            } => format!("{} on {}", episode_name, podcast_name),
        }
    }
}

/////////////////////
// AGGREGATED DATA //
/////////////////////

#[derive(Default, Serialize, Clone, Copy)]
pub struct AggregatedData {
    pub ms_played: u64,
    pub play_count: u32,
    pub skip_count: u32,
    pub skip_valid_plays: u32,
    pub click_count: u32,
    pub click_valid_plays: u32,
    pub shuffle_count: u32,
    pub shuffle_valid_plays: u32,
    pub autoplay_count: u32,
    pub autoplay_valid_plays: u32,
}

impl AggregatedData {
    fn add_time_to_ms_played(&mut self, new_ms_played: &u64) {
        self.ms_played += new_ms_played;
    }

    fn increment_play_count(&mut self) {
        self.play_count += 1;
    }

    fn add_to_skip_count(&mut self, skipped: bool) {
        if skipped {
            self.skip_count += 1;
        }
        self.skip_valid_plays += 1;
    }

    fn add_to_shuffle_count(&mut self, shuffled: bool) {
        if shuffled {
            self.shuffle_count += 1;
        }
        self.shuffle_valid_plays += 1;
    }

    fn add_to_click_count(&mut self, reason_start: &String) {
        if reason_start.eq_ignore_ascii_case("clickrow") {
            self.click_count += 1;
        }
        self.click_valid_plays += 1;
    }

    fn add_to_autoplay_count(&mut self, reason_start: &String) {
        if reason_start.eq_ignore_ascii_case("trackdone") {
            self.autoplay_count += 1;
        }
        self.autoplay_valid_plays += 1;
    }

    pub fn get_ms_played(&self) -> u64 {
        self.ms_played
    }

    pub fn get_play_count(&self) -> u32 {
        self.play_count
    }

    pub fn get_skip_pct(&self) -> f64 {
        if self.play_count == 0 {
            0.
        } else {
            self.skip_count as f64 / self.play_count as f64
        }
    }

    pub fn get_click_pct(&self) -> f64 {
        if self.play_count == 0 {
            0.
        } else {
            self.click_count as f64 / self.play_count as f64
        }
    }

    pub fn get_shuffle_pct(&self) -> f64 {
        if self.play_count == 0 {
            0.
        } else {
            self.shuffle_count as f64 / self.play_count as f64
        }
    }

    pub fn get_autoplay_pct(&self) -> f64 {
        if self.play_count == 0 {
            0.
        } else {
            self.autoplay_count as f64 / self.play_count as f64
        }
    }
}
