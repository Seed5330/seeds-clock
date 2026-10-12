pub mod unit_tests;
pub mod callbacks;
pub mod values;
pub mod clock_struct;
pub mod clock_data;
pub mod clock_bytes;

use clock_bytes::ClockBytes;

use slint_program::{ClockData, ClockGlobals, SlintProgram};
use clock_struct::{Clock};
use std::rc::{Rc};
use slint::{ComponentHandle, Model, Weak};

pub fn load(slint_weak: Weak<SlintProgram>) {
	let clock = Clock::new(slint_weak.clone());
	let clock_rc = Rc::new(clock);

	values::initialize_values(clock_rc.clone());
	callbacks::initialize_callbacks(clock_rc.clone());	
}

pub fn save_clock_list(slint_weak: Weak<SlintProgram>) {
	let slint_program = slint_weak.unwrap();
	let clock_list = slint_program
		.global::<ClockGlobals>()
		.get_clock_list()
		.iter()
		.collect::<Vec<ClockData>>();

	ClockBytes::save_clocks(clock_list);
}
