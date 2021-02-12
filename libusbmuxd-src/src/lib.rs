use cc::Build;
use std::path::PathBuf;

/// Builds libusbmuxd, returning the include directory on success.
pub fn build() -> Vec<String> {
	let mut include_path = libplist_src::build();

	let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").expect("No OUT_DIR set!"))
		.join("libusbmuxd-build");
	let libusbmuxd_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("libusbmuxd");
	let libusbmuxd_src = libusbmuxd_dir.join("src");
	let libusbmuxd_common = libusbmuxd_dir.join("common");
	include_path.push(libusbmuxd_common.display().to_string());
	include_path.push(libusbmuxd_dir.join("include").display().to_string());

	Build::new()
		.define("HAVE_STPNCPY", "1")
		.define("PACKAGE_STRING", "\"libusbmuxd\"")
		.file(libusbmuxd_common.join("collection.c"))
		.file(libusbmuxd_common.join("socket.c"))
		.file(libusbmuxd_common.join("thread.c"))
		.file(libusbmuxd_src.join("libusbmuxd.c"))
		.includes(&include_path)
		.out_dir(out_dir)
		.static_flag(true)
		.warnings(false)
		.compile("usbmuxd-2.0");

	include_path
}
