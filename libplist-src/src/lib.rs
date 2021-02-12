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

	Build::new()
		.define("HAVE_TM_TM_GMTOFF", "1")
		.define("HAVE_TM_TM_ZONE", "1")
		.file(libcnary_dir.join("node.c"))
		.file(libcnary_dir.join("node_list.c"))
		.file(libplist_src.join("base64.c"))
		.file(libplist_src.join("base64.h"))
		.file(libplist_src.join("bplist.c"))
		.file(libplist_src.join("bytearray.c"))
		.file(libplist_src.join("bytearray.c"))
		.file(libplist_src.join("hashtable.c"))
		.file(libplist_src.join("hashtable.h"))
		.file(libplist_src.join("plist.c"))
		.file(libplist_src.join("plist.h"))
		.file(libplist_src.join("ptrarray.c"))
		.file(libplist_src.join("ptrarray.h"))
		.file(libplist_src.join("strbuf.h"))
		.file(libplist_src.join("time64.c"))
		.file(libplist_src.join("time64.h"))
		.file(libplist_src.join("time64_limits.h"))
		.file(libplist_src.join("xplist.c"))
		.include(libcnary_include)
		.include(libplist_include)
		.include(libplist_src)
		.out_dir(out_dir)
		.static_flag(true)
		.warnings(false)
		.compile("plist");

	include_args
}
