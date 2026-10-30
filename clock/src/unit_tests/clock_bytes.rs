// #![cfg(test)]

// use crate::{ClockData, clock_bytes::ClockBytes, clock_data::ExtensionClock};

// #[test]
// fn clock_data_bytes_conversion() {
// 	let clock_data = ClockData::new("File Clock", 4, 20);

// 	let bytes = ClockBytes::clock_data_to_bytes(&clock_data);
// 	let bytes_reversion = ClockBytes::bytes_to_clock_data(&bytes).unwrap();

// 	assert_eq!(clock_data, bytes_reversion);
// }

// #[test]
// fn write_and_read_file_for_clocks() {
// 	let clock_list = vec![
// 		ClockData::new("OneByte", -1, 00),
// 		ClockData::new("TwoByte", -2, 30),
// 		ClockData::new("ThreeByte", -3, 00),
// 		ClockData::new("FourByte", -4, 30),
// 		ClockData::new("FiveByte", -5, 00),
// 	];

// 	ClockBytes::save_clocks(clock_list.clone());
// 	let clock_files = ClockBytes::get_clocks().unwrap();

// 	assert_eq!(clock_list, clock_files);
// }