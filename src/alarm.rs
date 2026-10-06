pub mod unit_tests;
pub mod alarm_struct;
pub mod alarm_data;

pub mod find_next_alarm;
pub mod time_display;


use seeds_common::slint_common::{SlintVec};
use slint::{ComponentHandle, SharedString, ToSharedString, Weak};

use crate::{AlarmGlobals, SlintProgram};

pub fn load(slint_weak: Weak<SlintProgram>) {
	let slint_program = slint_weak.unwrap();
	let alarm_globals = slint_program.global::<AlarmGlobals>();

	alarm_globals.set_preview_mode(false);


	let hour_scroll = (0..=23).into_iter()
		.map(|i| slint::format!("{:02}", i))
		.collect::<Vec<SharedString>>()
		.to_slint_vec();

	alarm_globals.set_hour_scroll(hour_scroll);


	let minute_scroll = (0..=59).into_iter()
		.map(|i| slint::format!("{:02}", i))
		.collect::<Vec<SharedString>>()
		.to_slint_vec();

	alarm_globals.set_minute_scroll(minute_scroll);


	alarm_globals.on_backend_time_display(move |hour, minute| {
		time_display::time_display(hour as usize, minute as usize)
			.expect("Frontend forcefully enters correct values.")
			.to_shared_string()
	});
}