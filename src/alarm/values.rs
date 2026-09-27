use std::rc::Rc;

use slint::{ComponentHandle, SharedString, VecModel};

use crate::{AlarmGlobals, alarm::alarm_struct::Alarm};

pub fn initialize_values(alarm: Rc<Alarm>) {
	let slint_program = alarm.get_weak().unwrap();
	let alarm_globals = slint_program.global::<AlarmGlobals>();

	alarm_globals.set_preview_mode(false);

	let hour_scroll = (0..=23).into_iter()
		.map(|i| slint::format!("{:02}", i))
		.collect::<VecModel<SharedString>>();

	let minute_scroll = (0..=59).into_iter()
		.map(|i| slint::format!("{:02}", i))
		.collect::<VecModel<SharedString>>();

	alarm_globals.set_hour_scroll(Rc::new(hour_scroll).into());
	alarm_globals.set_minute_scroll(Rc::new(minute_scroll).into());
}