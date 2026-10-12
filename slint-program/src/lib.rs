slint::include_modules!();

pub fn load() {
	println!("This is Slint Program.");
}

pub fn slint_test() -> SlintProgram {
	i_slint_backend_testing::init_no_event_loop();
	SlintProgram::new().expect("This SlintProgram has no event loop.")
}