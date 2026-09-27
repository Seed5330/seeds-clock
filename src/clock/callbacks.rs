use crate::{ClockData, ClockGlobals, clock::clock_struct::Clock};
use std::rc::Rc;
use slint::{ComponentHandle};

pub fn initialize_callbacks(clock: Rc<Clock>) {
	let slint_program = clock.get_weak().unwrap();
	let clock_globals = slint_program.global::<ClockGlobals>();


	clock_globals.on_backend_clock_data_new(move |
		name, hours_offset, minutes_offset| -> ClockData {
		ClockData::new(name.as_str(), hours_offset, minutes_offset)
	});

	clock_globals.on_exceeds_character_limit(move |typed_name| {
		typed_name.len() > 12
	});

	
	callbacks_clock_settings(clock.clone());
}

pub fn callbacks_clock_settings(clock: Rc<Clock>) {
	let slint_program = clock.get_weak().unwrap();
	let clock_globals = slint_program.global::<ClockGlobals>();


	let clock_clone = clock.clone();
	clock_globals.on_add_clock(move |clock_data| {
		clock_clone.add_clock(clock_data);
	});


	let clock_clone = clock.clone();
	clock_globals.on_remove_clock(move |index| {
		clock_clone.remove_clock(index as usize);
	});

	let clock_clone = clock.clone();
	clock_globals.on_modify_clock(move |index, clock_data| {
		clock_clone.modify_clock(index as usize, clock_data);
	});

	let clock_clone = clock.clone();
	clock_globals.on_has_clock(move |clock_name| {
		clock_clone.has_clock(&clock_name)
	});
}



