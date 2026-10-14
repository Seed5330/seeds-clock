use crate::{alarm_data::alarm_data::AlarmData, time_struct::Time};

pub struct AlarmDataBuilder {
	pub(crate) name: String,
	pub(crate) time: Time,
	pub(crate) schedule: [bool; 7],
	pub(crate) toggled: bool,
}

impl AlarmDataBuilder {
	pub fn new() -> Self {
		Self {
			name: "Alarm Data".to_string(),
			time: Time::new(0, 0),
			schedule: [false; 7],
			toggled: true,
		}
	}

	pub fn name(self, name: &str) -> Self {
		Self {
			name: name.to_string(),
			..self
		}
	}

	pub fn time(self, time: Time) -> Self {
		Self {
			time,
			..self
		}
	}

	pub fn schedule(self, schedule: [bool; 7]) -> Self {
		Self {
			schedule,
			..self
		}
	}

	pub fn toggled(self, toggled: bool) -> Self {
		Self {
			toggled,
			..self
		}
	}

	pub fn build(self) -> AlarmData {
		AlarmData::from_builder(self)
	}
}


