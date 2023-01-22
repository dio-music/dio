use std::{collections::HashMap, fmt::Display};

use crate::{plays::PlayItem, util};

use erased_serde;
use serde::Serialize;

pub trait Group: Display + Send + erased_serde::Serialize {
    fn from_play_item(play_item: &PlayItem) -> Result<Self, ()>
    where
        Self: Sized;

    fn get_aggregated_data(&self) -> &AggregatedData;
    fn get_aggregated_data_mut(&mut self) -> &mut AggregatedData;
    fn get_metadata(&self) -> Vec<MetaDataEntry>;

    fn generate_key(play_item: &PlayItem) -> Result<String, ()>
    where
        Self: Sized;
}

pub enum GroupBy {
    Album,
    Artist,
    Song,
    Podcast,
    PodcastEpisode,
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

/////////////////////
// METADATA STRUCT //
/////////////////////

pub enum MetaDataType {
    Album,
    Artist,
    Song,
    Podcast,
    PodcastEpisode,
}

pub struct MetaDataEntry {
    field_type: MetaDataType,
    field_data: String,
}

///////////
// ALBUM //
///////////

#[derive(Serialize)]
pub struct Album {
    album_name: String,
    artist_name: String,
    aggregated_data: AggregatedData,
}
impl Group for Album {
    fn from_play_item(play_item: &PlayItem) -> Result<Self, ()>
    where
        Self: Sized,
    {
        let Some(album_name) = play_item.master_metadata_album_album_name.to_owned() else {
            return Err(());
        };

        let Some(artist_name) = play_item.master_metadata_album_artist_name.to_owned() else {
            return Err(());
        };

        let aggregated_data = AggregatedData::default();

        Ok(Album {
            album_name,
            artist_name,
            aggregated_data,
        })
    }

    fn generate_key(play_item: &PlayItem) -> Result<String, ()>
    where
        Self: Sized,
    {
        let Some(album_name) = play_item.master_metadata_album_album_name.to_owned() else {
            return Err(());
        };

        let Some(artist_name) = play_item.master_metadata_album_artist_name.to_owned() else {
            return Err(());
        };

        let mut key_str = "".to_owned();
        key_str.push_str(&album_name);
        key_str.push_str(&artist_name);

        Ok(key_str)
    }

    fn get_aggregated_data(&self) -> &AggregatedData {
        &self.aggregated_data
    }

    fn get_aggregated_data_mut(&mut self) -> &mut AggregatedData {
        &mut self.aggregated_data
    }

    fn get_metadata(&self) -> Vec<MetaDataEntry> {
        let mut result: Vec<MetaDataEntry> = vec![];

        result.push(MetaDataEntry {
            field_type: MetaDataType::Album,
            field_data: self.album_name.clone(),
        });

        result.push(MetaDataEntry {
            field_type: MetaDataType::Artist,
            field_data: self.artist_name.clone(),
        });

        result
    }
}
impl Display for Album {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\"{}\" by \"{}\"\nTotal Time: {}\nPlays: {}\n",
            self.album_name,
            self.artist_name,
            util::get_total_listen_time_from_ms(self.aggregated_data.ms_played),
            self.aggregated_data.play_count
        )
    }
}

////////////
// ARTIST //
////////////

#[derive(Serialize)]
pub struct Artist {
    artist_name: String,
    aggregated_data: AggregatedData,
}
impl Group for Artist {
    fn from_play_item(play_item: &PlayItem) -> Result<Self, ()>
    where
        Self: Sized,
    {
        let Some(artist_name) = play_item.master_metadata_album_artist_name.to_owned() else {
            return Err(());
        };

        let aggregated_data = AggregatedData::default();

        Ok(Artist {
            artist_name,
            aggregated_data,
        })
    }

    fn generate_key(play_item: &PlayItem) -> Result<String, ()>
    where
        Self: Sized,
    {
        let Some(artist_name) = play_item.master_metadata_album_artist_name.to_owned() else {
            return Err(());
        };

        Ok(artist_name)
    }

    fn get_aggregated_data(&self) -> &AggregatedData {
        &self.aggregated_data
    }

    fn get_aggregated_data_mut(&mut self) -> &mut AggregatedData {
        &mut self.aggregated_data
    }

    fn get_metadata(&self) -> Vec<MetaDataEntry> {
        let mut result: Vec<MetaDataEntry> = vec![];

        result.push(MetaDataEntry {
            field_type: MetaDataType::Artist,
            field_data: self.artist_name.clone(),
        });

        result
    }
}
impl Display for Artist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\"{}\"\nTotal Time: {}\nPlays: {}\n",
            self.artist_name,
            util::get_total_listen_time_from_ms(self.aggregated_data.ms_played),
            self.aggregated_data.play_count
        )
    }
}

