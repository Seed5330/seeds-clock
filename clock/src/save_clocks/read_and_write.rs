use redb::{Database, ReadableDatabase, TableDefinition};
use speedy::{Readable, Writable};

use crate::clock_data::ClockData;

type ClockDataBytes = Vec<u8>;

const TABLE: TableDefinition<&str, Vec<ClockDataBytes>> = TableDefinition::new("clock_config");
const CONFIG_PATH: &str = "config/clock.redb";

pub fn write_clocks_to_file(clock_data_vec: Vec<ClockData>) -> Result<(), redb::Error> {
	let clock_bytes = clock_data_vec
		.iter()
		.map(|clock_data| clock_data.write_to_vec().expect("Struct values are std."))
		.collect::<Vec<ClockDataBytes>>();

	
	let database = Database::create(CONFIG_PATH)?;
	let writer = database.begin_write()?;
	{
        let mut table = writer.open_table(TABLE)?;
        table.insert("clock_list", &clock_bytes)?;
    }
    writer.commit()?;

	Ok(())
}

pub fn read_clocks_from_file() -> Result<Vec<ClockData>, redb::Error> {
	let database = Database::open(CONFIG_PATH)?;
	let reader = database.begin_read()?;

	let table = reader.open_table(TABLE)?;

	let clock_bytes = table.get("clock_list")?.expect("Key is \"clock_list\"").value();
	let clock_data_vec = clock_bytes
		.iter()
		.map(|clock_data_bytes| ClockData::read_from_buffer(clock_data_bytes).expect("Bytes convert to ClockData."))
		.collect::<Vec<ClockData>>();

	Ok(clock_data_vec)
}