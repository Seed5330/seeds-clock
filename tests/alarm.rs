#![cfg(test)]

mod alarm_tests;
use std::rc::Rc;


use slint::{ComponentHandle};
use seeds_clock::{
	self,
	SlintProgram,
	alarm::{alarm_struct::Alarm, callbacks, values},
};


#[test]
#[ignore = "Load executable with front end. Start at Alarm tab."]
fn alarm_load() {
	let slint_program = SlintProgram::new()
		.expect("Launching Slint Program!");

	slint_program.set_enabled_tabs([false, true, false, false].into());

	let alarm = Alarm::new(slint_program.as_weak());
	let alarm_rc = Rc::new(alarm);

	values::initialize_values(alarm_rc.clone());
	callbacks::initialize_callbacks(alarm_rc.clone());

	slint_program.run().unwrap();
}