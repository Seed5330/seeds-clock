use slint::{SharedString, ToSharedString};

use crate::AlarmData;

impl AlarmData {
	pub fn new(name: &str, time: (usize, usize), schedule: [bool; 7], toggled: bool) -> Self {
		Self {
			name: name.to_shared_string(),
			hour: time.0 as i32,
			minute: time.0 as i32,
			schedule: schedule.into(),
			toggled: toggled,
		}
	}

	pub fn get_time_display(hour: usize, minute: usize) -> SharedString {
		slint::format!("{:02}:{:02}", hour, minute)
	}
}