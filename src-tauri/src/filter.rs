use crate::group::GroupBy;
use crate::plays::PlayItem;
use chrono::prelude::*;

pub struct Filter {
    pub group_by: GroupBy,
    pub date_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    pub date_range_boundaries: (DateTime<Utc>, DateTime<Utc>),
}

impl Default for Filter {
    fn default() -> Self {
        Filter {
            group_by: GroupBy::Song,
            date_range: None,
            date_range_boundaries: (DateTime::default(), DateTime::default()),
        }
    }
}

pub fn get_play_items_between_dates(
    all_play_items: &[PlayItem],
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
) -> Vec<PlayItem> {
    let mut all_played_items_in_range: Vec<PlayItem> = vec![];

    for single_played_item in all_play_items.iter() {
        if let Some(ts) = &single_played_item.ts {
            if let Ok(timestamp_dt) = ts.parse::<DateTime<Utc>>() {
                if start_date <= timestamp_dt && timestamp_dt <= end_date {
                    all_played_items_in_range.push(single_played_item.clone());
                }
            }
        }
    }

    all_played_items_in_range
}
