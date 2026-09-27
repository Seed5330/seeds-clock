#![cfg(test)]
use seeds_common::slint_common;
use slint::{ComponentHandle};
use crate::{ClockData, ClockGlobals, common};
use crate::clock::{clock_struct::Clock, unit_tests::asserts};

#[test]
fn set_clock_list() {
	let new_clocks = vec![
		ClockData::new("Gekiranger", 1, 31),
		ClockData::new("Go Onger", 2, 32),
		ClockData::new("Shinkenger", 3, 33),
		ClockData::new("Goseiger", 4, 34),
		ClockData::new("Gokaiger", 5, 35),
	];


	let slint_program = common::slint_test();
	let clock_globals = slint_program.global::<ClockGlobals>();
	

	let clock = Clock::new(slint_program.as_weak());
	clock.set_clock_list(new_clocks.clone());


	let clocks_model = clock_globals.get_clock_list();
	asserts::assert_vec_equals_model(new_clocks, clocks_model);
}


#[test]
fn add_clock() {
	let new_clock = ClockData::new("The Finisher", 23, 30);


	let slint_program = common::slint_test();
	let clock_globals = slint_program.global::<ClockGlobals>();
	
	let mut clocks_vec = slint_common::model_to_vec(clock_globals.get_clock_list());
	

	let clock = Clock::new(slint_program.as_weak());
	clock.add_clock(new_clock.clone());
	clocks_vec.push(new_clock.clone());


	let clocks_model = clock_globals.get_clock_list();
	asserts::assert_vec_equals_model(clocks_vec, clocks_model);
}

#[test]
fn modify_clock() {
	let new_clock = ClockData::new("Middle Night", 20, 0);


	let slint_program = common::slint_test();
	let clock_globals = slint_program.global::<ClockGlobals>();
	
	let mut clocks_vec = slint_common::model_to_vec(clock_globals.get_clock_list());
	

	let clock = Clock::new(slint_program.as_weak());
	clock.modify_clock(4, new_clock.clone());
	clocks_vec[4] = new_clock.clone();


	let clocks_model = clock_globals.get_clock_list();
	asserts::assert_vec_equals_model(clocks_vec, clocks_model);
}

#[test]
fn remove_clock() {
	let slint_program = common::slint_test();
	let clock_globals = slint_program.global::<ClockGlobals>();
	
	let mut clocks_vec = slint_common::model_to_vec(clock_globals.get_clock_list());
	

	let clock = Clock::new(slint_program.as_weak());
	clock.remove_clock(3);
	clocks_vec.remove(3);


	let clocks_model = clock_globals.get_clock_list();
	asserts::assert_vec_equals_model(clocks_vec, clocks_model);
}

#[test]
fn clock_found() {
	let clock_list = vec![
		ClockData::new("Go Busters", 0, 0),
		ClockData::new("Kyoryuger", 0, 0),
		ClockData::new("ToQger", 0, 0),
		ClockData::new("Ninninger", 0, 0),
		ClockData::new("Zyuohger", 0, 0),

		ClockData::new("Kyuranger", 0, 0),
		ClockData::new("LuPat", 0, 0),
		ClockData::new("Ryusoulger", 0, 0),
		ClockData::new("Kiramager", 0, 0),
		ClockData::new("Zenkaiger", 0, 0),
	];


	let slint_program = common::slint_test();


	let clock = Clock::new(slint_program.as_weak());
	clock.get_clock_list().set_vec(clock_list.clone());
	
	assert!(clock.has_clock("LuPat"))
}

#[test]
fn clock_not_found() {
	let clock_list = vec![
		ClockData::new("Go Busters", 0, 0),
		ClockData::new("Kyoryuger", 0, 0),
		ClockData::new("ToQger", 0, 0),
		ClockData::new("Ninninger", 0, 0),
		ClockData::new("Zyuohger", 0, 0),

		ClockData::new("Kyuranger", 0, 0),
		ClockData::new("LuPat", 0, 0),
		ClockData::new("Ryusoulger", 0, 0),
		ClockData::new("Kiramager", 0, 0),
		ClockData::new("Zenkaiger", 0, 0),
	];

	
	let slint_program = common::slint_test();


	let clock = Clock::new(slint_program.as_weak());
	clock.get_clock_list().set_vec(clock_list.clone());
	
	assert!(!clock.has_clock("King Ohger"))
}