#![cfg(test)]

use slint::ToSharedString;
use crate::ClockData;

#[test]
fn new_clock_data() {
	let left = ClockData::new("Test Clock", 4, 20);
	let right = ClockData {
		name: "Test Clock".to_shared_string(),
		hours_offset: 4,
		minutes_offset: 20,
		time: ClockData::utc_with_offset(4, 20),
		utc_offset_display: "UTC  +04:20".to_shared_string(),
	};

	assert_eq!(left, right);
}

#[test]
fn utc_offset_display() {
	let left = ClockData::utc_offset_display(5, 30);
	let right = "UTC  +05:30".to_shared_string();

	assert_eq!(left, right);
}
