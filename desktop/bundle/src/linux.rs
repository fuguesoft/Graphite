use std::error::Error;

use crate::common::*;

pub fn main() -> Result<(), Box<dyn Error>> {
	let app_bin = build_bin("graphite-desktop-platform-linux", None)?;

	if std::env::args().any(|a| a == "flatpak") {
		// run_command("npm", &["run", "build-desktop-wasm"])?;
		// build_bin("raster-nodes-shaders", None)?;
		// let flatpak_src = target_path().join("graphite-flatpak-src");
		// clean_dir(&flatpak_src);
		// let workspace_dir = workspace_path();
		// copy_dir(&workspace_dir.join("frontend/dist"), &flatpak_src.join("resources"));
		// std::fs::copy(
		// 	workspace_dir.join("target/spirv-builder/spirv-unknown-naga-wgsl/release/deps/graphene_raster_nodes_shaders_entrypoint.wgsl"),
		// 	flatpak_src.join("graphene_raster_nodes_shaders_entrypoint.wgsl"),
		// )
		// .unwrap();
		let flatpak_src = target_path().join("graphite-flatpak-src");
		let bin_dir = flatpak_src.join("bin");
		clean_dir(&bin_dir);
		std::fs::copy(&app_bin, bin_dir.join(APP_BIN))?;
		let lib_dir = flatpak_src.join("lib");
		let cef_dir = lib_dir.join("cef");
		return Ok(());
	}

	// TODO: Consider adding more useful cli
	if std::env::args().any(|a| a == "open") {
		run_command(&app_bin.to_string_lossy(), &[]).expect("failed to open app");
	} else {
		println!("Binary built and placed at {}", app_bin.to_string_lossy());
		eprintln!("Bundling for Linux is not yet implemented.");
		eprintln!("You can still start the app with the `open` subcommand. `cargo run -p graphite-desktop-bundle -- open`");
		std::process::exit(1);
	}

	Ok(())
}
