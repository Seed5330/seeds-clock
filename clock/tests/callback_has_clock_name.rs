#![cfg(test)]

use clock::{clock_boundaries::has_clock_name::has_clock_name, clock_data::ClockData};
use helpers::slint_helpers::VecToModel;
use rand::seq::{IndexedRandom, SliceRandom};
use slint::{ComponentHandle, ToSharedString};
use slint_program::{ClockGlobals, SpClockData};




#[test]
fn callback_has_clock_name() {
	let slint_program = slint_program::slint_test();
	let clock_globals = slint_program.global::<ClockGlobals>();

	clock::load(slint_program.as_weak());

	let mut clock_data_vec = clock_data_vec();
	let string_pool = string_pool();

	let mut rng = rand::rng();
	for i in 0..100 {
		clock_data_vec.shuffle(&mut rng);
		
		let clock_data_model = clock_data_vec
			.clone()
			.iter()
			.map(|clock_data| clock_data.to_owned().to_sp_clock_data())
			.collect::<Vec<SpClockData>>()
			.to_model();
		clock_globals.set_clock_list(clock_data_model);

		let chosen_name = string_pool.choose(&mut rng).unwrap();
		let actual = clock_globals.invoke_has_clock_name(chosen_name.to_shared_string());
		let expected = has_clock_name(&clock_data_vec, chosen_name.as_str());

		assert_eq!(actual, expected, "Panicked at iteration {}.", i);
	}	
}


fn clock_data_vec() -> Vec<ClockData> {
	vec![
		ClockData::new("Negative Edge", -12, 15),
		ClockData::new("JAKAA", -10, 20),
		ClockData::new("The 1930s", -9, 30),
		ClockData::new("Clock Data", -7, 45),
		ClockData::new("Cut Away", -6, 40),

		ClockData::new("Sequel 2", -2, 30),
		ClockData::new("Nail Clip", -1, 45),
		ClockData::new("", 0, 0),
		ClockData::new("Jeal Teal", 1, 10),
		ClockData::new("CHARAMPO", 3, 55),

		ClockData::new("FiveMan", 5, 5),
		ClockData::new("DAI BAKUHATSU", 6, 50),
		ClockData::new("Loose Cannon", 9, 45),
		ClockData::new("1n1 Ratio", 11, 35),
		ClockData::new("Goal", 13, 25),
	]
}

fn string_pool() -> Vec<String> {
	vec![
		"",
		"Clock Data",
		"JAKAA",
		"1n1 Ratio",
		"The 1930s",

		"GOJAA",
		"Wireless",
		"Beet Mug",
		"Valve 3",
		"404 Not Found",
	]
		.iter()
		.map(|str| str.to_string())
		.collect()
}