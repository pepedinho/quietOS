use std::{env, path::Path, process::Command};

fn main() {
    let target = env::var("TARGET").unwrap_or_default();

    if target.contains("none") {
        let out_dir = env::var("OUT_DIR").unwrap();

        let asm_files = [
            "src/boot/m_boot_header.s",
            "src/boot/gdt.s",
            "src/boot/pmode_switch.s",
            "src/boot/common_stub.s",
            "src/boot/irq_wrappers.s",
        ];

        for file in &asm_files {
            let obj_path = Path::new(&out_dir)
                .join(Path::new(file).file_stem().unwrap())
                .with_extension("o");

            let status = Command::new("nasm")
                .args(["-felf32", file, "-o"])
                .arg(&obj_path)
                .status()
                .unwrap_or_else(|_| panic!("Failed to run nasm on {}", file));

            if !status.success() {
                panic!("nasm failed on {}", file);
            }

            println!("cargo:rustc-link-arg={}", obj_path.display());
        }
        let status = Command::new("gcc")
            .args([
                "-m32",
                "-ffreestanding",
                "-c",
                "src/boot/early_init.c",
                "-o",
            ])
            .arg(format!("{out_dir}/early_init.o"))
            .status()
            .expect("failed to compile early_init.c");

        if !status.success() {
            panic!("gcc failed on {}", "src/boot/early_init.c");
        }

        println!("cargo:rustc-link-arg={}/early_init.o", out_dir);

        println!("cargo:rerun-if-changed=src/boot/m_boot_header.s");
        println!("cargo:rerun-if-changed=src/boot/gdt.s");
        println!("cargo:rerun-if-changed=src/boot/pmode_switch.s");
        println!("cargo:rerun-if-changed=src/boot/common_stub.s");
        println!("cargo:rerun-if-changed=src/boot/irq_wrappers.s");
        println!("cargo:rerun-if-changed=src/boot/early_init.c");
    }
}
