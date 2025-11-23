use crate::page_manipulation::broken_json::PollingMetrics;
#[cfg(not(test))]
#[cfg(feature = "native")]
use std::time::SystemTime;

#[cfg(not(test))]
#[cfg(feature = "cloudflare")]
use web_time::SystemTime;

#[cfg(test)]
use mock_instant::thread_local::SystemTime;

fn get_end_date(polling_metrics: &PollingMetrics) -> String {
    match polling_metrics.is_active {
        true => {
            let seconds = (polling_metrics.ends_at
                - SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()) as f64
                / 3600.0
                / 24.0;
            format!("Ends in {} days", seconds.ceil())
        }
        false => "Poll ended".to_string(),
    }
}

pub async fn build_ascii_poll(polling_metrics: PollingMetrics) -> String {
    let end_date = get_end_date(&polling_metrics);
    let names: Vec<String> = polling_metrics
        .options
        .into_iter()
        .map(|x| x.display)
        .collect();
    let votes = polling_metrics.counts;

    let percentages: Vec<f64> = votes
        .iter()
        .map(|&v| (v as f64 / polling_metrics.total_votes as f64) * 100.0)
        .collect();

    let bar_max_len = 15;
    let bar_char = '▓';
    let bar_char_padding = '░';

    let mut result = String::with_capacity(500);

    for (name, percent) in names.iter().zip(percentages.iter()) {
        let bar_len = ((percent / 100.0) * bar_max_len as f64).round() as usize;
        let bar_padding_len = bar_max_len - bar_len;
        result.push_str(&format!(
            "█{filling}{padding}█ ({percent:4.01}%) {name}\n",
            filling = bar_char.to_string().repeat(bar_len),
            padding = bar_char_padding.to_string().repeat(bar_padding_len)
        ));
    }

    format!(
        "🗳️ Poll:\n\n{poll}\n{vote_count} votes · {end_date}",
        poll = result.trim_end_matches(">\n"),
        vote_count = polling_metrics.total_votes,
        end_date = end_date,
    )
}
