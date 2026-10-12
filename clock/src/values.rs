use std::{rc::Rc, time::Duration};

use helpers::slint_helpers;
use slint::{ComponentHandle, Model, SharedString, Weak};

use crate::{ClockData, ClockGlobals, SlintProgram, clock_bytes::ClockBytes, clock_data::ExtensionClock, clock_struct::Clock};

pub fn initialize_values(clock: Rc<Clock>) {
	let slint_program = clock.get_weak().unwrap();
	let clock_globals = slint_program.global::<ClockGlobals>();

	clock_globals.set_preview_mode(false);

	let hours_offset = slint_helpers::new_slint_vector(hours_offset_vec());
	clock_globals.set_hours_offset_scroll(hours_offset.clone().into());

	let minutes_offset = slint_helpers::new_slint_vector(minutes_offset_vec());
	clock_globals.set_minutes_offset_scroll(minutes_offset.clone().into());


	let clock_list = ClockBytes::get_clocks();

	if let Some(clock_list) = clock_list {
		clock.set_clock_list(clock_list);
	}


	let slint_weak = clock.get_weak();
	std::thread::spawn(move || {
		loop {
			let slint_weak = slint_weak.clone();
			clocks_update_loop(slint_weak);
			std::thread::sleep(Duration::from_millis(500));
		}
	});
}

pub fn hours_offset_vec() -> Vec<SharedString> {
	(-12..=14)
		.map(|x| slint::format!("{:+03}", x))
		.collect::<Vec<SharedString>>()
}

pub fn minutes_offset_vec() -> Vec<SharedString> {
	(0..=59)
		.map(|x| slint::format!("{:02}", x))
		.collect::<Vec<SharedString>>()
}

fn clocks_update_loop(slint_program_weak: Weak<SlintProgram>) {
	slint_program_weak.upgrade_in_event_loop(move |slint_program| {
		let clock_globals = slint_program.global::<ClockGlobals>();
		
		let current_local_clock = clock_globals.get_local_clock();
		let new_local_clock = ClockData::new(
			current_local_clock.name.as_str(),
			current_local_clock.hours_offset,
			current_local_clock.minutes_offset,
		);
		clock_globals.set_local_clock(new_local_clock);


		let current_clocks = clock_globals.get_clock_list();

		for (index, clock_data) in current_clocks.iter().enumerate() {
			let new_clock = ClockData::new(
				clock_data.name.as_str(),
				clock_data.hours_offset,
				clock_data.minutes_offset,
			);

			current_clocks.set_row_data(index, new_clock);
		}
	}).expect("Time data is always valid.");
}