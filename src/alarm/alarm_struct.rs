use std::rc::Rc;
use seeds_common::slint_common;
use slint::{ComponentHandle, Model, SharedString, VecModel, Weak};
use crate::{AlarmData, AlarmGlobals, SlintProgram};



pub struct Alarm {
	slint_weak: Weak<SlintProgram>,
	alarm_list: Rc<VecModel<AlarmData>>
}

impl Alarm {
	pub fn new(slint_weak: Weak<SlintProgram>) -> Self {
		let slint_program = slint_weak.unwrap();
		let alarm_globals = slint_program.global::<AlarmGlobals>();

		let alarms_vec = alarm_globals
			.get_alarm_list()
			.iter()
			.collect::<Vec<AlarmData>>();

		let alarm_list = slint_common::new_slint_vector(alarms_vec.clone());
		alarm_globals.set_alarm_list(alarm_list.clone().into());
		
		Self {
			slint_weak,
			alarm_list: alarm_list.clone(),
		}
	}

	pub fn get_weak(&self) -> Weak<SlintProgram> {
		self.slint_weak.clone()
	}

	pub fn set_alarm_list(&self, alarms_vec: Vec<AlarmData>) {
		self.alarm_list.set_vec(alarms_vec);
	}

	pub fn add_alarm(&self, alarm_data: AlarmData) {
		self.alarm_list.push(alarm_data);
	}

	pub fn remove_alarm(&self, index: usize) {
		self.alarm_list.remove(index);
	}

	pub fn modify_alarm(&self, index: usize, alarm_data: AlarmData) {
		self.alarm_list.set_row_data(index, alarm_data);
	}

	pub fn has_name(&self, name: SharedString) -> bool {
		for alarm in self.alarm_list.iter() {
			if name == alarm.name {
				return true;
			}
		}

		false
	}

	pub fn has_time(&self, time: (i32, i32)) -> bool {
		for alarm in self.alarm_list.iter() {
			if time == (alarm.hour, alarm.minute) {
				return true;
			}
		}
		
		false
	}
}