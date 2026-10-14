use slint::ToSharedString;
use slint_program::SpAlarmData;

use crate::{alarm_data::alarm_data_builder::AlarmDataBuilder, time_struct::Time};

pub struct AlarmData {
	name: String,
	time: Time,
	schedule: [bool; 7],
	toggled: bool,
}

impl AlarmData {
	pub fn from_builder(builder: AlarmDataBuilder) -> Self {
		AlarmData {
			name: builder.name.to_string(),
			time: builder.time,
			schedule: builder.schedule,
			toggled: builder.toggled,
		}
	}

	pub fn to_sp_alarm_data(self) -> SpAlarmData {
		SpAlarmData {
			name: self.name.to_shared_string(),
			time: self.time.to_sp_alarm_time(),
			schedule: self.schedule.into(),
			toggled: self.toggled,
		}
	}
}