#![cfg(test)]

use slint::ToSharedString;
use crate::{ClockData, clock_data::ExtensionClock};

#[test]
fn new_clock_data() {
	let actual = ClockData::new("Test Clock", 4, 20);
	let expected = ClockData {
		name: "Test Clock".to_shared_string(),
		hours_offset: 4,
		minutes_offset: 20,
		time: ClockData::utc_with_offset(4, 20),
		utc_offset_display: "UTC  +04:20".to_shared_string(),
	};

	assert_eq!(actual, expected);
}

#[test]
fn utc_offset_display() {
	let actual = ClockData::utc_offset_display(5, 30);
	let expected = "UTC  +05:30".to_shared_string();

	assert_eq!(actual, expected);
}