//////////
// SONG //
//////////

#[derive(Serialize)]
pub struct Song {
    track_name: String,
    album_name: String,
    artist_name: String,
    aggregated_data: AggregatedData,
}
impl Group for Song {
    fn from_play_item(play_item: &PlayItem) -> Result<Self, ()>
    where
        Self: Sized,
    {
        let Some(track_name) = play_item.master_metadata_track_name.to_owned() else {
            return Err(());
        };

        let Some(album_name) = play_item.master_metadata_album_album_name.to_owned() else {
            return Err(());
        };

        let Some(artist_name) = play_item.master_metadata_album_artist_name.to_owned() else {
            return Err(());
        };

        let aggregated_data = AggregatedData::default();

        Ok(Song {
            track_name,
            album_name,
            artist_name,
            aggregated_data,
        })
    }

    fn generate_key(play_item: &PlayItem) -> Result<String, ()>
    where
        Self: Sized,
    {
        let Some(track_name) = play_item.master_metadata_track_name.to_owned() else {
            return Err(());
        };

        let Some(album_name) = play_item.master_metadata_album_album_name.to_owned() else {
            return Err(());
        };

        let Some(artist_name) = play_item.master_metadata_album_artist_name.to_owned() else {
            return Err(());
        };

        let mut key_str = "".to_owned();
        key_str.push_str(&track_name);
        key_str.push_str(&album_name);
        key_str.push_str(&artist_name);

        Ok(key_str)
    }

    fn get_aggregated_data(&self) -> &AggregatedData {
        &self.aggregated_data
    }

    fn get_aggregated_data_mut(&mut self) -> &mut AggregatedData {
        &mut self.aggregated_data
    }

    fn get_metadata(&self) -> Vec<MetaDataEntry> {
        let mut result: Vec<MetaDataEntry> = vec![];

        result.push(MetaDataEntry {
            field_type: MetaDataType::Song,
            field_data: self.track_name.clone(),
        });

        result.push(MetaDataEntry {
            field_type: MetaDataType::Album,
            field_data: self.album_name.clone(),
        });

        result.push(MetaDataEntry {
            field_type: MetaDataType::Artist,
            field_data: self.artist_name.clone(),
        });

        result
    }
}
impl Display for Song {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\"{}\" on \"{}\" by \"{}\"\nTotal Time: {}\nPlays: {}\n",
            self.track_name,
            self.album_name,
            self.artist_name,
            util::get_total_listen_time_from_ms(self.aggregated_data.ms_played),
            self.aggregated_data.play_count
        )
    }
}

/////////////
// PODCAST //
/////////////

#[derive(Serialize)]
pub struct Podcast {
    podcast_name: String,
    aggregated_data: AggregatedData,
}
impl Group for Podcast {
    fn from_play_item(play_item: &PlayItem) -> Result<Self, ()>
    where
        Self: Sized,
    {
        let Some(podcast_name) = play_item.episode_show_name.to_owned() else {
            return Err(());
        };

        let aggregated_data = AggregatedData::default();

        Ok(Podcast {
            podcast_name,
            aggregated_data,
        })
    }

    fn generate_key(play_item: &PlayItem) -> Result<String, ()>
    where
        Self: Sized,
    {
        let Some(podcast_name) = play_item.episode_show_name.to_owned() else {
            return Err(());
        };

        Ok(podcast_name)
    }

    fn get_aggregated_data(&self) -> &AggregatedData {
        &self.aggregated_data
    }

    fn get_aggregated_data_mut(&mut self) -> &mut AggregatedData {
        &mut self.aggregated_data
    }

    fn get_metadata(&self) -> Vec<MetaDataEntry> {
        let mut result: Vec<MetaDataEntry> = vec![];

        result.push(MetaDataEntry {
            field_type: MetaDataType::Podcast,
            field_data: self.podcast_name.clone(),
        });

        result
    }
}
impl Display for Podcast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\"{}\"\nTotal Time: {}\nPlays: {}\n",
            self.podcast_name,
            util::get_total_listen_time_from_ms(self.aggregated_data.ms_played),
            self.aggregated_data.play_count
        )
    }
}

/////////////////////
// PODCAST EPISODE //
/////////////////////

