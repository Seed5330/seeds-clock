#![cfg(test)]



use slint::{ComponentHandle};
use slint_program::SlintProgram;


#[test]
#[ignore = "Open executable with front end. Start at Alarm tab."]
fn alarm_open() {
	let slint_program = SlintProgram::new().unwrap();

	slint_program.set_enabled_tabs([false, true, false, false].into());

	alarm::load(slint_program.as_weak());

	slint_program.run().unwrap();
}