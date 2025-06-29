#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

mod hypervisor;
mod memory;
mod process;
mod capability;
mod io;

use core::panic::PanicInfo;

/// Entry point for the RustOS kernel
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("RustOS Kernel Starting...");
    
    // Initialize hypervisor integration
    hypervisor::init();
    
    // Initialize memory management
    memory::init();
    
    // Initialize capability system
    capability::init();
    
    // Initialize process management
    process::init();
    
    // Initialize I/O subsystem
    io::init();
    
    println!("RustOS Kernel Initialized Successfully");
    
    // Start the main kernel loop
    kernel_main();
}

/// Main kernel loop
fn kernel_main() -> ! {
    loop {
        // Kernel main loop - process capabilities, manage resources, etc.
        x86_64::instructions::hlt();
    }
}

/// Simple print macro for kernel output
#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        // TODO: Implement proper console output
        // For now, this is a placeholder
    };
}

/// Panic handler for the kernel
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // TODO: Implement proper panic handling with logging
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}