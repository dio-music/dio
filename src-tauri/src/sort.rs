use crate::group::Group;
use rayon::prelude::*;

/// Enum to represent the different ways that PlayGroup instances can be sorted
#[derive(Clone)]
pub enum SortSpotifyDataBy {
    AutoPlayPct,
    ClickPct,
    PlayCount,
    ShufflePct,
    SkipPct,
    TotalListenTime,
}

pub fn sort_grouped_data(
    grouped_data: &mut Vec<Group>,
    sort_by: SortSpotifyDataBy,
    descending: bool,
) {
    match sort_by {
        SortSpotifyDataBy::AutoPlayPct => {
            grouped_data.par_sort_by(|a, b| {
                let a_autoplay_pct = a.get_aggregated_data().get_autoplay_pct();
                let b_autoplay_pct = b.get_aggregated_data().get_autoplay_pct();

                a_autoplay_pct.partial_cmp(&b_autoplay_pct).unwrap()
            });
        }
        SortSpotifyDataBy::ClickPct => {
            grouped_data.par_sort_by(|a, b| {
                let a_click_pct = a.get_aggregated_data().get_click_pct();
                let b_click_pct = b.get_aggregated_data().get_click_pct();

                a_click_pct.partial_cmp(&b_click_pct).unwrap()
            });
        }
        SortSpotifyDataBy::PlayCount => {
            grouped_data.par_sort_by_key(|e| e.get_aggregated_data().get_play_count());
        }
        SortSpotifyDataBy::ShufflePct => {
            grouped_data.par_sort_by(|a, b| {
                let a_shuffle_pct = a.get_aggregated_data().get_shuffle_pct();
                let b_shuffle_pct = b.get_aggregated_data().get_shuffle_pct();

                a_shuffle_pct.partial_cmp(&b_shuffle_pct).unwrap()
            });
        }
        SortSpotifyDataBy::SkipPct => {
            grouped_data.par_sort_by(|a, b| {
                let a_skip_pct = a.get_aggregated_data().get_skip_pct();
                let b_skip_pct = b.get_aggregated_data().get_skip_pct();

                a_skip_pct.partial_cmp(&b_skip_pct).unwrap()
            });
        }
        SortSpotifyDataBy::TotalListenTime => {
            grouped_data.par_sort_by_key(|e| e.get_aggregated_data().get_ms_played());
        }
    };

    if descending {
        grouped_data.reverse();
    }
}
