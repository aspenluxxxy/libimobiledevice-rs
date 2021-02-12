use cc::Build;
use std::path::PathBuf;

/// Builds libplist, returning the include directories.
pub fn build() -> Vec<String> {
	let out_dir =
		PathBuf::from(std::env::var_os("OUT_DIR").expect("No OUT_DIR set!")).join("libplist-build");
	let libplist_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("libplist");
	let libplist_src = libplist_dir.join("src");
	let libplist_include = libplist_dir.join("include");
	let libcnary_dir = libplist_dir.join("libcnary");
	let libcnary_include = libcnary_dir.join("include");

	let include_args = vec![
		libcnary_include.display().to_string(),
		libplist_include.display().to_string(),
	];

	let libcnary_files = ["node.c", "node_list.c"]
		.iter()
		.map(|file| libcnary_dir.join(file))
		.collect::<Vec<PathBuf>>();

	let libplist_files = [
		"base64.c",
		"bplist.c",
		"bytearray.c",
		"hashtable.c",
		"plist.c",
		"ptrarray.c",
		"time64.c",
		"xplist.c",
	]
	.iter()
	.map(|file| libplist_src.join(file))
	.collect::<Vec<PathBuf>>();

	Build::new()
		.define("HAVE_TM_TM_GMTOFF", "1")
		.define("HAVE_TM_TM_ZONE", "1")
		.files(libcnary_files)
		.files(libplist_files)
		.include(libcnary_include)
		.include(libplist_include)
		.include(libplist_src)
		.out_dir(out_dir)
		.static_flag(true)
		.warnings(false)
		.compile("plist");

	include_args
}
