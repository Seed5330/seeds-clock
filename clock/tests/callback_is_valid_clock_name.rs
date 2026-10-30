#![cfg(test)]

use clock::clock_boundaries::is_valid_clock_name::is_valid_clock_name;
use slint::{ComponentHandle, ToSharedString};
use slint_program::ClockGlobals;

#[test]
fn callback_is_valid_clock_name() {
	let slint_program = slint_program::slint_test();
	let clock_globals = slint_program.global::<ClockGlobals>();

	clock::load(slint_program.as_weak());

	let string_vec = vec![
		"",
		"    ",
		"This string consists of a full sentence.",
		"Excuse me but I would like to ask a question.",
		"🫀",
		"🌘🌑🌒",
		"👉 Right.",
		"Left. 👈",
		"Mid 🙌 Del",
		"Valid Name",
		"Sempel",
		"00112233"
	];

	for string in string_vec {
		let actual = clock_globals.invoke_is_valid_clock_name(string.to_shared_string());
		let expected = is_valid_clock_name(string).to_sp_clock_name_validity();

		assert_eq!(actual, expected, "Panicked at this string: {}", string);
	}
}