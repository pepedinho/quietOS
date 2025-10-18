use std::{env, path::Path, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=src/m_boot_header.s");

    let out_dir = env::var("OUT_DIR").unwrap();
    let obj_path = Path::new(&out_dir).join("m_boot_header.o");

    let status = Command::new("nasm")
        .args(&[
            "-felf32",
            "src/m_boot_header.s",
            "-o",
            obj_path.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to run nasm");

    if !status.success() {
        panic!("nasm failed");
    }

    println!("cargo:rustc-link-arg={}", obj_path.display());
}
