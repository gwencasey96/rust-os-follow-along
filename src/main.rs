//! # Rust OS
//!
//! A minimal operating system kernel following Philipp Oppermann's
//! "Writing an OS in Rust" blog series (https://os.phil-opp.com/)
//!
//! This project implements:
//! - Freestanding Rust binary
//! - Minimal Rust kernel
//! - VGA text mode output
//! - Testing framework
//! - CPU exceptions and double faults
//! - Hardware interrupts
//! - Memory paging
//! - Heap allocation
//! - Async/await support

#![no_std] // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use spin::Mutex;
use rust_sys_lib::ring_buffer::RingBuffer;
use rust_sys_lib::bump_allocator::BumpAllocator;

mod vga_buffer;
mod serial;

/// Keyboard input buffer — holds up to 256 pending keystrokes.
static KEYBOARD_BUFFER: Mutex<RingBuffer<u8, 256>> = Mutex::new(RingBuffer::new());

/// Early-init memory region for the bump allocator (4 KiB).
static mut EARLY_HEAP: [u8; 4096] = [0; 4096];

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

/// The entry point for the kernel
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    // Demonstrate the bump allocator over our static early-heap region.
    let mut early_alloc = unsafe {
        BumpAllocator::new(EARLY_HEAP.as_mut_ptr() as usize, 4096)
    };
    let _block = early_alloc.alloc(64, 8);
    println!("Early heap: {} bytes used, {} remaining",
        early_alloc.used(), early_alloc.remaining());

    // Demonstrate the keyboard buffer.
    {
        let mut buf = KEYBOARD_BUFFER.lock();
        buf.push(b'H');
        buf.push(b'i');
        if let Some(c) = buf.pop() {
            println!("Keyboard buffer first byte: {}", c as char);
        }
    }

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
