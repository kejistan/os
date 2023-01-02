use std::path::Path;

fn main() {
	// set by cargo, build scripts should use this directory for output files
	let out_dir = std::env::var_os("OUT_DIR").unwrap();
	// set by cargo's artifact dependency feature, see
	// https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#artifact-dependencies
	let kernel = std::env::var_os("CARGO_BIN_FILE_KERNEL_kernel").unwrap();

	// create an UEFI disk image (optional)
	let uefi_path = Path::new(&out_dir).join("uefi.img");
	bootloader::UefiBoot::new(&Path::new(&kernel))
		.create_disk_image(&uefi_path)
		.unwrap();

	// pass the disk image paths as env variables to the `main.rs`
	println!("cargo:rustc-env=UEFI_PATH={}", uefi_path.display());
}
