fn main() {
    // Moves found D compiler to the build directory
    let mut location = which::which("dmd");

    // If DMD wasn't found, try LDC2

    if location.is_err() {
        location = which::which("ldc2");
    }

    let location = location.unwrap();
    let location = location.to_str().unwrap();

    // Add .exe if on Windows
    if cfg!(any(target_os = "linux", target_os = "macos")) {
        std::fs::copy(location, "./d_compiler");
    } else if cfg!(target_os = "windows") {
        std::fs::copy(location, "./d_compiler.exe");
    }
}