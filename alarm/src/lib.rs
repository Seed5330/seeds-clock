pub mod unit_tests;
pub mod alarm_struct;

pub mod find_next_alarm;
pub mod time_display;
pub mod alarm_data;

use helpers::slint_helpers::{VecToModel};
use slint::{ComponentHandle, SharedString, Weak};

use slint_program::{AlarmGlobals, SlintProgram};
pub fn load(slint_weak: Weak<SlintProgram>) {
	let slint_program = slint_weak.unwrap();
	let alarm_globals = slint_program.global::<AlarmGlobals>();

	alarm_globals.set_preview_mode(false);


	let hour_scroll = (0..=23).into_iter()
		.map(|i| slint::format!("{:02}", i))
		.collect::<Vec<SharedString>>()
		.to_model();

	alarm_globals.set_hour_scroll(hour_scroll);


	let minute_scroll = (0..=59).into_iter()
		.map(|i| slint::format!("{:02}", i))
		.collect::<Vec<SharedString>>()
		.to_model();

	alarm_globals.set_minute_scroll(minute_scroll);


	time_display::set_callback_time_display(slint_program.as_weak());
}
