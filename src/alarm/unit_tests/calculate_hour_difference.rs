#![cfg(test)]

use crate::alarm::find_next_alarm::calculate_hour_difference;

#[test]
fn hour_difference() {
	let current_hour = 20;
	let target_hour = 23;

	let left = calculate_hour_difference(current_hour, target_hour);
	let right = 3;

	assert_eq!(left, right);
}

#[test]
fn hour_from_next_day() {
	let current_hour = 20;
	let target_hour = 6;

	let left = calculate_hour_difference(current_hour, target_hour);
	let right = 10;

	assert_eq!(left, right);	
}

#[test]
fn large_difference() {
	let current_hour = 22;
	let target_hour = 21;

	let left = calculate_hour_difference(current_hour, target_hour);
	let right = 23;

	assert_eq!(left, right);	
}

#[test]
fn no_difference() {
	let current_hour = 22;
	let target_hour = 22;

	let left = calculate_hour_difference(current_hour, target_hour);
	let right = 0;

	assert_eq!(left, right);		
}