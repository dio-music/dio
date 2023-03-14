use crate::plays::PlayItem;
use chrono::prelude::*;

fn get_datetime_from_play_item(play_item: &PlayItem) -> Option<DateTime<Utc>> {
    let mut dt_result: Option<DateTime<Utc>> = None;

    if let Some(ts) = &play_item.ts {
        if let Ok(timestamp_dt) = ts.parse::<DateTime<Utc>>() {
            dt_result = Some(timestamp_dt);
        }
    }

    dt_result
}

pub fn get_date_bounds_from_play_items(
    all_play_items: &Vec<PlayItem>,
) -> Result<(DateTime<Utc>, DateTime<Utc>), String> {
    // DOC:
    let Some(play_item_with_min_datetime) = all_play_items
        .iter()
        .clone()
        .min_by_key(|a| get_datetime_from_play_item(a)) else {
            return Err("TODO:".to_owned());
        };

    // DOC:
    let Some(play_item_with_max_datetime) = all_play_items
        .iter()
        .clone()
        .max_by_key(|a| get_datetime_from_play_item(a)) else {
            return Err("TODO:".to_owned());
        };

    // DOC:
    let Some(min_datetime) = get_datetime_from_play_item(play_item_with_min_datetime) else {
        return Err("TODO:".to_owned());
    };

    // DOC:
    let Some(max_datetime) = get_datetime_from_play_item(play_item_with_max_datetime) else {
        return Err("TODO:".to_owned());
    };

    Ok((min_datetime, max_datetime))
}
