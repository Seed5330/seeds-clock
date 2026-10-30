#![cfg(test)]

use clock::offsets::utc_offset_display::utc_offset_display;
use slint::{ComponentHandle, ToSharedString};
use slint_program::ClockGlobals;

#[test]
fn callback_utc_offset_display() {
	let slint_program = slint_program::slint_test();
	let clock_globals = slint_program.global::<ClockGlobals>();

	clock::load(slint_program.as_weak());

	for i in 0..100 {
		let hours_offset = rand::random_range(-12..=14);
		let minutes_offset = rand::random_range(0..=59);

		let expected = utc_offset_display(hours_offset, minutes_offset).to_shared_string();
		let actual = clock_globals.invoke_backend_utc_offset_display(hours_offset, minutes_offset);

		assert_eq!(expected, actual, "Panicked at iteration {}.", i);
	}
}