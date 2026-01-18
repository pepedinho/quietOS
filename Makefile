MODE ?= debug

BUILD_DIR  := build
TARGET_DIR := target/i686-none/$(MODE)
KERNEL     := $(TARGET_DIR)/quiet
ISO_DIR    := $(BUILD_DIR)/isodir
ISO        := $(BUILD_DIR)/quiet.iso
GRUBCFG    := grub.cfg
BENCHGRUB_CFG := bench_grub.cfg
QEMU       := qemu-system-i386
RUSTC      := cargo +nightly
TARGET     := arch/i686-none.json
RUST_FLAGS := -Z build-std=core,compiler_builtins --target $(TARGET)


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
	$(RUSTC) test 

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
	grub-file --is-x86-multiboot $(ISO_DIR)/boot/quietOS
	grub-mkrescue --compress=xz -o $(ISO) $(ISO_DIR) --modules="normal multiboot part_msdos ext2"
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

bench:
	@echo "$(CYAN)==> Building kernel with benchmark feature...$(RESET)"
	$(RUSTC) build $(RUST_FLAGS) $(BUILD_FLAGS) --features bench

	@echo "$(CYAN)==> Creating ISO with benchmark mode...$(RESET)"
	mkdir -p $(ISO_DIR)/boot/grub
	cp $(KERNEL) $(ISO_DIR)/boot/quietOS
	cp $(BENCHGRUB_CFG) $(ISO_DIR)/boot/grub/grub.cfg
	grub-mkrescue --compress=xz -o $(ISO) $(ISO_DIR) --modules="normal multiboot part_msdos ext2"

	@echo "$(CYAN)==> Running QEMU and logging output...$(RESET)"
	mkdir -p logs
	$(QEMU) -cdrom $(ISO) -m 512M -display curses | tee logs/bench.log

	@echo "$(CYAN)==> Benchmark results saved to logs/bench.log$(RESET)"
