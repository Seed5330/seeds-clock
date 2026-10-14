#![cfg(test)]


use alarm::{alarm_data::alarm_data_builder::AlarmDataBuilder, has_time::has_time, time_struct::Time};
use helpers::slint_helpers::VecToModel;
use slint::{ComponentHandle};
use slint_program::{SpAlarmData, AlarmGlobals};



#[test]
fn callback_has_time() {
	let slint_program = slint_program::slint_test();
	let alarm_globals = slint_program.global::<AlarmGlobals>();

	alarm::load(slint_program.as_weak());

	for i in 1..=10 {
		let time_vec = random_time_vector();
		
		let alarm_model = time_vec
			.iter()
			.map(|time|
				AlarmDataBuilder::new()
					.time(time.clone())
					.build()
					.to_sp_alarm_data()
			)
			.collect::<Vec<SpAlarmData>>()
			.to_model();
		alarm_globals.set_alarm_list(alarm_model);
		
		for j in 1..=10 {	
			let time = Time::random();
			let alarm_time = time.clone().to_sp_alarm_time();

			let expected = has_time(&time_vec, &time);
			let actual = alarm_globals.invoke_has_time(alarm_time);

			let current_iteration = (i * 10) + j;
			assert_eq!(expected, actual, "Panicked at iteration {}.", current_iteration);
		}
	}
}

fn random_time_vector() -> Vec<Time> {
	let vector_size = rand::random_range(0..=50);

	let mut time_vec = vec![];
	for _ in 0..vector_size {
		time_vec.push(Time::random());
	}

	time_vec
}



