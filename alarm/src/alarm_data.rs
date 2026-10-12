use slint::{ToSharedString};

use slint_program::AlarmData;

pub trait AlarmDataExtension {
	fn new(name: &str, time: (usize, usize), schedule: [bool; 7], toggled: bool) -> AlarmData {
		AlarmData {
			name: name.to_shared_string(),
			hour: time.0 as i32,
			minute: time.0 as i32,
			schedule: schedule.into(),
			toggled: toggled,
		}
	}
}


impl AlarmDataExtension for AlarmData {}