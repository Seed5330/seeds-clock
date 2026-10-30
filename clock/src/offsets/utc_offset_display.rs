pub fn utc_offset_display(hours_offset: i32, minutes_offset: i32) -> String {
	format!("{:+03}:{:02}", hours_offset, minutes_offset)
}

mod tests {
	#![cfg(test)]
	use crate::offsets::utc_offset_display::utc_offset_display;

	#[test]
	fn positive_offset() {
		let actual = utc_offset_display(12, 30);
		let expected = "+12:30";

		assert_eq!(actual, expected);
	}

	#[test]
	fn negative_offset() {
		let actual = utc_offset_display(-10, 45);
		let expected = "-10:45";

		assert_eq!(actual, expected);
	}

	#[test]
	fn zero_hour_offset_is_displayed_as_positive() {
		let actual = utc_offset_display(0, 15);
		let expected = "+00:15";

		assert_eq!(actual, expected);
	}

	#[test]
	fn hour_value_is_single_digit() {
		let actual = utc_offset_display(5, 20);
		let expected = "+05:20";

		assert_eq!(actual, expected);		
	}

	#[test]
	fn minute_value_is_single_digit() {
		let actual = utc_offset_display(10, 0);
		let expected = "+10:00";

		assert_eq!(actual, expected);		
	}

	#[test]
	fn both_values_are_single_digit() {
		let actual = utc_offset_display(-8, 0);
		let expected = "-08:00";

		assert_eq!(actual, expected);		
	}		
}