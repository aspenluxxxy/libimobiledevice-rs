use cc::Build;
use std::path::PathBuf;

/// Builds libirecovery, returning the include directory on success.
pub fn build() -> Vec<String> {
	let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").expect("No OUT_DIR set!"))
		.join("libirecovery-build");
	let libirecovery_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("libirecovery");
	let libirecovery_src = libirecovery_dir.join("src");
	let include_path = vec![
		libirecovery_dir.join("include").display().to_string(),
		std::env::var("DEP_USB_1.0_INCLUDE").expect("libusb1-sys failed to compile"),
	];

	let libirecovery_files = ["libirecovery.c", "utils.c", "thread.c"]
		.iter()
		.map(|file| libirecovery_src.join(file))
		.collect::<Vec<PathBuf>>();

	let mut builder = Build::new();
	if cfg!(target_vendor = "apple") {
		builder.define("HAVE_IOKIT", "1");
	}
	builder
		.define(
			"PACKAGE_URL",
			"\"https://github.com/libimobiledevice/libirecovery\"",
		)
		.define(
			"PACKAGE_BUGREPORT",
			"\"https://github.com/libimobiledevice/libirecovery/issues/new/choose\"",
		)
		.files(libirecovery_files)
		.includes(&include_path)
		.out_dir(out_dir)
		.static_flag(true)
		.warnings(false)
		.compile("usbmuxd-2.0");

	include_path
}
