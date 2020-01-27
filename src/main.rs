#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustdev::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use rustdev::println;

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    rustdev::init();

    println!("{}", boot_info.physical_memory_offset);

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    rustdev::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rustdev::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustdev::test_panic_handler(info)
}
