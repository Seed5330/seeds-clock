use slint::ToSharedString;
use slint_program::SpClockData;
use speedy::{Readable, Writable};

#[derive(Clone, Readable, Writable, Debug, PartialEq)]
pub struct ClockData {
	name: String,
	hours_offset: i32,
	minutes_offset: i32,
}

impl ClockData {
	pub fn new(name: &str, hours_offset: i32, minutes_offset: i32) -> Self {
		Self {
			name: name.to_string(),
			hours_offset,
			minutes_offset,
		}
	}

	pub fn to_sp_clock_data(self) -> SpClockData {
		SpClockData {
			name: self.name.to_shared_string(),
			hours_offset: self.hours_offset,
			minutes_offset: self.minutes_offset,
		}
	}

	pub fn from_sp_clock_data(sp_clock_data: SpClockData) -> Self {
		Self {
			name: sp_clock_data.name.to_string(),
			hours_offset: sp_clock_data.hours_offset,
			minutes_offset: sp_clock_data.minutes_offset,
		}
	}

	pub fn name(&self) -> &String {
		&self.name
	}
}