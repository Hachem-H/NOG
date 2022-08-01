#![no_std]
#![no_main]

use kernel;

const BOARD_X: usize = 32;
const BOARD_Y: usize = 7;

#[rustfmt::skip]
fn draw_ui() {
    kernel::WRITER.lock().write( " _______ __        _______              _______              ", 8, 0,);
    kernel::WRITER.lock().write( "|_     _|__|.----.|_     _|.---.-.----.|_     _|.-----.-----.", 8, 1,);
    kernel::WRITER.lock().write( "  |   | |  ||  __|  |   |  |  _  |  __|  |   |  |  _  |  -__|", 8, 2,);
    kernel::WRITER.lock().write( "  |___| |__||____|  |___|  |___._|____|  |___|  |_____|_____|", 8, 3,);
    kernel::WRITER.lock().write( "                   === By Hachem H. ===                      ", 8, 4,);
    kernel::WRITER.lock().write("NOG collection @ https://github.com/hh-Naram/NOG", 0, 24);

    kernel::WRITER.lock().write("Current Player: ", 1, 12);
    kernel::WRITER.lock().write("To specify where you ", 50, 7);
    kernel::WRITER.lock().write("want to write your choice,", 50, 8);
    kernel::WRITER.lock().write("insert the coordinates", 50, 9);
    kernel::WRITER.lock().write("in the form: x, y", 50, 10);
    kernel::WRITER.lock().write("  - with x/y between 1-3", 50, 10);
}

fn draw_board() {
    for i in 0..11 {
        if i == 3 || i == 7 {
            kernel::WRITER
                .lock()
                .write("---+---+---", BOARD_X, BOARD_Y + i);
        } else {
            kernel::WRITER
                .lock()
                .write("   |   |   ", BOARD_X, BOARD_Y + i);
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    draw_ui();
    draw_board();

    loop {}
}

#[panic_handler]
pub fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
