use std::process::Command;

fn main() {
    let submodule_dir = "vendor/OskGraphics";

    let status = Command::new("make")
        .current_dir(submodule_dir)
        .status()
        .expect("failed to run make — is it installed and on PATH?");

    if !status.success() {
        panic!("make failed while building OskGraphics");
    }

    println!("cargo:rustc-link-search=native={}", submodule_dir);
    println!("cargo:rustc-link-lib=static=OskGraphics");

    println!("cargo:rustc-link-lib=dylib=user32");
    println!("cargo:rustc-link-lib=dylib=gdi32");
    println!("cargo:rustc-link-lib=dylib=opengl32");

    println!("cargo:rerun-if-changed={}/src", submodule_dir);
}