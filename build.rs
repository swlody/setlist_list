// stolen from https://gist.github.com/mcpar-land/b853da913ca7f45faa179704772961efs

use std::{
    io::{self, Write},
    process,
};

fn main() {
    println!("cargo:rerun-if-changed=tailwind.config.js");
    println!("cargo:rerun-if-changed=templates/styles/input.css");

    match process::Command::new("npx")
        .args([
            "tailwindcss",
            "-i",
            "templates/styles/input.css",
            "-o",
            "templates/styles/output.css",
        ])
        .output()
    {
        Ok(output) => {
            if !output.status.success() {
                let _ = io::stdout().write_all(&output.stdout);
                let _ = io::stdout().write_all(&output.stderr);
                panic!("Tailwind error");
            }
        }
        Err(e) => panic!("Tailwind error: {:?}", e),
    };
}
