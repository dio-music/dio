use crate::plays::PlayItem;
use chrono::prelude::*;

pub struct Filter {
    pub date_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    pub date_range_boundaries: (DateTime<Utc>, DateTime<Utc>),
    // pub
}

impl Default for Filter {
    fn default() -> Self {
        Filter {
            date_range: None,
            date_range_boundaries: (DateTime::default(), DateTime::default()),
        }
    }
}

// impl Filter {
//     fn
// }

// TODO: Allow users to do things like "filter for all songs with play count above 100" or
// "filter for all artists with skip % below 25%", then sort those results with the normal sorts

pub fn get_play_items_between_dates(
    all_play_items: &Vec<PlayItem>,
    filter: &Filter,
) -> Vec<PlayItem> {
    let (start_date, end_date) = match filter.date_range {
        Some((start_date, end_date)) => (start_date, end_date),
        None => filter.date_range_boundaries,
    };

    let mut play_items_in_range: Vec<PlayItem> = Vec::new();

    for single_played_item in all_play_items.iter() {
        if let Some(ts) = &single_played_item.ts {
            if let Ok(timestamp_dt) = ts.parse::<DateTime<Utc>>() {
                if start_date <= timestamp_dt && timestamp_dt <= end_date {
                    play_items_in_range.push(single_played_item.clone());
                }
            }
        }
    }

    play_items_in_range
}
