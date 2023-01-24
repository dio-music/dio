use std::{collections::HashMap, fmt::Display};

use serde::Serialize;

use crate::{plays::PlayItem, util};

pub enum GroupBy {
    Album,
    Artist,
    Song,
    Podcast,
    PodcastEpisode,
}

#[derive(Clone, Serialize)]
pub struct GroupData {
    meta_data: MetaData,
    aggregated_data: AggregatedData,
}

#[derive(Clone, Serialize)]
pub enum Group {
    Album(GroupData),
    Artist(GroupData),
    Song(GroupData),
    Podcast(GroupData),
    PodcastEpisode(GroupData),
}

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

    pub fn get_aggregated_data(&self) -> &AggregatedData {
        match self {
            Self::Album(group_data) => &group_data.aggregated_data,
            Self::Artist(group_data) => &group_data.aggregated_data,
            Self::Song(group_data) => &group_data.aggregated_data,
            Self::Podcast(group_data) => &group_data.aggregated_data,
            Self::PodcastEpisode(group_data) => &group_data.aggregated_data,
        }
    }

    pub fn get_aggregated_data_mut(&mut self) -> &mut AggregatedData {
        match self {
            Self::Album(group_data) => &mut group_data.aggregated_data,
            Self::Artist(group_data) => &mut group_data.aggregated_data,
            Self::Song(group_data) => &mut group_data.aggregated_data,
            Self::Podcast(group_data) => &mut group_data.aggregated_data,
            Self::PodcastEpisode(group_data) => &mut group_data.aggregated_data,
        }
    }

    pub fn get_metadata(&self) -> &MetaData {
        match self {
            Self::Album(group_data) => &group_data.meta_data,
            Self::Artist(group_data) => &group_data.meta_data,
            Self::Song(group_data) => &group_data.meta_data,
            Self::Podcast(group_data) => &group_data.meta_data,
            Self::PodcastEpisode(group_data) => &group_data.meta_data,
        }
    }
}

impl Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let group_data = match self {
            Self::Album(group_data) => group_data,
            Self::Artist(group_data) => group_data,
            Self::Song(group_data) => group_data,
            Self::Podcast(group_data) => group_data,
            Self::PodcastEpisode(group_data) => group_data,
        };

        write!(f,
            "{}\nTotal Listening Time: {}\nPlays: {}\nSkip: {:.2}%\nClick: {:.2}%\nShuffle: {:.2}%\nAutoplay: {:.2}%\n\n",
            group_data.meta_data.as_string(),
            util::get_total_listen_time_from_ms(group_data.aggregated_data.get_ms_played()),
            group_data.aggregated_data.get_play_count(),
            group_data.aggregated_data.get_skip_pct(),
            group_data.aggregated_data.get_click_pct(),
            group_data.aggregated_data.get_shuffle_pct(),
            group_data.aggregated_data.get_autoplay_pct(),
        )
    }
}

/////////////////////
// METADATA STRUCT //
/////////////////////

#[derive(Clone, Serialize)]
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
    fn as_string(&self) -> String {
        match self {
            Self::Album {
                album_name,
                artist_name,
            } => format!("\"{}\" by \"{}\"", album_name, artist_name),
            Self::Artist { artist_name } => format!("\"{}\"", artist_name),
            Self::Song {
                track_name,
                album_name,
                artist_name,
            } => format!(
                "\"{}\" on \"{}\" by \"{}\"",
                track_name, album_name, artist_name
            ),
            Self::Podcast { podcast_name } => format!("\"{}\"", podcast_name),
            Self::PodcastEpisode {
                episode_name,
                podcast_name,
            } => format!("\"{}\" on \"{}\"", episode_name, podcast_name),
        }
    }
}

/////////////////////
// AGGREGATED DATA //
/////////////////////

#[derive(Clone, Default, Serialize)]
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

    pub fn get_skip_pct(&self) -> f32 {
        if self.skip_valid_plays == 0 {
            0.
        } else {
            100. * self.skip_count as f32 / self.skip_valid_plays as f32
        }
    }

    pub fn get_click_pct(&self) -> f32 {
        if self.click_valid_plays == 0 {
            0.
        } else {
            100. * self.click_count as f32 / self.click_valid_plays as f32
        }
    }

    pub fn get_shuffle_pct(&self) -> f32 {
        if self.shuffle_valid_plays == 0 {
            0.
        } else {
            100. * self.shuffle_count as f32 / self.shuffle_valid_plays as f32
        }
    }

    pub fn get_autoplay_pct(&self) -> f32 {
        if self.autoplay_valid_plays == 0 {
            0.
        } else {
            100. * self.autoplay_count as f32 / self.autoplay_valid_plays as f32
        }
    }
}

//////////////////////////
// GROUP RAW PLAY ITEMS //
//////////////////////////

fn update_hash_map_entry(entry: &mut Group, play_item: &PlayItem, ms_played: &u64) {
    let aggregated_data = entry.get_aggregated_data_mut();
    aggregated_data.increment_play_count();
    aggregated_data.add_time_to_ms_played(ms_played);

    if let Some(reason_start) = &play_item.reason_start {
        aggregated_data.add_to_click_count(reason_start);
        aggregated_data.add_to_autoplay_count(reason_start);
    }

    if let Some(skipped) = play_item.skipped {
        aggregated_data.add_to_skip_count(skipped);
    }

    if let Some(shuffled) = play_item.shuffle {
        aggregated_data.add_to_shuffle_count(shuffled);
    }
}

pub fn get_grouped_data(group_by: &GroupBy, played_items: Vec<PlayItem>) -> Vec<Group> {
    let mut grouped_data_map: HashMap<String, Group> = HashMap::new();

    for play_item in played_items.iter() {
        // Play items should be skipped if their ms_played field is None
        let PlayItem {
            ms_played: Some(ms_played),
            ..
        } = play_item else {continue;};

        let group_result = match group_by {
            GroupBy::Album => Group::new_album(play_item),
            GroupBy::Artist => Group::new_artist(play_item),
            GroupBy::Song => Group::new_song(play_item),
            GroupBy::Podcast => Group::new_podcast(play_item),
            GroupBy::PodcastEpisode => Group::new_podcast_episode(play_item),
        };

        // Play items should be skipped if they cannot be successfully turned into a group
        let Ok(group_for_play_item) = group_result else {continue;};

        let key = &group_for_play_item.get_metadata().as_string();

        if !grouped_data_map.contains_key(key) {
            grouped_data_map.insert(key.clone(), group_for_play_item);
        }

        let Some(entry) = grouped_data_map.get_mut(key) else {continue;};
        update_hash_map_entry(entry, play_item, ms_played);
    }

    grouped_data_map.into_values().collect()
}
