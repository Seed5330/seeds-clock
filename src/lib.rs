pub mod common;
pub mod audio_handler;
pub mod clock;
pub mod alarm;


slint::include_modules!();
use slint::{CloseRequestResponse::HideWindow, Model};

use clock::clock_bytes::ClockBytes;


pub fn load() {
	let slint_program = SlintProgram::new().expect("Program failed to initialize!");
	let window = slint_program.window();

	clock::load(slint_program.as_weak());
	alarm::load(slint_program.as_weak());

	let slint_program_weak = slint_program.as_weak();
	window.on_close_requested(move || {
		let slint_program = slint_program_weak.unwrap();
		let clock_globals = slint_program.global::<ClockGlobals>();

		let clocks_vec: Vec<ClockData> = clock_globals
			.get_clock_list()
			.iter()
			.collect();

		ClockBytes::save_clocks(clocks_vec);
		HideWindow
	});

	#[cfg(not(test))]
	slint_program.run().expect("Program failed to start!");
}
