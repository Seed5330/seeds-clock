pub mod alarm_struct;
pub mod alarm_data;

pub mod unit_tests;

pub mod values;
pub mod callbacks;

pub mod find_next_alarm;

use std::rc::Rc;

use slint::{Weak};

use crate::{SlintProgram, alarm::alarm_struct::Alarm};

pub fn load(slint_weak: Weak<SlintProgram>) {
	let alarm = Alarm::new(slint_weak.clone());
	let alarm_rc = Rc::new(alarm);
	
	values::initialize_values(alarm_rc.clone());
	callbacks::initialize_callbacks(alarm_rc.clone());
}