#![cfg(test)]
use rand::seq::SliceRandom;
use crate::time_struct::Time;
use crate::find_nearest_time::{find_nearest_time};



#[test]
fn find_nearby_time() {
	let time = Time::new(20, 40);
	let mut time_list = vec![
		Time::new(20, 20),
		Time::new(20, 50),
		Time::new(19, 30),
		Time::new(0, 5),

		Time::new(6, 30),
		Time::new(20, 25),
		Time::new(2, 25),
		Time::new(4, 20),
	];
	let expected = Time::new(20, 50);

	let mut rng = rand::rng();
	for i in 0..=100 {
		time_list.shuffle(&mut rng);
		let actual = find_nearest_time(&time_list, &time);
		assert_eq!(actual, expected, "Panicked at iteration {}.", i);
	}
}

#[test]
fn time_is_next_day() {
	let time = Time::new(23, 30);
	let mut time_list = vec![
		Time::new(23, 25),
		Time::new(22, 0),
		Time::new(19, 30),
		Time::new(14, 5),

		Time::new(5, 10),
		Time::new(9, 15),
		Time::new(3, 10),
		Time::new(4, 20),
	];
	let expected = Time::new(3, 10);

	let mut rng = rand::rng();
	for i in 0..=100 {
		time_list.shuffle(&mut rng);
		let actual = find_nearest_time(&time_list, &time);
		assert_eq!(actual, expected, "Panicked at iteration {}.", i);
	}	
}

#[test]
fn time_is_not_the_given_time() {
	let time = Time::new(4, 20);
	let mut time_list = vec![
		Time::new(23, 25),
		Time::new(22, 0),
		Time::new(13, 10),
		Time::new(0, 0),

		Time::new(4, 20),
		Time::new(11, 5),
		Time::new(7, 45),
		Time::new(5, 0),
	];
	let expected = Time::new(5, 0);

	let mut rng = rand::rng();
	for i in 0..=100 {
		time_list.shuffle(&mut rng);
		let actual = find_nearest_time(&time_list, &time);
		assert_eq!(actual, expected, "Panicked at iteration {}.", i);
	}	
}

#[test]
fn time_list_has_same_hour_as_given_time() {
	let time = Time::new(19, 30);
	let mut time_list = vec![
		Time::new(19, 0),
		Time::new(19, 15),
		Time::new(19, 20),
		Time::new(19, 30),

		Time::new(19, 35),
		Time::new(19, 45),
		Time::new(19, 50),
		Time::new(19, 55),
	];
	let expected = Time::new(19, 35);

	let mut rng = rand::rng();
	for i in 0..=100 {
		time_list.shuffle(&mut rng);
		let actual = find_nearest_time(&time_list, &time);
		assert_eq!(actual, expected, "Panicked at iteration {}.", i);
	}	
}
