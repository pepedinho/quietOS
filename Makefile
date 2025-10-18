
BUILD_DIR  := build
TARGET_DIR := target/i686-none/debug
KERNEL     := $(TARGET_DIR)/quiet
ISO_DIR    := $(BUILD_DIR)/isodir
ISO        := $(BUILD_DIR)/quiet.iso
GRUBCFG    := grub.cfg
QEMU       := qemu-system-i386
RUSTC      := cargo +nightly
TARGET     := arch/i686-none.json

SRC := $(shell find src -name '*.rs')

CYAN := \033[38;5;217m
ORANGE := \033[38;5;215m


.PHONY: all clean run iso rust re reb draw

all: run

draw:
	@echo "$(ORANGE)⠀⠀⠀ ⠀⠀⠀ ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\033[0m"; sleep 0.05
	@echo "$(ORANGE)⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⡼⡽⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠂⠀⠀\033[0m"; sleep 0.05
	@echo "$(ORANGE)⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⠠⣠⣢⡧⣏⡿⣽⢯⣿⡷⣧⣦⣦⣤⡤⣯⣯⡏⡇⠀⠀⠀⠀⠀⠀⢠⡃⠀⠀⠐⠀⠀⠀\033[0m"; sleep 0.05
	@echo "$(ORANGE)⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢄⣓⡴⣿⡋⠏⠎⠃⠉⠀⠀⠀⠀⠀⠉⠛⠷⡟⡏⣟⡷⡵⣂⠀⠀⠀⠀⢰⣿⠁⠀⠀⠀⠀⠀⡀\033[0m"; sleep 0.05
	@echo "$(ORANGE)⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⢠⣜⣯⣕⠿⠑⠌$(CYAN)⠂⠠⡀⠄⢀$(ORANGE)⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠛⠯⣧⣥⣧⣄⣴⣿⡇⠀⠀⠀⠀⠀⠀⣿\033[0m"; sleep 0.05
	@echo "$(ORANGE)⠀⠀⠀⠀⠀⠀⠀⠀⠀⣄⢝⣏⣕⠏⠁$(CYAN)⠄⡪⠉⠁⢂⣷⡆⣴⣄⠂⡀$(ORANGE)⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠨⠍⢿⣿⣏⡁⠀⠀⠀⠀⢀⣮⡅\033[0m"; sleep 0.05
	@echo "$(ORANGE)⠀⠀⠀⠀⠀⠀⠀⢀⣌⣣⡷⡟⠁$(CYAN)⢀⠁⠠⠁⠀⠀⠠⠫⡟⠛⡟⠃⠀⠄$(ORANGE)⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻⡵⣥⣭⣵⣿⣼⡷⠀\033[0m"; sleep 0.05
	@echo "$(ORANGE)⠀⠀⠀⠀⠀⠀⣀⣯⠏⠏⠃⠀$(CYAN)⣄⠄⣳⡎⢄⢀⣀⡌⢇⣿⣗⡨⡆⡀⠂$(ORANGE)⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠁⠙⣟⣿⡟⠁⠀\033[0m"; sleep 0.05
	@echo "$(ORANGE)⠀⠀⠀⠀⢠⠞⠉⠀⠀⠀⠀⠀$(CYAN)⢻⣇⡈⢟⣟⣿⣏⢏⡛⣏⡟⣜⡃⡃⠈$(ORANGE)⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡸⠀⠉⠀⠀\033[0m"; sleep 0.05
	@echo "$(ORANGE)⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀$(CYAN)⠐⣿⣟⣷⣥⣬⣧⣟⡟⡧⡑⢰⣏⡆⠁$(ORANGE)⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⠎⠀⠀⠀⠀⠀\033[0m"; sleep 0.05
	@echo "$(ORANGE)⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀$(CYAN)⠣⣻⣍⡍⣖⡝⡇⣃⢪⢔⣩⡣⢁⠁$(ORANGE)⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣈⠟⠳⣦⡀⠀⠀⠀⠀\033[0m"; sleep 0.05
	@echo "$(ORANGE)⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀$(CYAN)⠡⠣⣩⣦⣈⣏⡡⣣⢝⠇⠠$(ORANGE)⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠀⣀⡤⠒⠁⠀⠀⠀⠇⠀⠀⠀⠀\033[0m"; sleep 0.05
	@echo "$(ORANGE)⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀$(CYAN)⠈⠉⠍⡉⠁⡉⡀⠨⠈$(ORANGE)⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⠬⠛⣧⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\033[0m"; sleep 0.05
	@echo "$(ORANGE)⠀⠀⠀⠀⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠠⣀⡄⠔⠉⠀⠀⠀⠀⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\033[0m"; sleep 0.05
	@echo "$(ORANGE)⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠈⠁⠒⠲⣦⠂⠒⠋⠉⠙⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\033[0m"; sleep 0.05
	@echo "$(ORANGE)⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\033[0m"; sleep 0.05
	@echo "$(ORANGE)                       ███            █████   $(CYAN)    ███████     █████████ \033[0m"; sleep 0.05
	@echo "$(ORANGE)                      ▒▒▒            ▒▒███    $(CYAN)  ███▒▒▒▒▒███  ███▒▒▒▒▒███\033[0m"; sleep 0.05
	@echo "$(ORANGE)  ████████ █████ ████ ████   ██████  ███████  $(CYAN) ███     ▒▒███▒███    ▒▒▒ \033[0m"; sleep 0.05
	@echo "$(ORANGE) ███▒▒███ ▒▒███ ▒███ ▒▒███  ███▒▒███▒▒▒███▒   $(CYAN)▒███      ▒███▒▒█████████ \033[0m"; sleep 0.05
	@echo "$(ORANGE)▒███ ▒███  ▒███ ▒███  ▒███ ▒███████   ▒███    $(CYAN)▒███      ▒███ ▒▒▒▒▒▒▒▒███\033[0m"; sleep 0.05
	@echo "$(ORANGE)▒███ ▒███  ▒███ ▒███  ▒███ ▒███▒▒▒    ▒███ ███$(CYAN)▒▒███     ███  ███    ▒███\033[0m"; sleep 0.05
	@echo "$(ORANGE)▒▒███████  ▒▒████████ █████▒▒██████   ▒▒█████ $(CYAN) ▒▒▒███████▒  ▒▒█████████ \033[0m"; sleep 0.05
	@echo "$(ORANGE) ▒▒▒▒▒███   ▒▒▒▒▒▒▒▒ ▒▒▒▒▒  ▒▒▒▒▒▒     ▒▒▒▒▒  $(CYAN)   ▒▒▒▒▒▒▒     ▒▒▒▒▒▒▒▒▒  \033[0m"; sleep 0.05
	@echo "$(ORANGE)     ▒███                                                                \033[0m"; sleep 0.05
	@echo "$(ORANGE)     █████                                                               \033[0m"; sleep 0.05
	@echo "$(ORANGE)    ▒▒▒▒▒                                                                \033[0m"; sleep 0.05

rust: $(KERNEL)

$(KERNEL): $(SRC)
	$(RUSTC) build -Z build-std=core,compiler_builtins --target $(TARGET)

iso: $(ISO)

$(ISO): $(KERNEL) $(GRUBCFG)
	@echo "==> Creating ISO..."
	mkdir -p $(ISO_DIR)/boot/grub
	cp $(KERNEL) $(ISO_DIR)/boot/quietOS
	cp $(GRUBCFG) $(ISO_DIR)/boot/grub/grub.cfg
	grub-file --is-x86-multiboot $(ISO_DIR)/boot/quietOS
	grub-mkrescue --compress=xz -o $(ISO) $(ISO_DIR) --modules="normal multiboot part_msdos ext2"
	@echo "ISO created: $(ISO)"

run: $(ISO) draw
	$(QEMU) -cdrom $(ISO) -m 512M

clean:
	rm -rf $(BUILD_DIR)
	$(RUSTC) clean

# Rebuild and run
re: clean run

reb: clean iso