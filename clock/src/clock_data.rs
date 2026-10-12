use slint::{SharedString, ToSharedString};

use crate::ClockData;

pub trait ExtensionClock {
	fn new(name: &str, hours_offset: i32, minutes_offset: i32) -> Self;
	fn utc_with_offset(hours_offset: i32, minutes_offset: i32) -> SharedString;
	fn utc_offset_display(hours_offset: i32, minutes_offset: i32) -> SharedString;
}

impl ExtensionClock for ClockData {
	fn new(name: &str, hours_offset: i32, minutes_offset: i32) -> Self {
		let hours_offset = {
			if hours_offset < -12 { -12 }
			else if hours_offset > 14 { 14 }
			else { hours_offset }
		};
		
		let minutes_offset = {
			if minutes_offset < 0 { 0 }
			else if minutes_offset > 59 { 59 }
			else { minutes_offset }
		};

		Self {
			name: name.to_shared_string(),
			hours_offset,
			minutes_offset,
			time: Self::utc_with_offset(hours_offset, minutes_offset),
			utc_offset_display: Self::utc_offset_display(hours_offset, minutes_offset),
		}
	}

	fn utc_with_offset(hours_offset: i32, minutes_offset: i32) -> SharedString {
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
		).to_shared_string()		
	}

	fn utc_offset_display(hours_offset: i32, minutes_offset: i32) -> SharedString {
		slint::format!("UTC  {:+03}:{:02}", hours_offset, minutes_offset)
	}
}



