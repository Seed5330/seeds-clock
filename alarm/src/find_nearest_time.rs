use crate::time_struct::Time;

pub fn find_nearest_time(time_list: &Vec<Time>, given_time: &Time) -> Time {
	let mut smallest_difference = 1000000;
	let mut indexed = 0;

	for (index, target_time) in time_list.iter().enumerate() {
		let difference = {
			let value = time_difference_in_minutes(&given_time, &target_time);

			if value == 0 {
				24 * 60
			}
			else {
				value
			}
		};

		if difference < smallest_difference {
			smallest_difference = difference;
			indexed = index;
		}
	}

	time_list[indexed].clone()
}

pub fn time_difference_in_minutes(given_time: &Time, target_time: &Time) -> i32 {
	let hourly_difference = calculate_hour_difference(
		given_time.hour() as i32, 
		target_time.hour() as i32,
	);
	let minutes_comparison = calculate_minute_difference(
		given_time.minute() as i32, 
		target_time.minute() as i32,
	);

	if hourly_difference == 0 && minutes_comparison.is_negative() {
		return (24 * 60) + minutes_comparison;
	}

	(hourly_difference * 60) + minutes_comparison
}

pub fn calculate_hour_difference(given_hour: i32, target_hour: i32) -> i32 {
	let mut modified_hour = given_hour;
	let mut difference = 0;

	loop {
		if modified_hour == target_hour {
			break;
		}

		difference += 1;
		if modified_hour == 23 {
			modified_hour = 0
		}
		else {
			modified_hour += 1;
		}
	}

	difference
}

pub fn calculate_minute_difference(given_minute: i32, target_minute: i32) -> i32 {
	target_minute - given_minute
}