use crate::clock_boundaries::clock_name_validity::ClockNameValidity;



pub fn is_valid_clock_name(clock_name: &str) -> ClockNameValidity {
	if clock_name.is_empty() {
		return ClockNameValidity::Empty;
	}

	if clock_name.len() > 16 {
		return ClockNameValidity::Exceeds16Characters;
	}

	if clock_name.chars().all(|c| c.is_whitespace()) {
		return ClockNameValidity::Empty;
	}

	if clock_name.chars().any(|c| emojis::get(&c.to_string()).is_some()) {
		return ClockNameValidity::HasEmoji;
	}	

	ClockNameValidity::Valid
}

mod tests {
	#![cfg(test)]
	use crate::clock_boundaries::is_valid_clock_name::is_valid_clock_name;
	use crate::clock_boundaries::clock_name_validity::ClockNameValidity;

	#[test]
	fn string_is_empty() {
		let string = "";

		let actual = is_valid_clock_name(string);
		let expected = ClockNameValidity::Empty;

		assert_eq!(actual, expected);
	}

	#[test]
	fn string_is_all_whitespaces() {
		let string = "          ";

		let actual = is_valid_clock_name(string);
		let expected = ClockNameValidity::Empty;

		assert_eq!(actual, expected);
	}

	#[test]
	fn string_is_20_characters() {
		let string = "TypedFivesChainMails";

		let actual = is_valid_clock_name(string);
		let expected = ClockNameValidity::Exceeds16Characters;

		assert_eq!(actual, expected);
	}

	#[test]
	fn string_is_24_characters() {
		let string = "Chou Ninpou! Kage No Mai";

		let actual = is_valid_clock_name(string);
		let expected = ClockNameValidity::Exceeds16Characters;

		assert_eq!(actual, expected);
	}

	#[test]
	fn speak_message() {
		let string = "Speak! 🗣️";

		let actual = is_valid_clock_name(string);
		let expected = ClockNameValidity::HasEmoji;

		assert_eq!(actual, expected);
	}

	#[test]
	fn all_emojis() {
		let string = "🥰🐴🎲🛑";

		let actual = is_valid_clock_name(string);
		let expected = ClockNameValidity::HasEmoji;

		assert_eq!(actual, expected);
	}

	#[test]
	fn valid_clock_name() {
		let string = "This Is Valid";

		let actual = is_valid_clock_name(string);
		let expected = ClockNameValidity::Valid;

		assert_eq!(actual, expected);
	}
}