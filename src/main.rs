use std::io;

fn main() -> io::Result<()> {
	// read env variables that were set in build script
	let uefi_path = env!("UEFI_PATH");

	// choose whether to start the UEFI or BIOS image
	let uefi = true;

	let mut cmd = std::process::Command::new("qemu-system-x86_64");
	if uefi {
		cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
		cmd.arg("-drive")
			.arg(format!("format=raw,file={uefi_path}"));
	}
	let mut child = cmd.spawn()?;
	child.wait()?;

	Ok(())
}