# Rust OS

A minimal operating system kernel written in Rust, following Philipp Oppermann's excellent [Writing an OS in Rust](https://os.phil-opp.com/) blog series.

## Features Implemented

Based on the blog series chapters:

- [x] **Freestanding Rust Binary** - A bare-metal executable without the standard library
- [x] **Minimal Rust Kernel** - Basic kernel setup with custom target
- [x] **VGA Text Mode** - Colored text output to the screen
- [x] **Testing** - Custom test framework for kernel code
- [ ] **CPU Exceptions** - Handling CPU exceptions with Interrupt Descriptor Table (IDT)
- [ ] **Double Faults** - Special handling for double fault exceptions
- [ ] **Hardware Interrupts** - Timer interrupts and keyboard input
- [ ] **Introduction to Paging** - Understanding virtual memory
- [ ] **Paging Implementation** - Setting up page tables
- [ ] **Heap Allocation** - Dynamic memory allocation
- [ ] **Allocator Designs** - Bump, linked list, and fixed-size block allocators
- [ ] **Async/Await** - Cooperative multitasking
- [ ] **Async Keyboard Input** - Non-blocking keyboard handling
- [ ] **Multitasking** - Task scheduling with async executors

## Project Structure

```
rust-os/
├── src/
│   ├── main.rs           # Kernel entry point
│   ├── vga_buffer.rs     # VGA text mode driver
│   └── serial.rs         # Serial port output for testing
├── .cargo/
│   └── config.toml       # Build configuration
├── x86_64-rust_os.json   # Custom target specification
└── Cargo.toml            # Dependencies and build settings
```

## Building

### Prerequisites

- Rust nightly toolchain
- QEMU (for running the OS)
- bootimage tool

```bash
# Install required tools
rustup component add rust-src llvm-tools-preview
cargo install bootimage
```

### Build and Run

```bash
# Build the kernel
cargo build

# Create a bootable disk image and run in QEMU
cargo run

# Run tests
cargo test
```

## Dependencies

- `bootloader` - Bootloader for loading the kernel
- `volatile` - Volatile memory access (for VGA buffer)
- `spin` - Spinlock implementation for no_std
- `x86_64` - x86_64 specific functions and data structures
- `uart_16550` - Serial port driver
- `pic8259` - Programmable Interrupt Controller driver
- `pc-keyboard` - PS/2 keyboard driver
- `lazy_static` - Lazy static initialization

## Learning Resources

This project follows the structure and concepts from:

- **Blog Series**: https://os.phil-opp.com/
- **Uncle Scientist YouTube Series**: [Writing an OS in Rust](https://www.youtube.com/playlist?list=PLHh55M_Kq4OCpPWYgORwgjYMC1jTIJLZ2)
- **Rust OS Dev Community**: https://rust-osdev.com/

## License

This project is created for educational purposes, following the open-source examples from the Writing an OS in Rust blog.

## Next Steps

1. Add CPU exception handling with IDT
2. Implement double fault handler with separate stack
3. Add hardware interrupt support (timer, keyboard)
4. Implement memory paging
5. Add heap allocation with different allocator strategies
6. Implement async/await for cooperative multitasking
