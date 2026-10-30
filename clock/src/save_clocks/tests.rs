#![cfg(test)]

use rand::seq::SliceRandom;

use crate::{clock_data::ClockData, save_clocks::read_and_write::{read_clocks_from_file, write_clocks_to_file}};


#[test]
fn shuffle_test() {
	let mut clock_data_vec = clock_data_vec_1();

	let mut rng = rand::rng();
	for i in 0..100 {
		clock_data_vec.shuffle(&mut rng);

		write_clocks_to_file(clock_data_vec.clone()).unwrap();
		let retrived_clocks = read_clocks_from_file().unwrap();

		pretty_assertions::assert_eq!(clock_data_vec, retrived_clocks, "Panicked at iteration {}.", i);
	}
}


fn clock_data_vec_1() -> Vec<ClockData> {
	vec![
		ClockData::new("First", 2, 10),
		ClockData::new("^$#%()", 3, 30),
		ClockData::new("String", 4, 25),
		ClockData::new("YesSir", 5, 35),
		ClockData::new("-^%&*", -7, 55),

		ClockData::new("BLANK", 0, 0),
		ClockData::new("Sequel 1", -2, 30),
		ClockData::new("9088", -3, 45),
		ClockData::new("I Ate", 8, 30),
		ClockData::new("Charge", 10, 45),
	]
}