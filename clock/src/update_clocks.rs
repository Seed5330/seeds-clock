use slint::{ComponentHandle, Weak};
use slint_program::{ClockGlobals, SlintProgram};

pub fn update_clocks(slint_program_weak: Weak<SlintProgram>) {
	slint_program_weak.upgrade_in_event_loop(move |slint_program| {
		let clock_globals = slint_program.global::<ClockGlobals>();
		
		let ticks_elapsed = clock_globals.get_ticks_elapsed();
		println!("Current ticks: {}", ticks_elapsed);
		clock_globals.set_ticks_elapsed(ticks_elapsed + 1);
	}).expect("ClockData is always valid.");
}