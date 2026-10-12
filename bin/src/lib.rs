
// pub mod clock;
// pub mod alarm;

use slint_program::{self, ClockData, ClockGlobals, SlintProgram};

use slint::{self, CloseRequestResponse::HideWindow, ComponentHandle, Model};

use clock::clock_bytes::ClockBytes;


pub fn load() -> Result<(), slint::PlatformError> {
	let slint_program = SlintProgram::new()?;
	let window = slint_program.window();

	clock::load(slint_program.as_weak());
	alarm::load(slint_program.as_weak());

	let slint_program_weak = slint_program.as_weak();
	window.on_close_requested(move || {
		let slint_program = slint_program_weak.upgrade().expect("Slint program is verified beforehand.");
		let clock_globals = slint_program.global::<ClockGlobals>();

		let clocks_vec: Vec<ClockData> = clock_globals
			.get_clock_list()
			.iter()
			.collect();

		ClockBytes::save_clocks(clocks_vec);
		HideWindow
	});

	slint_program.run()?;

	Ok(())
}
