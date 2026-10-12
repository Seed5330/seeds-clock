#![cfg(test)]

use slint::ToSharedString;

use crate::AlarmData;

#[test]
fn time_display() {
	let actual = AlarmData::get_time_display(3, 5);
	let expected = "03:05".to_shared_string();

	assert_eq!(actual, expected); 
}