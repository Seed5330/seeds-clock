use anyhow::{Result, bail};

#[derive(Clone, PartialEq, Debug)]
pub struct Time {
	hour: i32,
	minute: i32,
}

impl Time {
	pub fn new(hour: i32, minute: i32) -> Self {
		Self {
			hour,
			minute,
		}
	}
}

pub fn find_next_alarm(time_list: &Vec<Time>, given_time: &Time) -> Time {
	let mut smallest_difference = 100;
	let mut selected_index = 0;

	for (index, iter_time) in time_list.iter().enumerate() {
		let hour_difference = calculate_hour_difference(given_time.hour, iter_time.hour);

		if hour_difference < smallest_difference {
			smallest_difference = hour_difference;
			selected_index = index;
		}
		else if hour_difference == smallest_difference {
			// The difference in hours is the same so now we have to compare minutes.

			let minute_difference = calculate_minute_difference(given_time.minute, iter_time.minute);
			if let Err(_) = minute_difference {
				continue;
			}
			else if let Ok(difference) = minute_difference {
				if difference < time_list[selected_index].minute {
					smallest_difference = hour_difference;
					selected_index = index;
					println!("New selected index is {}.", index);
				}
			}
		}
	}

	println!("Final index is {}.", selected_index);
	time_list[selected_index].clone()
}

pub fn calculate_hour_difference(current_hour: i32, target_hour: i32) -> i32 {
	let mut modified_hour = current_hour;
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

pub fn calculate_minute_difference(current_minute: i32, target_minute: i32) -> Result<i32> {
	if current_minute > target_minute {
		bail!("Given minute is larger than target minute.");
	}

	Ok(target_minute - current_minute)
}