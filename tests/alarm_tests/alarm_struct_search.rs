#![cfg(test)]

use std::rc::Rc;

use seeds_clock::{AlarmData, alarm::alarm_struct::Alarm, common};
use slint::{ComponentHandle, ToSharedString};

#[test]
fn name_found() {
	let alarms_vec = alarms_vec();
	let name = "Kill Shot".to_shared_string();

	let slint_program = common::slint_test();

	let alarm = Alarm::new(slint_program.as_weak());
	let alarm_rc = Rc::new(alarm);

	alarm_rc.set_alarm_list(alarms_vec.clone());
	assert!(alarm_rc.clone().has_name(name))
}

#[test]
fn name_not_found() {
	let alarms_vec = alarms_vec();
	let name = "Fail Fast".to_shared_string();

	let slint_program = common::slint_test();

	let alarm = Alarm::new(slint_program.as_weak());
	let alarm_rc = Rc::new(alarm);

	alarm_rc.set_alarm_list(alarms_vec.clone());
	assert!(!alarm_rc.clone().has_name(name))
}

#[test]
fn time_found() {
	let alarms_vec = alarms_vec();
	let time = (10, 10);

	let slint_program = common::slint_test();

	let alarm = Alarm::new(slint_program.as_weak());
	let alarm_rc = Rc::new(alarm);

	alarm_rc.set_alarm_list(alarms_vec.clone());
	assert!(alarm_rc.clone().has_time(time));
}

#[test]
fn time_not_found() {
	let alarms_vec = alarms_vec();
	let time = (23, 59);

	let slint_program = common::slint_test();

	let alarm = Alarm::new(slint_program.as_weak());
	let alarm_rc = Rc::new(alarm);

	alarm_rc.set_alarm_list(alarms_vec.clone());
	assert!(!alarm_rc.clone().has_time(time));
}


fn alarms_vec() -> Vec<AlarmData> {
	vec![
		AlarmData::new(
			"Test Mode", (0, 0),
			[true, true, true, true, true, false, false],
			true,
		),
		AlarmData::new(
			"Honing Missile", (10, 0),
			[true, true, true, true, true, true, true],
			true,
		),
		AlarmData::new(
			"Kill Shot", (3, 30),
			[false, false, false, false, false, false, true],
			true,
		),
		AlarmData::new(
			"Broken Alarm", (21, 0),
			[true, true, false, false, false, false, false],
			false,
		),
		AlarmData::new(
			"FIVEMAN", (5, 5),
			[false, false, false, false, true, false, false],
			true,
		),
	]
}