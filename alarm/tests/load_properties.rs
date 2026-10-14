#![cfg(test)]

use helpers::slint_helpers::ModelToVec;
use slint::{ComponentHandle, SharedString};
use slint_program::AlarmGlobals;

#[test]
fn preview_mode_is_false() {
	let slint_program = slint_program::slint_test();
	let alarm_globals = slint_program.global::<AlarmGlobals>();

	alarm::load(slint_program.as_weak());

	let actual = alarm_globals.get_preview_mode();
	let expected = false;

	assert_eq!(actual, expected);
}

#[test]
fn hour_scroll_is_0_to_23() {
	let slint_program = slint_program::slint_test();
	let alarm_globals = slint_program.global::<AlarmGlobals>();

	alarm::load(slint_program.as_weak());

	let actual = alarm_globals.get_hour_scroll().to_vec();
	let expected = (0..=23)
		.map(|x| slint::format!("{:02}", x))
		.collect::<Vec<SharedString>>();

	assert_eq!(actual, expected);
}

#[test]
fn minute_scroll_is_0_to_59() {
	let slint_program = slint_program::slint_test();
	let alarm_globals = slint_program.global::<AlarmGlobals>();

	alarm::load(slint_program.as_weak());

	let actual = alarm_globals.get_minute_scroll().to_vec();
	let expected = (0..=59)
		.map(|x| slint::format!("{:02}", x))
		.collect::<Vec<SharedString>>();

	assert_eq!(actual, expected);
}