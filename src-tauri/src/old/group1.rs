use crate::plays::PlayItem;
use eyre::{eyre, Result};

pub enum GroupBy {
    Album,
    Artist,
    Podcast,
    PodcastEpisode,
    Song,
}

pub struct DataGroup {
    group_by: GroupBy,
    meta_data: MetaData,
    aggregated_data: AggregatedData,
}

impl DataGroup {
    fn from_play_item(played_item: &PlayItem, group_by: GroupBy) -> Self {
        // let meta_data = match group_by {
        //     GroupBy::Album => MetaData::Album {
        //         album_name: played_item.master_metadata_album_album_name,
        //         artist_name: played_item.master_metadata_album_artist_name,
        //     },
        // };

        DataGroup {
            group_by: group_by,
            meta_data: (),
            aggregated_data: AggregatedData::default(),
        }
    }
}

///////////////////////
/* META DATA STRUCTS */
///////////////////////

#[derive(Default)]
struct AlbumMetaData {
    album_name: String,
    artist_name: String,
}

impl AlbumMetaData {
    fn from_play_item(played_item: &PlayItem, group_by: GroupBy) -> Result<Self> {
        // If the PlayItem has all applicable fields, create a new instance using that data.
        if let PlayItem {
            master_metadata_album_album_name: Some(album_name),
            master_metadata_album_artist_name: Some(artist_name),
            ..
        } = played_item
        {
            Ok(AlbumMetaData {
                album_name: album_name.to_owned(),
                artist_name: artist_name.to_owned(),
            })
        // Else, use default data.
        } else {
            Err(eyre!(
                "Not all values needed for initialization are present in played_item."
            ))
        }
    }
}

#[derive(Default)]
struct ArtistMetaData {
    artist_name: String,
}

#[derive(Default)]
struct PodcastMetaData {
    podcast_name: String,
    author_name: String,
}

#[derive(Default)]
struct PodcastEpisodeMetaData {
    episode_name: String,
    podcast_name: String,
    author_name: String,
}

#[derive(Default)]
struct SongMetaData {
    song_name: String,
    album_name: String,
    artist_name: String,
}

pub enum MetaData {
    Album(AlbumMetaData),
    Artist(ArtistMetaData),
    Podcast(PodcastMetaData),
    PodcastEpisode(PodcastEpisodeMetaData),
    Song(SongMetaData),
}

impl MetaData {
    fn from_play_item(played_item: &PlayItem, group_by: GroupBy) -> Self {
        match group_by {
            GroupBy::Album => MetaData::Album(AlbumMetaData::from_play_item(played_item, group_by)),
            GroupBy::Artist => {
                // If the PlayItem has all applicable fields, create a new instance using that data.
                if let PlayItem {
                    master_metadata_album_artist_name: Some(artist_name),
                    ..
                } = played_item
                {
                    MetaData::Artist(ArtistMetaData {
                        artist_name: artist_name.to_owned(),
                    })
                // Else, use default data.
                } else {
                    MetaData::Artist(ArtistMetaData::default())
                }
            }
            _ => todo!(),
        }
    }

    // fn get_key(&self) -> String {
    //     match self {
    //         MetaData::Album {
    //             album_name,
    //             artist_name,
    //         } => format!("{} by {}", album_name, artist_name).to_owned(),
    //         MetaData::Artist { artist_name } => format!("{}", artist_name).to_owned(),
    //         MetaData::Song {
    //             song_name,
    //             album_name,
    //             artist_name,
    //         } => format!(
    //             "{} on the album {} by {}",
    //             song_name, artist_name, album_name
    //         )
    //         .to_owned(),
    //         MetaData::Podcast {
    //             podcast_name,
    //             author_name,
    //         } => format!("{} by {}", podcast_name, author_name).to_owned(),
    //         MetaData::PodcastEpisode {
    //             episode_name,
    //             podcast_name,
    //             author_name,
    //         } => format!(
    //             "{} on the podcast {} by {}",
    //             episode_name, podcast_name, author_name
    //         )
    //         .to_owned(),
    //     }
    // }
}

#[derive(Default)]
struct AggregatedData {
    pub ms_played: u64,
    pub play_count: u32,
    pub skip_count: u32,
    pub click_count: u32,
    pub shuffle_count: u32,
}

impl AggregatedData {
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
}
