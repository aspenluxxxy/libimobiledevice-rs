use std::path::PathBuf;

fn main() {
	pkg_config::Config::new()
		.probe("libimobiledevice-1.0")
		.expect("Could not find libimobiledevice! Ensure that it is installed!");

	println!("cargo:rustc-link-lib=imobiledevice-1.0");

	let bindings = bindgen::builder()
		.header("wrapper.h")
		.parse_callbacks(Box::new(bindgen::CargoCallbacks))
		.generate()
		.expect("Unable to generate bindings for libimobiledevice!");

	let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("bindings.rs");
	bindings
		.write_to_file(out_path)
		.expect("Couldn't write bindings!");
}
