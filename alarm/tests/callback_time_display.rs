#![cfg(test)]

use alarm::time_display;
use slint::ComponentHandle;
use slint_program::AlarmGlobals;



#[test]
fn callback_time_display() {
	let slint_program = slint_program::slint_test();
	let alarm_globals = slint_program.global::<AlarmGlobals>();

	alarm::load(slint_program.as_weak());

	for i in 0..100 {
		let time_value = (rand::random_range(0..30), rand::random_range(0..72));	
		let time_display = time_display::time_display(time_value.0 as usize, time_value.1 as usize);
	
		let actual = alarm_globals.invoke_time_display(time_value.0, time_value.1);
		if let Ok(expected) = time_display {
			assert_eq!(actual, expected, "Panicked at iteration {}.", i);
		}
		else {
			let expected = "ERROR";
			assert_eq!(actual, expected, "Panicked at iteration {}.", i);
		}
	}
}











