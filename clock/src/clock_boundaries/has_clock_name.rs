use crate::clock_data::ClockData;

pub fn has_clock_name(clock_data_vec: &Vec<ClockData>, clock_name: &str) -> bool {
	if clock_data_vec.is_empty() { return false; }

	for clock in clock_data_vec {
		if clock.name() == clock_name { return true; }
	}

	false
}

mod tests {
	#![cfg(test)]
	use crate::clock_data::ClockData;
	use crate::clock_boundaries::has_clock_name::has_clock_name;

	#[test]
	fn clock_name_found() {
		let actual = has_clock_name(&clock_data_vec(), "Power Punch");
		let expected = true;

		assert_eq!(actual, expected);
	}

	#[test]
	fn clock_name_not_found() {
		let actual = has_clock_name(&clock_data_vec(), "BURANK");
		let expected = false;

		assert_eq!(actual, expected);
	}

	#[test]
	fn clock_vec_is_empty() {
		let actual = has_clock_name(&vec![], "50 Cents");
		let expected = false;

		assert_eq!(actual, expected);
	}

	fn clock_data_vec() -> Vec<ClockData> {
		vec![
			ClockData::new("50 Cents", 5, 0),
			ClockData::new("Upside Rightside", 9, 40),
			ClockData::new("Quarter Point", -3, 55),
			ClockData::new("Midnight Snack", 1, 45),
			ClockData::new("Name Not", 3, 15),

			ClockData::new("Power Punch", -6, 0),
			ClockData::new("Beat 25", 3, 25),
			ClockData::new("Stevener", 7, 30),
			ClockData::new("Negative Edge", -12, 10),
			ClockData::new("I Ate Sum", -8, 5),
		]
	}
}