MODE ?= debug

BUILD_DIR  := build
TARGET_DIR := target/i686-none/$(MODE)
KERNEL     := $(TARGET_DIR)/quiet
ISO_DIR    := $(BUILD_DIR)/isodir
ISO        := $(BUILD_DIR)/quiet.iso
GRUBCFG    := grub.cfg
QEMU       := qemu-system-i386
RUSTC      := cargo +nightly
TARGET     := arch/i686-none.json
RUST_FLAGS := -Z build-std=core,compiler_builtins --target $(TARGET)

GRUB_FILE := $(shell command -v grub-file 2>/dev/null || command -v i686-elf-grub-file 2>/dev/null)
GRUB_MKRESCUE := $(shell command -v grub-mkrescue 2>/dev/null || command -v i686-elf-grub-mkrescue 2>/dev/null)


ifeq ($(MODE), release)
	BUILD_FLAGS := --release
else
	BUILD_FLAGS :=
endif

SRC := $(shell find src -name '*.rs')

CYAN := \033[38;5;217m
ORANGE := \033[38;5;215m


.PHONY: all clean run iso rust re reb draw

all: run

check:
	$(RUSTC) check 

clippy:
	$(RUSTC) clippy --all-features -- -D warnings

fmt:
	$(RUSTC) fmt --all -- --check

test:
	$(RUSTC) nextest run 

v: check clippy fmt

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
	$(RUSTC) build $(RUST_FLAGS) $(BUILD_FLAGS)

iso: $(ISO)

$(ISO): $(KERNEL) $(GRUBCFG)
	@echo "==> Creating ISO..."
	mkdir -p $(ISO_DIR)/boot/grub
	cp $(KERNEL) $(ISO_DIR)/boot/quietOS
	cp $(GRUBCFG) $(ISO_DIR)/boot/grub/grub.cfg

	@if [ -n "$(GRUB_FILE)" ]; then \
		echo "==> Checking multiboot header with $(GRUB_FILE)"; \
		$(GRUB_FILE) --is-x86-multiboot $(ISO_DIR)/boot/quietOS; \
	else \
		echo "==> grub-file not found, skipping multiboot check"; \
	fi

	@if [ -z "$(GRUB_MKRESCUE)" ]; then \
		echo "ERROR: grub-mkrescue not found"; \
		exit 1; \
	fi

	$(GRUB_MKRESCUE) --compress=xz -o $(ISO) $(ISO_DIR) \
		--modules="normal multiboot part_msdos ext2"

	@echo "ISO created: $(ISO)"

run: $(ISO) draw
	$(QEMU) -cdrom $(ISO) -m 512M

release:
	$(MAKE) MODE=release

clean:
	rm -rf $(BUILD_DIR)
	$(RUSTC) clean

# Rebuild and run
re: clean run

reb: clean iso