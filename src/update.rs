use std::process::{Command, exit};

pub fn update() -> ! {
    let status = Command::new("cargo")
        .args(["install", env!("CARGO_PKG_NAME"), "--force"])
        .status()
        .expect("Failed to run cargo install");

    if status.success() {
        println!("✅ Updated successfully.");
        exit(0)
    } else {
        eprintln!("❌ Update failed.");
        exit(1)
    }
}
