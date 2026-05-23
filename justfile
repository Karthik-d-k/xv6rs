# Common aliases

alias c := clean
alias g := gdb
alias k := kernel
alias q := qemu
alias qg := qemu-gdb

# Set required variables

NUM_CPUS := "3"
GDBPORT := "1234"
QEMU := "qemu-system-riscv64"
QEMU_OPTS := "-machine virt -bios none -m 128M -nographic"
KERNEL_TARGET := "target/riscv64gc-unknown-none-elf/release/kernel"
TOOL_PREFIX := "$HOME" + "/tools/xpack-riscv-none-elf-gcc-15.2.0-1/bin/riscv-none-elf-"
OBJCOPY := TOOL_PREFIX + "objcopy"
OBJDUMP := TOOL_PREFIX + "objdump"
GDB := TOOL_PREFIX + "gdb"

default: help

# Show available recipes and usage info
help:
    @echo "xv6rs Build System"
    @echo "==================="
    @echo ""
    @echo "Usage: just qemu"
    @echo ""
    @just --list --unsorted

# Build the kernel and emit kernel.asm / kernel.sym
kernel:
    cd kernel && cargo build --release
    {{ OBJDUMP }} -S {{ KERNEL_TARGET }} > kernel/kernel.asm
    {{ OBJDUMP }} -t {{ KERNEL_TARGET }} | sed '1,/SYMBOL TABLE/d; s/ .* / /; /^$/d' > kernel/kernel.sym

# Run QEMU normally
qemu: kernel
    @echo "*** Exit QEMU with Ctrl-A X ***"
    {{ QEMU }} {{ QEMU_OPTS }} -smp {{ NUM_CPUS }} -kernel {{ KERNEL_TARGET }}

# Run QEMU paused, with gdb stub on port {{GDBPORT}}
qemu-gdb: kernel
    @echo "*** Now run 'just gdb' in another terminal ***"
    @echo "*** Exit QEMU with Ctrl-A X ***"
    {{ QEMU }} {{ QEMU_OPTS }} -smp {{ NUM_CPUS }} -kernel {{ KERNEL_TARGET }} -S -gdb tcp::{{ GDBPORT }}

# Launch gdb and auto-connect to qemu-gdb
gdb:
    {{ GDB }} {{ KERNEL_TARGET }} \
        -x gdbinit.riscv \
        -ex "target remote 127.0.0.1:{{ GDBPORT }}"

# Clean output artifacts
clean:
    cargo clean
    rm -f kernel/kernel.asm kernel/kernel.sym
