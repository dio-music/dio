use crate::group;

/// Enum to represent the different ways that PlayGroup instances can be sorted
pub enum SortSpotifyDataBy {
    TotalListenTime,
    PlayCount,
    ClickPct,
    ShufflePct,
    SkipPct,
    AutoPlayPct,
}

pub fn sort_grouped_data(
    grouped_data: &mut Vec<Box<dyn group::Group>>,
    sort_by: SortSpotifyDataBy,
    descending: bool,
) {
    match sort_by {
        SortSpotifyDataBy::TotalListenTime => {
            grouped_data.sort_by_key(|e| e.get_aggregated_data().get_ms_played());
        }
        SortSpotifyDataBy::PlayCount => {
            grouped_data.sort_by_key(|e| e.get_aggregated_data().get_play_count());
        }
        SortSpotifyDataBy::ClickPct => {
            grouped_data.sort_by(|a, b| {
                let a_click_pct = a.get_aggregated_data().get_click_pct();
                let b_click_pct = b.get_aggregated_data().get_click_pct();

                a_click_pct.partial_cmp(&b_click_pct).unwrap()
            });
        }
        SortSpotifyDataBy::ShufflePct => {
            grouped_data.sort_by(|a, b| {
                let a_shuffle_pct = a.get_aggregated_data().get_shuffle_pct();
                let b_shuffle_pct = b.get_aggregated_data().get_shuffle_pct();

                a_shuffle_pct.partial_cmp(&b_shuffle_pct).unwrap()
            });
        }
        SortSpotifyDataBy::SkipPct => {
            grouped_data.sort_by(|a, b| {
                let a_skip_pct = a.get_aggregated_data().get_skip_pct();
                let b_skip_pct = b.get_aggregated_data().get_skip_pct();

                a_skip_pct.partial_cmp(&b_skip_pct).unwrap()
            });
        }
        SortSpotifyDataBy::AutoPlayPct => {
            grouped_data.sort_by(|a, b| {
                let a_autoplay_pct = a.get_aggregated_data().get_autoplay_pct();
                let b_autoplay_pct = b.get_aggregated_data().get_autoplay_pct();

                a_autoplay_pct.partial_cmp(&b_autoplay_pct).unwrap()
            });
        }
    };

    if descending {
        grouped_data.reverse();
    }
}
