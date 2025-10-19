# Quiet OS

Quiet OS is an experimental, modular kernel written in Rust.

---

```zsh
        ⠀⠀⠀ ⠀⠀⠀ ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⡼⡽⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠂⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⠠⣠⣢⡧⣏⡿⣽⢯⣿⡷⣧⣦⣦⣤⡤⣯⣯⡏⡇⠀⠀⠀⠀⠀⠀⢠⡃⠀⠀⠐⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢄⣓⡴⣿⡋⠏⠎⠃⠉⠀⠀⠀⠀⠀⠉⠛⠷⡟⡏⣟⡷⡵⣂⠀⠀⠀⠀⢰⣿⠁⠀⠀⠀⠀⠀⡀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⢠⣜⣯⣕⠿⠑⠌⠂⠠⡀⠄⢀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠛⠯⣧⣥⣧⣄⣴⣿⡇⠀⠀⠀⠀⠀⠀⣿
⠀⠀⠀⠀⠀⠀⠀⠀⠀⣄⢝⣏⣕⠏⠁⠄⡪⠉⠁⢂⣷⡆⣴⣄⠂⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠨⠍⢿⣿⣏⡁⠀⠀⠀⠀⢀⣮⡅
⠀⠀⠀⠀⠀⠀⠀⢀⣌⣣⡷⡟⠁⢀⠁⠠⠁⠀⠀⠠⠫⡟⠛⡟⠃⠀⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻⡵⣥⣭⣵⣿⣼⡷⠀
⠀⠀⠀⠀⠀⠀⣀⣯⠏⠏⠃⠀⣄⠄⣳⡎⢄⢀⣀⡌⢇⣿⣗⡨⡆⡀⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠁⠙⣟⣿⡟⠁⠀
⠀⠀⠀⠀⢠⠞⠉⠀⠀⠀⠀⠀⢻⣇⡈⢟⣟⣿⣏⢏⡛⣏⡟⣜⡃⡃⠈⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡸⠀⠉⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠐⣿⣟⣷⣥⣬⣧⣟⡟⡧⡑⢰⣏⡆⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⠎⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠣⣻⣍⡍⣖⡝⡇⣃⢪⢔⣩⡣⢁⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣈⠟⠳⣦⡀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠡⠣⣩⣦⣈⣏⡡⣣⢝⠇⠠⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠀⣀⡤⠒⠁⠀⠀⠀⠇⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠍⡉⠁⡉⡀⠨⠈⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⠬⠛⣧⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠠⣀⡄⠔⠉⠀⠀⠀⠀⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠈⠁⠒⠲⣦⠂⠒⠋⠉⠙⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                       ███            █████       ███████     █████████ 
                      ▒▒▒            ▒▒███      ███▒▒▒▒▒███  ███▒▒▒▒▒███
  ████████ █████ ████ ████   ██████  ███████   ███     ▒▒███▒███    ▒▒▒ 
 ███▒▒███ ▒▒███ ▒███ ▒▒███  ███▒▒███▒▒▒███▒   ▒███      ▒███▒▒█████████ 
▒███ ▒███  ▒███ ▒███  ▒███ ▒███████   ▒███    ▒███      ▒███ ▒▒▒▒▒▒▒▒███
▒███ ▒███  ▒███ ▒███  ▒███ ▒███▒▒▒    ▒███ ███▒▒███     ███  ███    ▒███
▒▒███████  ▒▒████████ █████▒▒██████   ▒▒█████  ▒▒▒███████▒  ▒▒█████████ 
 ▒▒▒▒▒███   ▒▒▒▒▒▒▒▒ ▒▒▒▒▒  ▒▒▒▒▒▒     ▒▒▒▒▒     ▒▒▒▒▒▒▒     ▒▒▒▒▒▒▒▒▒  
     ▒███                                                                
     █████                                                               
    ▒▒▒▒▒                                                                                                          
```

<!-- <img width="620" height="481" alt="image" src="https://github.com/user-attachments/assets/25a4b80c-ab9a-40ee-9915-4bed001fc201" /> -->


## Features

* **Bootloader Support**

  * Multiboot-compatible header for GRUB.
  * Kernel can be loaded directly into memory by GRUB.

* **Rust + Assembly Integration**

  * Core kernel logic written in Rust.
  * Boot routines implemented in NASM.
  * Linked using GNU LD for seamless integration.

* **ISO Build & Emulation**

  * Automated build process via Makefile.
  * ISO creation using `grub-mkrescue`.
  * Emulation and testing using QEMU.

* **Console I/O Module**

  * Basic VGA text output.
  * Minimal ANSI color parsing (foreground and bright colors).
  * Modular design allowing future expansion (e.g., background colors, input handling).

---

## Planned Features

* Full ANSI color support including background colors.
* Keyboard input handling module.
* Modular expansion allowing easy addition of new kernel features.

---

## Build & Run

1. Build the kernel and ISO:

```bash
make iso
```

2. Launch the OS in QEMU:

```bash
make run
```

3. Clean build artifacts:

```bash
make clean
```

---

## Documentation used

* General OS Development: [OSDev Expanded Main Page](https://wiki.osdev.org/Expanded_Main_Page)
* Console & Colors: [ANSI Escape Codes](https://en.wikipedia.org/wiki/ANSI_escape_code), [VGA Color Palettes](https://www.fountainware.com/EXPL/vga_color_palettes.htm)

---

This README presents the current features and workflow of Quiet OS. Detailed documentation about modules, development notes, and future design plans should be maintained in separate files.
