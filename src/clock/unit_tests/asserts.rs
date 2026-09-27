#![cfg(test)]
use seeds_common::slint_common;
use slint::{Model, ModelRc};

use crate::{ClockData};


#[test]
fn success_assert_vec_equals_model() {
	let clocks_vec = vec![
		ClockData::new("One", 1, 1),
		ClockData::new("Two", 2, 2),
		ClockData::new("Three", 3, 3),
		ClockData::new("Four", 4, 4),
		ClockData::new("Five", 5, 5),
	];

	let clocks_rc = slint_common::new_slint_vector(clocks_vec.clone());
	let clocks_model = ModelRc::from(clocks_rc.clone());

	assert_vec_equals_model(clocks_vec, clocks_model);
}


#[test]
#[should_panic]
fn fail_assert_vec_equals_model() {
	let mut clocks_vec = vec![
		ClockData::new("One", 1, 1),
		ClockData::new("Two", 2, 2),
		ClockData::new("Three", 3, 3),
		ClockData::new("Four", 4, 4),
		ClockData::new("Five", 5, 5),
	];

	let clocks_rc = slint_common::new_slint_vector(clocks_vec.clone());
	let clocks_model = ModelRc::from(clocks_rc.clone());

	clocks_vec.remove(2);
	assert_vec_equals_model(clocks_vec, clocks_model);
}

pub fn assert_vec_equals_model<T>(vec: Vec<T>, model: ModelRc<T>)
where
	T: PartialEq + std::fmt::Debug,
{
	let model_vec = model
		.iter()
		.collect::<Vec<T>>();

	pretty_assertions::assert_eq!(vec, model_vec);
}