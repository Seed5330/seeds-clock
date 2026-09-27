use slint::{ComponentHandle};

use crate::SlintProgram;

pub fn slint_test() -> SlintProgram {
	i_slint_backend_testing::init_no_event_loop();
	SlintProgram::new().expect("This SlintProgram has no event loop.")
}

pub fn load_tab(tab_index: usize) -> SlintProgram {
	let slint_program = SlintProgram::new()
		.expect("Slint Program file is always there.");

	let mut tabs_visible = [false, false, false, false];
	tabs_visible[tab_index] = true;
	println!("Loaded a tab.");

	slint_program.set_enabled_tabs(tabs_visible.into());
	slint_program.clone_strong()
}