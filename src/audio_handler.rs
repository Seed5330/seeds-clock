use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, source::Source};

pub fn play_audio(path: &str) -> Result<(), std::io::Error> {
	// Get an OS-Sink handle to the default physical sound device.
	// Note that the playback stops when the sink_handle is dropped.
	let sink_handle = rodio::DeviceSinkBuilder::open_default_sink()
	        .expect("open default audio stream");

	// Load a sound from a file, using a path relative to Cargo.toml
	let file = BufReader::new(File::open(path)?);
	// Note that the playback stops when the player is dropped
	let player = rodio::play(&sink_handle.mixer(), file);

	// The sound plays in a separate audio thread,
	// so we need to keep the main thread alive while it's playing.
	std::thread::sleep(std::time::Duration::from_secs(5));

	Ok(())
}

pub fn play_audio_looped() {
    // 1. Get a handle to the default output device
    let sinker = rodio::DeviceSinkBuilder::open_default_sink().unwrap();
	let sink = rodio::Player::connect_new(&sinker.mixer());

    // 2. Load and decode your audio file
    let file = File::open("assets/starry_night.mp3").unwrap();
    let reader = BufReader::new(file);
    let source = Decoder::new(reader).unwrap();

    // 3. Chain .repeat_infinite() to loop the track infinitely
    let looped_source = source.repeat_infinite();

    // 4. Append to the sink and let it play
    sink.append(looped_source);

    // Keep the program alive so the background thread can play the loop
    sink.sleep_until_end();
}