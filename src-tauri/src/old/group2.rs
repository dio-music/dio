use crate::plays::PlayItem;

pub enum GroupBy {
    Album,
    Artist,
    Song,
    Podcast,
    PodcastEpisode,
}

pub

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
