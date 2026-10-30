pub mod unit_tests;
pub mod update_clocks;
pub mod clock_bytes;

pub mod offsets;
pub mod clock_boundaries;

pub mod clock_data;
pub mod save_clocks;

use std::time::Duration;

use helpers::slint_helpers::VecToModel;
use slint_program::{ClockGlobals, SlintProgram};
use slint::{ComponentHandle, Model, SharedString, ToSharedString, Weak};

use crate::{clock_boundaries::{has_clock_name, is_valid_clock_name}, clock_data::ClockData, offsets::{utc_now, utc_offset_display}};

pub fn load(slint_weak: Weak<SlintProgram>) {
	let slint_program = slint_weak.unwrap();
	let clock_globals = slint_program.global::<ClockGlobals>();

	clock_globals.set_preview_mode(false);
	
	let hours_offset_scroll = (-12..=14)
		.map(|x| slint::format!("{:+03}", x))
		.collect::<Vec<SharedString>>()
		.to_model();
	clock_globals.set_hours_offset_scroll(hours_offset_scroll);

	let minutes_offset_scroll = (0..=59)
		.map(|x| slint::format!("{:02}", x))
		.collect::<Vec<SharedString>>()
		.to_model();
	clock_globals.set_minutes_offset_scroll(minutes_offset_scroll);


	clock_globals.on_backend_utc_offset_display(move |hours_offset, minutes_offset| {
		utc_offset_display::utc_offset_display(hours_offset, minutes_offset)
			.to_shared_string()
	});

	clock_globals.on_backend_utc_now(move |hours_offset, minutes_offset| {
		utc_now::utc_now(hours_offset, minutes_offset)
			.to_shared_string()
	});


	let slint_weak = slint_program.as_weak();
	clock_globals.on_has_clock_name(move |clock_name| {
		let slint_program = slint_weak.unwrap();
		let clock_globals = slint_program.global::<ClockGlobals>();

		let clock_data_vec = clock_globals
			.get_clock_list()
			.iter()
			.map(|sp_clock_data| ClockData::from_sp_clock_data(sp_clock_data))
			.collect::<Vec<ClockData>>();

		has_clock_name::has_clock_name(&clock_data_vec, clock_name.as_str())
	});

	clock_globals.on_is_valid_clock_name(move |clock_name| {
		is_valid_clock_name::is_valid_clock_name(&clock_name)
			.to_sp_clock_name_validity()
	});

	let slint_weak = slint_program.as_weak();
	std::thread::spawn(move || {
		let slint_weak = slint_weak.clone();
		loop {
			update_clocks::update_clocks(slint_weak.clone());
			std::thread::sleep(Duration::from_millis(500));
		}
	});
}

// pub fn save_clock_list(slint_weak: Weak<SlintProgram>) {
// 	let slint_program = slint_weak.unwrap();
// 	let clock_list = slint_program
// 		.global::<ClockGlobals>()
// 		.get_clock_list()
// 		.iter()
// 		.collect::<Vec<ClockData>>();

// 	ClockBytes::save_clocks(clock_list);
// }
