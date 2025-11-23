mod extractor;
mod input_structs;
mod interest_extractor;
mod interest_to_emoji;
mod output_struct;

pub use extractor::extract_data_from_broken_json;
use input_structs::BrokenJson;
pub use input_structs::{Polling, PollingMetrics};
use interest_extractor::get_interest;
pub use interest_to_emoji::get_interest_emoji;
pub use output_struct::HtmlNineGagBrokenJson;
