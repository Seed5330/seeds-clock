use slint_program::{SlintProgram, ClockGlobals, ClockData};
use helpers::slint_helpers;
use std::rc::Rc;
use slint::{ComponentHandle, Model, ToSharedString, VecModel, Weak};


pub struct Clock {
	#[allow(dead_code)]
	slint_weak: Weak<SlintProgram>,
	clock_list: Rc<VecModel<ClockData>>,
}

impl Clock {
	pub fn new(slint_weak: Weak<SlintProgram>) -> Self {
		let slint_program = slint_weak.unwrap();
		let clock_globals = slint_program.global::<ClockGlobals>();

		let clocks_vec = clock_globals
			.get_clock_list()
			.iter()
			.collect::<Vec<ClockData>>();

		let clock_list = slint_helpers::new_slint_vector(clocks_vec);

		clock_globals.set_clock_list(clock_list.clone().into());
	
		Self {
			slint_weak,
			clock_list,
		}
	}


	pub fn get_clock_list(&self) -> Rc<VecModel<ClockData>> {
		self.clock_list.clone()
	}

	pub fn get_weak(&self) -> Weak<SlintProgram> {
		self.slint_weak.clone()
	}


	pub fn set_clock_list(&self, clocks_vec: Vec<ClockData>) {
		self.clock_list.set_vec(clocks_vec);
	}

	pub fn add_clock(&self, clock_data: ClockData) {
		let clock_list = self.clock_list.clone();
		clock_list.push(clock_data);
	}

	pub fn remove_clock(&self, index: usize) {
		let clock_list = self.clock_list.clone();
		clock_list.remove(index);
	}

	pub fn modify_clock(&self, index: usize, clock_data: ClockData) {
		let clock_list = self.clock_list.clone();
		clock_list.set_row_data(index, clock_data);
	}


	pub fn has_clock(&self, clock_name: &str) -> bool {
		let clock_list = self.clock_list.clone();

		for clock in clock_list.iter() {
			if clock.name == clock_name.to_shared_string() { return true; }
		}

		false
	}
}