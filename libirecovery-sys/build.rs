use bindgen::Builder;
use std::path::PathBuf;

#[cfg(not(feature = "vendored"))]
fn link(mut binder: Builder) -> Builder {
	let package = pkg_config::Config::new()
		.probe("libirecovery-1.0")
		.expect("Could not find libirecovery! Ensure that it is installed!");

	for lib in package.libs {
		println!("cargo:rustc-link-lib={}", lib);
	}

	for include_path in package.include_paths {
		binder = binder.clang_arg(format!("-I{}", include_path.display()));
	}
	binder
}

#[cfg(feature = "vendored")]
fn link(mut binder: Builder) -> Builder {
	for include_path in libirecovery_src::build() {
		binder = binder.clang_arg(format!("-I{}", include_path));
	}
	binder
}

fn main() {
	let bindings = link(
		bindgen::builder()
			.header("wrapper.h")
			.parse_callbacks(Box::new(bindgen::CargoCallbacks)),
	)
	.generate()
	.expect("Unable to generate bindings for libirecovery!");

	let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("bindings.rs");
	bindings
		.write_to_file(out_path)
		.expect("Couldn't write bindings!");
}
