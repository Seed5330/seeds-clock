fn main() {
	const SEEDS_COMMON_PATH: &str = "C:/Users/seed5/Desktop/Programming/--Rust--/CommonScripts/seeds-common";
	const SLINT_PROGRAM_FILE: &str = "ui/slint_program.slint";

	let manifest_dir = std::path::PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());


	let library_paths = std::collections::HashMap::from([
		(
			"seeds-common".to_string(),
			manifest_dir.join(SEEDS_COMMON_PATH),
		),
		(
			"seeds-clock".to_string(),
			manifest_dir.join(""),
		),
	]);

	let config = slint_build::CompilerConfiguration::new()
		.with_library_paths(library_paths);

	slint_build::compile_with_config(SLINT_PROGRAM_FILE, config).unwrap();
}
