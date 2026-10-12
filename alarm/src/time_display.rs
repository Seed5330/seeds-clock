use anyhow::{Result, bail};

pub fn time_display(hour: usize, minute: usize) -> Result<String> {
	if hour > 23 {
		bail!("Hour value cannot be greater than 23.");
	}
	else if minute > 59 {
		bail!("Minute value cannot be greater than 59.");
	}
	
	Ok(format!("{:02}:{:02}", hour, minute))
}

mod tests {
	#![cfg(test)]
	use crate::time_display::time_display;

	#[test]
	fn both_time_values_are_2_digits() {
		let actual = time_display(19, 30).unwrap();
		let expect = "19:30";
		assert_eq!(actual, expect);
	}

	#[test]
	fn hour_value_is_one_digit() {
		let actual = time_display(2, 45).unwrap();
		let expect = "02:45";
		assert_eq!(actual, expect);
	}

	#[test]
	fn minute_value_is_one_digit() {
		let actual = time_display(23, 0).unwrap();
		let expect = "23:00";
		assert_eq!(actual, expect);
	}

	#[test]
	fn both_times_values_are_1_digit() {
		let actual = time_display(3, 5).unwrap();
		let expect = "03:05";
		assert_eq!(actual, expect);
	}

	#[test]
	fn hour_value_exceeds_23() {
		let value = time_display(25, 50);
		assert!(value.is_err());
	}

	#[test]
	fn minute_value_exceeds_59() {
		let value = time_display(9, 70);
		assert!(value.is_err());
	}
}
