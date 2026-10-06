use seeds_common::slint_common;
use slint::{ComponentHandle, Model, SharedString, Weak};
use crate::{AlarmData, AlarmGlobals, SlintProgram};



pub struct Alarm {
	slint_weak: Weak<SlintProgram>,
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
		}
	}

	pub fn get_weak(&self) -> Weak<SlintProgram> {
		self.slint_weak.clone()
	}

	pub fn set_alarm_list(&self, alarms_vec: Vec<AlarmData>) {
		let slint_program = self.slint_weak.unwrap();
		let alarm_globals = slint_program.global::<AlarmGlobals>();

		alarm_globals.set_alarm_list(slint_common::new_slint_vector(alarms_vec).into());
	}

	pub fn has_name(&self, name: SharedString) -> bool {
		let slint_program = self.slint_weak.unwrap();
		let alarm_globals = slint_program.global::<AlarmGlobals>();

		let alarm_list = alarm_globals.get_alarm_list();
		for alarm in alarm_list.iter() {
			if name == alarm.name {
				return true;
			}
		}

		false
	}

	pub fn has_time(&self, time: (i32, i32)) -> bool {
		let slint_program = self.slint_weak.unwrap();
		let alarm_globals = slint_program.global::<AlarmGlobals>();

		let alarm_list = alarm_globals.get_alarm_list();

		for alarm in alarm_list.iter() {
			if time == (alarm.hour, alarm.minute) {
				return true;
			}
		}
		
		false
	}
}