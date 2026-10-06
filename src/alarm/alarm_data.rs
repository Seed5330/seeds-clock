use slint::{ToSharedString};

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
}