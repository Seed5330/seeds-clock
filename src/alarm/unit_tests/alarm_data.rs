#![cfg(test)]

use slint::ToSharedString;

use crate::AlarmData;

#[test]
fn time_display() {
	let left = AlarmData::get_time_display(3, 5);
	let right = "03:05".to_shared_string();

	assert_eq!(left, right); 
}