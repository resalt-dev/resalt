use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=NULL");

    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=static/");
    println!("cargo:rerun-if-changed=package.json");
    println!("cargo:rerun-if-changed=bun.lockb");

    // Check if Bun is installed (usually installed in ~/.bun/bin/bun)
    let home = std::env::var("HOME").unwrap();
    let bun_path = format!("{}/.bun/bin/bun", home);
    if !std::path::Path::new(&bun_path).exists() {
        panic!(
            r#"
            ----------------------------------------
            Bun is not installed. Please install Bun from bun.sh and try again.
            
            curl -fsSL https://bun.sh/install | bash
            ----------------------------------------
        "#
        );
    }

    // Run "bun install"
    let output = Command::new("bun")
        .arg("install")
        .output()
        .expect("Failed to run bun install");

    if !output.status.success() {
        panic!(
            r#"
            ----------------------------------------
            Bun failed to install dependencies. Please try again.
            ----------------------------------------
        "#
        );
    }

    // Run "bun build"
    let output = Command::new("bun")
        .arg("run")
        .arg("build")
        .output()
        .expect("Failed to run bun build");

    if !output.status.success() {
        panic!(
            r#"
            ----------------------------------------
            Bun failed to build. Please try again.
            ----------------------------------------
        "#
        );
    }
}
