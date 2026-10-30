#![cfg(test)]


use slint::{ComponentHandle};
use slint_program::SlintProgram;


#[test]
#[ignore = "Load executable with front end. Start at Clock tab."]
fn clock_open() {
	let slint_program = SlintProgram::new().unwrap();

	slint_program.set_active_tab(0);
	clock::load(slint_program.as_weak());


	// slint_program.window().on_close_requested(move || {
	// 	clock::save_clock_list(slint_weak.clone());
	// 	HideWindow
	// });

	slint_program.run().unwrap();
}

// #[test]
// fn save_clocks_to_file() {
// 	let clocks_vec = vec![
// 		ClockData::new("Save Mode", 8, 0),
// 		ClockData::new("Two Say", 7, 0),
// 		ClockData::new("Half Way", -6, 0),
// 		ClockData::new("Me Too", 2, 0),

// 		ClockData::new("Quarter", 3, 0),
// 		ClockData::new("Negative X", -14, 0),
// 		ClockData::new("Positive X", 12, 0),
// 		ClockData::new("Byte Eight", -8, 0),
// 	];


// 	let slint_program = slint_program::slint_test();
// 	let slint_weak = slint_program.as_weak();

// 	let clock = Rc::new(Clock::new(slint_weak.clone()));

// 	clock.set_clock_list(clocks_vec.clone());
// 	clock::save_clock_list(slint_weak.clone());

// 	let clock_list = ClockBytes::get_clocks().unwrap();


// 	let left = clocks_vec
// 		.iter()
// 		.map(|cd| (cd.name.clone(), cd.hours_offset, cd.minutes_offset))
// 		.collect::<Vec<(SharedString, i32, i32)>>();

// 	let right = clock_list
// 		.iter()
// 		.map(|cd| (cd.name.clone(), cd.hours_offset, cd.minutes_offset))
// 		.collect::<Vec<(SharedString, i32, i32)>>();

// 	pretty_assertions::assert_eq!(left, right);
// }
