#![cfg(test)]

use helpers::slint_helpers::ModelToVec;
use slint::{ComponentHandle, SharedString};
use slint_program::ClockGlobals;


#[test]
fn preview_mode_is_false() {
	let slint_program = slint_program::slint_test();
	let clock_globals = slint_program.global::<ClockGlobals>();

	clock::load(slint_program.as_weak());

	let actual = clock_globals.get_preview_mode();
	let expected = false;

	assert_eq!(actual, expected);
}

#[test]
fn hours_offset_scroll() {
	let slint_program = slint_program::slint_test();
	let clock_globals = slint_program.global::<ClockGlobals>();

	clock::load(slint_program.as_weak());

	let expected = (-12..=14)
		.map(|x| slint::format!("{:+03}", x))
		.collect::<Vec<SharedString>>();
	let actual = clock_globals
		.get_hours_offset_scroll()
		.to_vec();

	pretty_assertions::assert_eq!(expected, actual);
}

#[test]
fn minutes_offset_scroll() {
	let slint_program = slint_program::slint_test();
	let clock_globals = slint_program.global::<ClockGlobals>();

	clock::load(slint_program.as_weak());

	let expected = (0..=59)
		.map(|x| slint::format!("{:02}", x))
		.collect::<Vec<SharedString>>();
	let actual = clock_globals
		.get_minutes_offset_scroll()
		.to_vec();

	pretty_assertions::assert_eq!(expected, actual);
}