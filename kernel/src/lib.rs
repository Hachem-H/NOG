#![no_std]
#![feature(const_mut_refs)]
#![feature(abi_x86_interrupt)]

pub mod gdt;
pub mod interrupts;
pub mod writer;

pub use gdt::*;
pub use interrupts::*;
pub use writer::*;

pub fn init() {
    gdt::init();
    interrupts::init_idt();
}
