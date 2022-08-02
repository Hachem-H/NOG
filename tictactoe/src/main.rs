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

    pub pos_x: usize,
    pub pos_y: usize,
}

impl Application {
    fn new() -> Application {
        Application {
            board: [' '; 3 * 3],
            current_player: 0,

            pos_x: 0,
            pos_y: 0,
        }
    }

    fn get_player(&self) -> char {
        if self.current_player % 2 == 0 {
            'x'
        } else {
            'o'
        }
    }

    fn check_win(&self, player: char) -> bool {
        for i in 0..3 {
            if (self.board[i + 0 * 3] == self.board[i + 1 * 3]
                && self.board[i + 0 * 3] == self.board[i + 2 * 3]
                && self.board[i + 0 * 3] == player)
                || (self.board[0 + i * 3] == self.board[1 + i * 3]
                    && self.board[0 + i * 3] == self.board[2 + i * 3]
                    && self.board[0 + i * 3] == player)
            {
                return true;
            }
        }

        if (self.board[0 + 0 * 3] == self.board[1 + 1 * 3]
            && self.board[0 + 0 * 3] == self.board[2 + 2 * 3]
            && self.board[0 + 0 * 3] == player)
            || (self.board[0 + 2 * 3] == self.board[1 + 1 * 3]
                && self.board[0 + 2 * 3] == self.board[2 + 0 * 3]
                && self.board[0 + 2 * 3] == player)
        {
            return true;
        }

        return false;
    }

    #[rustfmt::skip]
    fn draw_ui(&self) {
        kernel::WRITER.lock().write_str(" _______ __        _______              _______              ", 8, 0);
        kernel::WRITER.lock().write_str("|_     _|__|.----.|_     _|.---.-.----.|_     _|.-----.-----.", 8, 1);
        kernel::WRITER.lock().write_str("  |   | |  ||  __|  |   |  |  _  |  __|  |   |  |  _  |  -__|", 8, 2);
        kernel::WRITER.lock().write_str("  |___| |__||____|  |___|  |___._|____|  |___|  |_____|_____|", 8, 3);
        kernel::WRITER.lock().write_str("                   === By Hachem H. ===                      ", 8, 4);
        kernel::WRITER.lock().write_str("NOG collection @ https://github.com/hh-Naram/NOG", 0, 24);

        kernel::WRITER.lock().write_str("Current Player: ", 1, 12);
        kernel::WRITER.lock().write_str("To specify where you ", 50, 7);
        kernel::WRITER.lock().write_str("want to write your choice,", 50, 8);
        kernel::WRITER.lock().write_str("insert the coordinates", 50, 9);
        kernel::WRITER.lock().write_str("in the form: x, y", 50, 10);
        kernel::WRITER.lock().write_str("  - with x/y between 1-3", 50, 10);

        kernel::WRITER.lock().write_char(self.get_player(), 17, 12);
    }
    
    #[rustfmt::skip]
    fn draw_win(&self, player: char, y: usize) {
        kernel::WRITER.lock().write_str("|-----------------------------------------------------------|", 8, y+0);
        kernel::WRITER.lock().write_str("|                                                           |", 8, y+1);
        kernel::WRITER.lock().write_str("|                        _ Won the game!                    |", 8, y+2);
        kernel::WRITER.lock().write_str("|                                                           |", 8, y+3);
        kernel::WRITER.lock().write_str("|-----------------------------------------------------------|", 8, y+4);

        kernel::WRITER.lock().write_char(player, 33, y+2);
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

        kernel::WRITER.lock().write_char(
            {
                match self.pos_x {
                    1 => '1',
                    2 => '2',
                    3 => '3',
                    _ => '_',
                }
            },
            x + 3,
            y + 1,
        );
        kernel::WRITER.lock().write_char(
            {
                match self.pos_y {
                    1 => '1',
                    2 => '2',
                    3 => '3',
                    _ => '_',
                }
            },
            x + 5,
            y + 1,
        );
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    kernel::init();

    unsafe {
        kernel::KEY_CALLBACK = spin::Mutex::new(|character, _| {
            let application = &mut APPLICATION.lock();

            if application.pos_x == 0 {
                match character {
                    '0' => application.pos_x = 1,
                    '1' => application.pos_x = 2,
                    '2' => application.pos_x = 3,
                    _ => {}
                }
            } else if application.pos_x != 0 && application.pos_y == 0 {
                match character {
                    '0' => application.pos_y = 1,
                    '1' => application.pos_y = 2,
                    '2' => application.pos_y = 3,
                    _ => {}
                }

                let pos_x = application.pos_x.clone() - 1;
                let pos_y = application.pos_y.clone() - 1;

                if application.board[pos_x + pos_y * 3] == ' ' {
                    application.board[pos_x + pos_y * 3] = application.get_player();
                    application.current_player += 1;
                }

                application.pos_x = 0;
                application.pos_y = 0;
            }
        });

        kernel::CLOCK_CALLBACK = spin::Mutex::new(|| {
            let application = &mut APPLICATION.lock();

                application.draw_ui();
                application.draw_board();

            if application.check_win('x') {
                kernel::WRITER.lock().clear();
                application.draw_win('x', 10);
                kernel::hlt();
            } else if application.check_win('o') {
                kernel::WRITER.lock().clear();
                application.draw_win('o', 10);
                kernel::hlt();
            }
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
