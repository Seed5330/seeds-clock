fn main() {
	let manifest_dir = std::path::PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());


	let library_paths = std::collections::HashMap::from([
		(
			"root".to_string(),
			manifest_dir.join("../"),
		),		
		(
			"assets".to_string(),
			manifest_dir.join("../assets"),
		),
		(
			"slint-program".to_string(),
			manifest_dir.join("ui"),
		),

		(
			"helpers".to_string(),
			manifest_dir.join("../helpers/ui"),
		),
		(
			"clock".to_string(),
			manifest_dir.join("../clock/ui"),
		),
		(
			"alarm".to_string(),
			manifest_dir.join("../alarm/ui"),
		),
(
			"stopwatch".to_string(),
			manifest_dir.join("../stopwatch/ui"),
		),
(
			"timer".to_string(),
			manifest_dir.join("../timer/ui"),
		),
	]);

	let config = slint_build::CompilerConfiguration::new()
		.with_library_paths(library_paths);

	slint_build::compile_with_config("ui/slint-program.slint", config).unwrap();
}
