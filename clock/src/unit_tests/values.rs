#![cfg(test)]

use slint::SharedString;

use crate::values;

#[test]
fn hours_offsets() {
	let offsets_vec = vec![
		"-12", "-11", "-10", "-09", "-08", "-07",
		"-06", "-05", "-04", "-03", "-02", "-01",
		"+00",
		"+01", "+02", "+03", "+04", "+05", "+06", "+07",
		"+08", "+09", "+10", "+11", "+12", "+13", "+14",
	];

	let expected = offsets_vec
		.into_iter()
		.map(SharedString::from)
		.collect::<Vec<SharedString>>();

	let actual = values::hours_offset_vec();

	pretty_assertions::assert_eq!(expected, actual);
}

#[test]
fn minutes_offsets() {
	let offsets_vec = vec![
		"00", "01", "02", "03", "04", "05",
		"06", "07", "08", "09", "10", "11",

		"12", "13", "14", "15", "16", "17",
		"18", "19", "20", "21", "22", "23",

		"24", "25", "26", "27", "28", "29",
		"30", "31", "32", "33", "34", "35",

		"36", "37", "38", "39", "40", "41",
		"42", "43", "44", "45", "46", "47",

		"48", "49", "50", "51", "52", "53",
		"54", "55", "56", "57", "58", "59",
	];

	let expected = offsets_vec
		.into_iter()
		.map(SharedString::from)
		.collect::<Vec<SharedString>>();

	let actual = values::minutes_offset_vec();

	pretty_assertions::assert_eq!(expected, actual);
}