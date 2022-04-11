use std::path::Path;

fn main() {
    // Will rerun if d_compiler was not found or build.rs has changed
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=d_compiler");

    // Moves found D compiler to the build directory
    let mut location = which::which("dmd");

    // If DMD wasn't found, try LDC2
    if location.is_err() {
        location = which::which("ldc2");
    }

    let location = location.unwrap();
    let location = location.to_str().unwrap();

    // If it already exists in this directory, just ignore and stop
    if Path::new("./d_compiler").exists() || Path::new("./d_compiler.exe").exists() {
        return;
    }

    // Add .exe if on Windows
    let result = if cfg!(target_os = "windows") {
        std::fs::copy(location, "./d_compiler.exe")
    } else {
        std::fs::copy(location, "./d_compiler")
    };

    if result.is_err() {
        panic!("{}", result.unwrap_err());
    }
}
