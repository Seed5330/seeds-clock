#![cfg(test)]

use crate::find_next_alarm::{Time, time_difference_in_minutes};

#[test]
fn small_minute_difference() {
	let given_time = Time::new(19, 05);
	let target_time = Time::new(19, 10);

	let actual = time_difference_in_minutes(&given_time, &target_time);
	let expected = 5;

	assert_eq!(actual, expected);
}

#[test]
fn difference_by_a_few_hours() {
	let given_time = Time::new(6, 30);
	let target_time = Time::new(11, 30);

	let actual = time_difference_in_minutes(&given_time, &target_time);
	let expected = 5 * 60;

	assert_eq!(actual, expected);	
}

#[test]
fn difference_by_a_few_hours_plus_a_few_minutes() {
	let given_time = Time::new(6, 30);
	let target_time = Time::new(10, 10);

	let actual = time_difference_in_minutes(&given_time, &target_time);
	let expected = (4 * 60) - 20;

	assert_eq!(actual, expected);	
}

#[test]
fn large_difference() {
	let given_time = Time::new(3, 05);
	let target_time = Time::new(22, 50);

	let actual = time_difference_in_minutes(&given_time, &target_time);
	let expected = (19 * 60) + 45;

	assert_eq!(actual, expected);	
}

#[test]
fn the_day_after() {
	let given_time = Time::new(20, 20);
	let target_time = Time::new(5, 30);

	let actual = time_difference_in_minutes(&given_time, &target_time);
	let expected = (9 * 60) + 10;

	assert_eq!(actual, expected);	
}

#[test]
fn same_hour_different_day() {
	let given_time = Time::new(7, 30);
	let target_time = Time::new(7, 20);

	let actual = time_difference_in_minutes(&given_time, &target_time);
	let expected = (24 * 60) - 10;

	assert_eq!(actual, expected);	
}