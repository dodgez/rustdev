#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::InterruptStackFrame;
use rustdev::serial_print;
use rustdev::{exit_qemu, QemuExitCode, serial_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
  serial_print!("stack_overflow...");

  rustdev::gdt::init();
  init_test_idt();

  stack_overflow();

  panic!("Execution continued after stack overflow");
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
  stack_overflow();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  rustdev::test_panic_handler(info);
}

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(rustdev::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

extern "x86-interrupt" fn test_double_fault_handler(_stack_frame: &mut InterruptStackFrame, _error_code: u64) -> ! {
  serial_println!("[ok]");
  exit_qemu(QemuExitCode::Success);
  loop {}
}

pub fn init_test_idt() {
    TEST_IDT.load();
}
