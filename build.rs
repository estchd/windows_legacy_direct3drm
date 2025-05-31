fn main() {
	println!("cargo:rerun-if-changed=./windows/winmd/Microsoft.Win32.Graphics.Direct3DRM.winmd");
	println!("cargo:rerun-if-changed=./windows/winmd/Microsoft.Win32.Graphics.Direct3D.Legacy.winmd");
	println!("cargo:rerun-if-changed=build.rs");

	windows_bindgen::bindgen([
		"--in",
		"default",
		".windows/winmd/Microsoft.Win32.Graphics.Direct3D.Legacy.winmd",
		".windows/winmd/Microsoft.Win32.Graphics.Direct3DRM.winmd",
		"--out",
		"src/bindings.rs",
		"--filter",
		"Microsoft.Win32.Graphics.Direct3DRM",
		"--reference",
		"windows,skip-root,Windows",
		"windows_legacy_direct3d,full,Microsoft.Win32.Graphics.Direct3D.Legacy"
	]).unwrap();
}