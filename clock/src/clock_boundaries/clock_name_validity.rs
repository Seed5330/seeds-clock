use slint_program::SpClockNameValidity;

#[derive(PartialEq, Debug)]
pub enum ClockNameValidity {
	Valid,
	Empty,
	Exceeds16Characters,
	HasEmoji,
	Duplicate,
}

impl ClockNameValidity {
	pub fn from_sp_clock_name_validity(validity: SpClockNameValidity) -> ClockNameValidity {
		match validity {
			SpClockNameValidity::Valid => ClockNameValidity::Valid,
			SpClockNameValidity::Empty => ClockNameValidity::Empty,
			SpClockNameValidity::Exceeds16Characters => ClockNameValidity::Exceeds16Characters,
			SpClockNameValidity::HasEmoji => ClockNameValidity::HasEmoji,
			SpClockNameValidity::Duplicate => ClockNameValidity::Duplicate,
		}
	}
	pub fn to_sp_clock_name_validity(self) -> SpClockNameValidity {
		match self {
			ClockNameValidity::Valid => SpClockNameValidity::Valid,
			ClockNameValidity::Empty => SpClockNameValidity::Empty,
			ClockNameValidity::Exceeds16Characters => SpClockNameValidity::Exceeds16Characters,
			ClockNameValidity::HasEmoji => SpClockNameValidity::HasEmoji,
			ClockNameValidity::Duplicate => SpClockNameValidity::Duplicate,
		}
	}
} 