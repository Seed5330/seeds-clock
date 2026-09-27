use std::rc::Rc;

use slint::{ComponentHandle};

use crate::{AlarmData, AlarmGlobals, alarm::alarm_struct::Alarm};

pub fn initialize_callbacks(alarm: Rc<Alarm>) {
	let slint_program = alarm.get_weak().unwrap();
	let alarm_globals = slint_program.global::<AlarmGlobals>();

	alarm_settings(alarm.clone());

	alarm_globals.on_get_time_display(move |hour, minute| {
		AlarmData::get_time_display(hour as usize, minute as usize)
	});

	let alarm_clone = alarm.clone();
	alarm_globals.on_has_time(move |hour, minute| {
		alarm_clone.has_time((hour, minute))
	});
}

fn alarm_settings(alarm: Rc<Alarm>) {
	let slint_program = alarm.get_weak().unwrap();
	let alarm_globals = slint_program.global::<AlarmGlobals>();

	let alarm_clone = alarm.clone();
	alarm_globals.on_add_alarm(move |alarm_data| {
		alarm_clone.add_alarm(alarm_data);
	});

	let alarm_clone = alarm.clone();
	alarm_globals.on_remove_alarm(move |index| {
		alarm_clone.remove_alarm(index as usize);
	});

	let alarm_clone = alarm.clone();
	alarm_globals.on_modify_alarm(move |index, alarm_data| {
		alarm_clone.modify_alarm(index as usize, alarm_data);
	});		
}