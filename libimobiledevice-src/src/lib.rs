use cc::Build;
use std::path::PathBuf;

/// Builds libusbmuxd, returning the include directory on success.
pub fn build() -> Vec<String> {
	let mut include_path = libusbmuxd_src::build();

	let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").expect("No OUT_DIR set!"))
		.join("libimobiledevice-build");
	let libimobiledevice_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("libimobiledevice");
	let libimobiledevice_common = libimobiledevice_dir.join("common");
	let libimobiledevice_src = libimobiledevice_dir.join("src");
	include_path.push(libimobiledevice_dir.display().to_string());
	include_path.push(libimobiledevice_common.display().to_string());
	include_path.push(libimobiledevice_dir.join("include").display().to_string());

	let common_files = [
		"debug.c",
		"debug.h",
		"socket.c",
		"socket.h",
		"thread.c",
		"thread.h",
		"userpref.c",
		"userpref.h",
		"utils.c",
		"utils.h",
	]
	.iter()
	.map(|file| libimobiledevice_common.join(file))
	.collect::<Vec<PathBuf>>();

	let lib_files = [
		"idevice.c",
		"idevice.h",
		"service.c",
		"service.h",
		"property_list_service.c",
		"property_list_service.h",
		"device_link_service.c",
		"device_link_service.h",
		"lockdown.c",
		"lockdown.h",
		"afc.c",
		"afc.h",
		"file_relay.c",
		"file_relay.h",
		"notification_proxy.c",
		"notification_proxy.h",
		"installation_proxy.c",
		"installation_proxy.h",
		"sbservices.c",
		"sbservices.h",
		"mobile_image_mounter.c",
		"mobile_image_mounter.h",
		"screenshotr.c",
		"screenshotr.h",
		"mobilesync.c",
		"mobilesync.h",
		"mobilebackup.c",
		"mobilebackup.h",
		"house_arrest.c",
		"house_arrest.h",
		"mobilebackup2.c",
		"mobilebackup2.h",
		"misagent.c",
		"misagent.h",
		"restore.c",
		"restore.h",
		"diagnostics_relay.c",
		"diagnostics_relay.h",
		"heartbeat.c",
		"heartbeat.h",
		"debugserver.c",
		"debugserver.h",
		"webinspector.c",
		"webinspector.h",
		"mobileactivation.c",
		"mobileactivation.h",
		"preboard.c",
		"preboard.h",
		"companion_proxy.c",
		"companion_proxy.h",
		"syslog_relay.c",
		"syslog_relay.h",
	]
	.iter()
	.map(|file| libimobiledevice_src.join(file))
	.collect::<Vec<PathBuf>>();

	Build::new()
		.define("HAVE_VASPRINTF", "1")
		.define("HAVE_ASPRINTF", "1")
		.define("HAVE_STPCPY", "1")
		.define("HAVE_SYS_TYPES_H", "1")
		.define("PACKAGE_NAME", "\"libimobiledevice\"")
		.define(
			"PACKAGE_URL",
			"\"https://github.com/libimobiledevice/libimobiledevice\"",
		)
		.define(
			"PACKAGE_BUGREPORT",
			"\"https://github.com/libimobiledevice/libimobiledevice/issues/new\"",
		)
		.files(common_files)
		.files(lib_files)
		.includes(&include_path)
		.out_dir(out_dir)
		.static_flag(true)
		.warnings(false)
		.compile("imobiledevice-1.0");

	include_path
}