#[derive(Serialize)]
pub struct PodcastEpisode {
    episode_name: String,
    podcast_name: String,
    aggregated_data: AggregatedData,
}
impl Group for PodcastEpisode {
    fn from_play_item(play_item: &PlayItem) -> Result<Self, ()>
    where
        Self: Sized,
    {
        let Some(episode_name) = play_item.episode_name.to_owned() else {
            return Err(());
        };

        let Some(podcast_name) = play_item.episode_show_name.to_owned() else {
            return Err(());
        };

        let aggregated_data = AggregatedData::default();

        Ok(PodcastEpisode {
            episode_name,
            podcast_name,
            aggregated_data,
        })
    }

    fn generate_key(play_item: &PlayItem) -> Result<String, ()>
    where
        Self: Sized,
    {
        let Some(episode_name) = play_item.episode_name.to_owned() else {
            return Err(());
        };

        let Some(podcast_name) = play_item.episode_show_name.to_owned() else {
            return Err(());
        };

        let mut key_str = "".to_owned();
        key_str.push_str(&episode_name);
        key_str.push_str(&podcast_name);

        Ok(key_str)
    }

    fn get_aggregated_data(&self) -> &AggregatedData {
        &self.aggregated_data
    }

    fn get_aggregated_data_mut(&mut self) -> &mut AggregatedData {
        &mut self.aggregated_data
    }

    fn get_metadata(&self) -> Vec<MetaDataEntry> {
        let mut result: Vec<MetaDataEntry> = vec![];

        result.push(MetaDataEntry {
            field_type: MetaDataType::PodcastEpisode,
            field_data: self.episode_name.clone(),
        });

        result.push(MetaDataEntry {
            field_type: MetaDataType::Podcast,
            field_data: self.podcast_name.clone(),
        });

        result
    }
}
impl Display for PodcastEpisode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\"{}\" on \"{}\"\nTotal Time: {}\nPlays: {}\n",
            self.episode_name,
            self.podcast_name,
            util::get_total_listen_time_from_ms(self.aggregated_data.ms_played),
            self.aggregated_data.play_count
        )
    }
}

/////////////////////////////////////////////////////
// GROUPING RAW PlayItems INTO <dyn Group> OBJECTS //
/////////////////////////////////////////////////////

fn get_key(group_by: &GroupBy, play_item: &PlayItem) -> Result<String, ()> {
    match group_by {
        GroupBy::Album => Album::generate_key(play_item),
        GroupBy::Artist => Artist::generate_key(play_item),
        GroupBy::Song => Song::generate_key(play_item),
        GroupBy::Podcast => Podcast::generate_key(play_item),
        GroupBy::PodcastEpisode => PodcastEpisode::generate_key(play_item),
    }
}

fn create_entry(group_by: &GroupBy, play_item: &PlayItem) -> Result<Box<dyn Group>, ()> {
    let entry: Box<dyn Group> = match group_by {
        GroupBy::Album => {
            let Ok(album_group) = Album::from_play_item(play_item) else {
                return Err(());
            };

            Box::new(album_group)
        }
        GroupBy::Artist => {
            let Ok(artist_group) = Artist::from_play_item(play_item) else {
                return Err(());
            };

            Box::new(artist_group)
        }
        GroupBy::Song => {
            let Ok(song_group) = Song::from_play_item(play_item) else {
                return Err(());
            };

            Box::new(song_group)
        }
        GroupBy::Podcast => {
            let Ok(podcast_group) = Podcast::from_play_item(play_item) else {
                return Err(());
            };

            Box::new(podcast_group)
        }
        GroupBy::PodcastEpisode => {
            let Ok(episode_group) = PodcastEpisode::from_play_item(play_item) else {
                return Err(());
            };

            Box::new(episode_group)
        }
    };

    Ok(entry)
}

pub fn get_grouped_data(group_by: &GroupBy, played_items: &Vec<PlayItem>) -> Vec<Box<dyn Group>> {
    let mut grouped_data_map: HashMap<String, Box<dyn Group>> = HashMap::new();

    for play_item in played_items.iter() {
        if let PlayItem {
            ms_played: Some(ms_played),
            ..
        } = play_item
        {
            let Ok(key) = get_key(group_by, play_item) else {continue;};

            if !grouped_data_map.contains_key(&key) {
                let Ok(new_entry) = create_entry(group_by, play_item) else {continue;};

                grouped_data_map.insert(key.clone(), new_entry);
            }

            let Some(entry) = grouped_data_map.get_mut(&key) else {continue;};

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
        } else {
            continue;
        }
    }

    grouped_data_map.into_values().collect()
}

serialize_trait_object!(Group);
