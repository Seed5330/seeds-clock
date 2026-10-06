#![cfg(test)]


use seeds_clock::{AlarmData, AlarmGlobals, alarm::alarm_struct::Alarm, common};
use slint::{ComponentHandle, Model, ModelRc};

#[test]
fn set_alarm_list() {
	let alarms_vec = alarms_vec();

	let slint_program = common::slint_test();
	let alarm_globals = slint_program.global::<AlarmGlobals>();

	let alarm = Alarm::new(slint_program.as_weak());
	alarm.set_alarm_list(alarms_vec.clone());

	let alarm_list = alarm_globals.get_alarm_list();
	assert_alarm_datas_collection(alarms_vec.clone(), alarm_list.clone());
}

fn assert_alarm_datas_collection(left: Vec<AlarmData>, right: ModelRc<AlarmData>) {
	for i in 0..left.len() {
		let left = left.get(i).unwrap().clone();
		let right = right.row_data(i).unwrap();

		assert_alarm_datas(left, right, i);
	}
}

fn assert_alarm_datas(left: AlarmData, right: AlarmData, index: usize) {
	let left = (
		left.name,
		left.hour,
		left.minute,
		left.schedule.iter().collect::<Vec<bool>>(),
		left.toggled,
	);

	let right = (
		right.name,
		right.hour,
		right.minute,
		right.schedule.iter().collect::<Vec<bool>>(),
		right.toggled,
	);

	pretty_assertions::assert_eq!(left, right, "Failed at index {}.", index);
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
	]
}











