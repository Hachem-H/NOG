#![no_std]
#![no_main]

use kernel::*;
use spin::Mutex;

const BOARD_X: usize = 32;
const BOARD_Y: usize = 7;

lazy_static::lazy_static! {
    static ref APPLICATION: Mutex<Application> = Mutex::new(Application::new());
}

#[allow(dead_code)]
struct Application {
    pub board: [char; 3 * 3],
    pub current_player: u8,
    pub running: bool,
}

impl Application {
    fn new() -> Application {
        Application {
            board: [' '; 3 * 3],
            current_player: 0,
            running: true,
        }
    }

    #[rustfmt::skip]
    fn draw_ui(&self) {
        kernel::WRITER.lock().write_str(" _______ __        _______              _______              ", 8, 0,);
        kernel::WRITER.lock().write_str("|_     _|__|.----.|_     _|.---.-.----.|_     _|.-----.-----.", 8, 1,);
        kernel::WRITER.lock().write_str("  |   | |  ||  __|  |   |  |  _  |  __|  |   |  |  _  |  -__|", 8, 2,);
        kernel::WRITER.lock().write_str("  |___| |__||____|  |___|  |___._|____|  |___|  |_____|_____|", 8, 3,);
        kernel::WRITER.lock().write_str("                   === By Hachem H. ===                      ", 8, 4,);
        kernel::WRITER.lock().write_str("NOG collection @ https://github.com/hh-Naram/NOG", 0, 24);

        kernel::WRITER.lock().write_str("Current Player: ", 1, 12);
        kernel::WRITER.lock().write_str("To specify where you ", 50, 7);
        kernel::WRITER.lock().write_str("want to write your choice,", 50, 8);
        kernel::WRITER.lock().write_str("insert the coordinates", 50, 9);
        kernel::WRITER.lock().write_str("in the form: x, y", 50, 10);
        kernel::WRITER.lock().write_str("  - with x/y between 1-3", 50, 10);

        kernel::WRITER.lock().write_char(if self.current_player % 2 == 0 { 'x' } else { 'o' }, 17, 12);
    }

    fn draw_board(&self) {
        for i in 0..11 {
            if i == 3 || i == 7 {
                kernel::WRITER
                    .lock()
                    .write_str("---+---+---", BOARD_X, BOARD_Y + i);
            } else {
                kernel::WRITER
                    .lock()
                    .write_str("   |   |   ", BOARD_X, BOARD_Y + i);
            }
        }

        let mut y = BOARD_Y + 1;
        let mut x = BOARD_X + 1;

        let mut index = 0;
        for _ in 0..3 {
            kernel::WRITER.lock().write_char(self.board[index], x, y);
            x += 4;
            index += 1;

            kernel::WRITER.lock().write_char(self.board[index], x, y);
            x += 4;
            index += 1;

            kernel::WRITER.lock().write_char(self.board[index], x, y);
            x -= 8;
            index += 1;

            y += 4;
        }

        kernel::WRITER.lock().write_char('_', x + 3, y + 1);
        kernel::WRITER.lock().write_char('_', x + 5, y + 1);
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    kernel::init();

    unsafe {
        kernel::KEY_CALLBACK = spin::Mutex::new(|_| {
            APPLICATION.lock().current_player += 1;
        });

        kernel::CLOCK_CALLBACK = spin::Mutex::new(|| {
            APPLICATION.lock().draw_ui();
            APPLICATION.lock().draw_board();
        });
    }

    kernel::hlt();
}

#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    kernel::WRITER
        .lock()
        .set_color(ColorCode::new(Color::White, Color::Red));
    kernel::print!("{}", info);
    kernel::hlt();
}
