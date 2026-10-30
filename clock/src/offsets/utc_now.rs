pub fn utc_now(hours_offset: i32, minutes_offset: i32) -> String {
	use chrono::{Utc, FixedOffset, Timelike};
	
	let utc_time = Utc::now();
	let total_offset_in_seconds = (3600 * hours_offset) + (60 * minutes_offset);

	let offset = FixedOffset::east_opt(total_offset_in_seconds).unwrap();
	let time = utc_time.with_timezone(&offset);

	format!(
		"{:02}:{:02}:{:02}",
		time.hour(),
		time.minute(),
		time.second(),
	)	
}