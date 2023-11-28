use std::process::Command;

fn main() {
    // println!("cargo:rerun-if-changed=NULL");

    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=static/");
    println!("cargo:rerun-if-changed=package.json");

    // Check if Bun is installed (usually installed in ~/.bun/bin/bun)
    let home = std::env::var("HOME").unwrap();
    let bun_path = format!("{}/.bun/bin/bun", home);
    if !std::path::Path::new(&bun_path).exists() {
        panic!(
            r#"
            ----------------------------------------
            Bun is not installed. Please install Bun from bun.sh and try again.

            HOME: {}
            
            curl -fsSL https://bun.sh/install | bash
            ----------------------------------------
        "#,
            home
        );
    }

    // Run "bun install"
    let output = Command::new(&bun_path).arg("install").output();

    let output = match output {
        Ok(output) => output,
        Err(e) => panic!(
            r#"
            ----------------------------------------
            Failed to execute Bun binary during install.

            Error: {}
            ----------------------------------------
        "#,
            e
        ),
    };
    if output.status.code().unwrap() != 0 {
        panic!(
            r#"
            ----------------------------------------
            Bun failed to install dependencies. Please try again.

            Status: {}
            Error: {}
            ----------------------------------------
        "#,
            output.status.code().unwrap(),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // Run "bun build"
    let output = Command::new(&bun_path).arg("run").arg("build").output();

    let output = match output {
        Ok(output) => output,
        Err(e) => panic!(
            r#"
            ----------------------------------------
            Failed to execute Bun binary during build.

            Error: {}
            ----------------------------------------
        "#,
            e
        ),
    };
    if output.status.success() {
        panic!(
            r#"
            ----------------------------------------
            Bun failed to build. Please try again.

            Error: {}
            ----------------------------------------
        "#,
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
