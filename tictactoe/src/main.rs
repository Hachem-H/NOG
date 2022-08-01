#![no_std]
#![no_main]

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xB8000 as *mut u8;

    for (i, &byte) in b"Kernel test".iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xF;
        }
    }

    loop {}
}

#[panic_handler]
pub fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
