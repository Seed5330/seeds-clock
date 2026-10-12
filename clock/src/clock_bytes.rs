use redb::{Database, ReadableDatabase, TableDefinition};
use speedy::{Readable, Writable};
use crate::{ClockData, clock_data::ExtensionClock};

const TABLE: TableDefinition<&str, Vec<Vec<u8>>> = TableDefinition::new("clocks");
const CLOCKS_PATH: &str = "config/clock.redb";

#[derive(Readable, Writable)]
pub struct ClockBytes {
	name: String,
	hours_offset: i32,
	minutes_offset: i32,
}


impl ClockBytes {
	pub fn clock_data_to_bytes(clock_data: &ClockData) -> Vec<u8> {
		let clock_bytes = Self {
			name: clock_data.name.to_string(),
			hours_offset: clock_data.hours_offset,
			minutes_offset: clock_data.minutes_offset,
		};

		clock_bytes.write_to_vec().expect("String and i32 are convertable to bytes.")
	}

	pub fn bytes_to_clock_data(bytes: &Vec<u8>) -> Result<ClockData, speedy::Error>  {
		let clock_bytes = Self::read_from_buffer(bytes)?;
		
		Ok(ClockData::new(
			clock_bytes.name.as_str(),
			clock_bytes.hours_offset,
			clock_bytes.minutes_offset,
		))
	}

	pub fn save_clocks(clocks: Vec<ClockData>) {
		let clocks_vec_bytes: Vec<Vec<u8>> = clocks
			.iter()
			.map(|clock_data| Self::clock_data_to_bytes(clock_data))
			.collect();

		let clocks_file = Database::create(CLOCKS_PATH).expect("File will be created or opened.");
		let clocks_writer = clocks_file.begin_write().expect("File will always be there.");
		{
			let mut writing = clocks_writer.open_table(TABLE)
				.expect("Writing table is opened or created.");
			writing.insert("Clocks", &clocks_vec_bytes).expect("Bytes are always of ClockData");
		}
		clocks_writer.commit().expect("No problems writing to file.");
	}

	pub fn get_clocks() -> Option<Vec<ClockData>> {
		let clocks_file = Database::create(CLOCKS_PATH).expect("File will be created or open");
		let clocks_reader =  clocks_file.begin_read().expect("File will always be there.");

		let reading = clocks_reader.open_table(TABLE);
		
		if let Ok(_) = &reading {}
		else { return None; }

		let getting_bytes = reading
			.expect("This will execute if the table is read successfully.")
			.get("Clocks");

		if let Ok(_) = &getting_bytes {}
		else { return None; }

		let clocks_bytes = getting_bytes
			.unwrap()
			.expect("A guard clause should prevent this from executing if the bytes retrived is invalid.")
			.value();

		if clocks_bytes.is_empty() {
			return None;
		}
		
		let clocks_vec: Vec<ClockData> = clocks_bytes
			.iter()
			.map(|clock_bytes| Self::bytes_to_clock_data(clock_bytes).expect("Bytes are always of ClockData"))
			.collect();

		Some(clocks_vec)
	}
}