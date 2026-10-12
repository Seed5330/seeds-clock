#![cfg(test)]

use crate::find_next_alarm::calculate_minute_difference;

#[test]
fn small_difference() {
	let current_minute = 20;
	let target_minute = 40;

	let actual = calculate_minute_difference(current_minute, target_minute);
	let expected = 20;

	assert_eq!(actual, expected);	
}

#[test]
fn no_difference() {
	let current_minute = 30;
	let target_minute = 30;

	let actual = calculate_minute_difference(current_minute, target_minute);
	let expected = 0;

	assert_eq!(actual, expected);		
}

#[test]
fn large_difference() {
	let current_minute = 0;
	let target_minute = 59;

	let actual = calculate_minute_difference(current_minute, target_minute);
	let expected = 59;

	assert_eq!(actual, expected);		
}

#[test]
fn negative_difference() {
	let current_minute = 50;
	let target_minute = 20;

	let actual = calculate_minute_difference(current_minute, target_minute);
	let expected = -30;

	assert_eq!(actual, expected);	
}