use helpers::slint_helpers;
use slint::{ComponentHandle, Model, Weak};
use slint_program::{SpAlarmData, AlarmGlobals, SlintProgram};



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
			.collect::<Vec<SpAlarmData>>();

		let alarm_list = slint_helpers::new_slint_vector(alarms_vec.clone());
		alarm_globals.set_alarm_list(alarm_list.clone().into());
		
		Self {
			slint_weak,
		}
	}

	pub fn get_weak(&self) -> Weak<SlintProgram> {
		self.slint_weak.clone()
	}
}