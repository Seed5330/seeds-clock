use crate::time_struct::Time;

pub fn has_time(time_vec: &Vec<Time>, given_time: &Time) -> bool {
	if time_vec.is_empty() {
		return false;
	}

	for indexed_time in time_vec.iter() {
		if given_time == indexed_time {
			return true;
		}
	}
	
	false
}

mod tests {
	#![cfg(test)]
	use crate::time_struct::Time;
	use crate::has_time::has_time;

	#[test]
	fn time_found() {
		assert!(has_time(&time_vec(), &Time::new(1, 25)));
	}

	#[test]
	fn time_not_found() {
		assert!(!has_time(&time_vec(), &Time::new(0, 10)));
	}

	#[test]
	fn time_vec_is_empty() {
		assert!(!has_time(&vec![], &Time::new(4, 20)));
	}

	fn time_vec() -> Vec<Time> {
		vec![
			Time::new(0, 0),
			Time::new(5, 40),
			Time::new(10, 20),
			Time::new(0, 45),
			Time::new(3, 40),

			Time::new(23, 50),
			Time::new(23, 20),
			Time::new(13, 0),
			Time::new(12, 0),
			Time::new(1, 25),
		]
	}
}