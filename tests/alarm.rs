#![cfg(test)]

mod alarm_tests;


use slint::{ComponentHandle};
use seeds_clock::{
	self, SlintProgram, alarm::{self},
};


#[test]
#[ignore = "Load executable with front end. Start at Alarm tab."]
fn alarm_load() {
	let slint_program = SlintProgram::new()
		.expect("Launching Slint Program!");

	slint_program.set_enabled_tabs([false, true, false, false].into());

	alarm::load(slint_program.as_weak());

	slint_program.run().unwrap();
}