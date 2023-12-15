use std::process::Command;

fn run_command(command: &str) {
    let output = Command::new("bash")
        .arg("-c")
        .arg(format!("source ~/.bashrc && {}", command))
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        // println!("cargo:warning={}", String::from_utf8_lossy(&output.stderr));
    } else {
        panic!(
            r#"
            ----------------------------------------
            Failed to execute command.

            Command: {}
            Error: {}
            ----------------------------------------
        "#,
            command,
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

fn main() {
    // println!("cargo:rerun-if-changed=NULL");

    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=static/");
    println!("cargo:rerun-if-changed=package.json");

    // Check if Bun is installed (usually installed in ~/.bun/bin/bun)
    let home = std::env::var("HOME").unwrap();
    let bun_path = format!("{}/.bun/bin/bun", home);
    if !std::path::Path::new(&bun_path).exists() {
        let ls_home = Command::new("ls")
            .arg("-la")
            .arg(&home)
            .output()
            .expect("Failed to execute command");
        panic!(
            r#"
            ----------------------------------------
            Bun is not installed. Please install Bun from bun.sh and try again.

            HOME: {}
            Bun path: {}
            ls -la $HOME: {}
            
            curl -fsSL https://bun.sh/install | bash
            ----------------------------------------
        "#,
            home,
            bun_path,
            String::from_utf8_lossy(&ls_home.stdout)
        );
    }

    // Run "bun install"
    run_command("bun install");

    // Run "bun run build"
    run_command("bun run --bun build");

    // Check if "output" directory exists
    if !std::path::Path::new("output").exists() {
        panic!(
            r#"
            ----------------------------------------
            Bun failed to build. Output "build" folder is missing.
            ----------------------------------------
        "#
        );
    }
}
