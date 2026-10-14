mod unit_tests;

pub mod find_next_alarm;
pub mod alarm_struct;

pub mod time_display;
pub mod alarm_data;
pub mod time_struct;

pub mod find_nearest_time;
pub mod has_time;

use helpers::slint_helpers::{VecToModel};
use slint::{ComponentHandle, Model, SharedString, ToSharedString, Weak};

use slint_program::{AlarmGlobals, SlintProgram};

use crate::{has_time::has_time, time_struct::Time};
pub fn load(slint_weak: Weak<SlintProgram>) {
	let slint_program = slint_weak.unwrap();
	let alarm_globals = slint_program.global::<AlarmGlobals>();

	alarm_globals.set_preview_mode(false);


	let hour_scroll = (0..=23)
		.map(|i| slint::format!("{:02}", i))
		.collect::<Vec<SharedString>>()
		.to_model();

	alarm_globals.set_hour_scroll(hour_scroll);


	let minute_scroll = (0..=59)
		.map(|i| slint::format!("{:02}", i))
		.collect::<Vec<SharedString>>()
		.to_model();

	alarm_globals.set_minute_scroll(minute_scroll);


	alarm_globals.on_backend_time_display(move |hour, minute| {
		time_display::time_display(hour as usize, minute as usize)
			.unwrap_or_else(|_| "ERROR".into())
			.to_shared_string()
	});

	let slint_weak = slint_program.as_weak();
	alarm_globals.on_has_time(move |alarm_time | {
		let slint_program = slint_weak.unwrap();
		let alarm_globals = slint_program.global::<AlarmGlobals>();

		let time = Time::from_alarm_time(alarm_time);
		let time_vec = alarm_globals
			.get_alarm_list()
			.iter()
			.map(|alarm_data| Time::from_alarm_data(&alarm_data))
			.collect::<Vec<Time>>();
		
		has_time(&time_vec, &time)
	});
}
