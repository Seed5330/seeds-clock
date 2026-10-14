use slint_program::{SpAlarmData, SpAlarmTime};

use crate::time_display;

#[derive(Debug, PartialEq, Clone)]
pub struct Time {
	hour: usize,
	minute: usize,
}

impl Time {
	pub fn new(hour: usize, minute: usize) -> Self {
		Self {
			hour,
			minute,
		}
	}

	pub fn hour(&self) -> usize {
		self.hour
	}

	pub fn minute(&self) -> usize {
		self.minute
	}

	pub fn time_display(&self) -> String {
		time_display::time_display(self.hour(), self.minute()).expect("Forced time boundaries.")
	}

	pub fn from_alarm_time(alarm_time: SpAlarmTime) -> Self {
		Self {
			hour: alarm_time.hour as usize,
			minute: alarm_time.minute as usize,
		}
	}

	pub fn to_sp_alarm_time(self) -> SpAlarmTime {
		SpAlarmTime {
			hour: self.hour as i32,
			minute: self.minute as i32,
		}
	}

	pub fn from_alarm_data(alarm_data: &SpAlarmData) -> Self {
		Self {
			hour: alarm_data.time.hour as usize,
			minute: alarm_data.time.minute as usize,
		}
	}

	pub fn random() -> Self {
		Self {
			hour: rand::random_range(0..=23),
			minute: rand::random_range(0..=59),
		}
	}
}