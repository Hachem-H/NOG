#![no_std]
#![no_main]

use kernel::*;
use pc_keyboard::KeyCode;
use spin::Mutex;

lazy_static::lazy_static! {
    static ref APPLICATION: Mutex<Application> = Mutex::new(Application::new());
}

const RIGHT_PADDLE_X: usize = kernel::BUFFER_WIDTH - 2;
const LEFT_PADDLE_X: usize = 1;
const PADDLE_HEIGHT: usize = 5;

struct Vec2 {
    x: isize,
    y: isize,
}

impl Vec2 {
    fn move_up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }

    fn move_down(&mut self) {
        if <isize as TryInto<usize>>::try_into(self.y).unwrap() + PADDLE_HEIGHT
            < kernel::BUFFER_HEIGHT
        {
            self.y += 1;
        }
    }
}

struct Application {
    right_paddle: Vec2,
    left_paddle: Vec2,
    ball: Vec2,
    ball_vel: Vec2,
    right_score: u8,
    left_score: u8,
}

impl Application {
    fn new() -> Application {
        Application {
            right_paddle: Vec2 {
                x: RIGHT_PADDLE_X as isize,
                y: 10 as isize,
            },

            left_paddle: Vec2 {
                x: LEFT_PADDLE_X as isize,
                y: 10,
            },

            ball: Vec2 {
                x: (kernel::BUFFER_WIDTH / 2) as isize,
                y: (kernel::BUFFER_HEIGHT / 2) as isize,
            },

            ball_vel: Vec2 { x: 1, y: 1 },

            left_score: 0,
            right_score: 0,
        }
    }

    #[rustfmt::skip]
    fn draw_win(&self, player: char, y: usize) {
        kernel::WRITER.lock().write_str("|-----------------------------------------------------------|", 8, y+0);
        kernel::WRITER.lock().write_str("|                                                           |", 8, y+1);
        kernel::WRITER.lock().write_str("|                        _ won the game!                    |", 8, y+2);
        kernel::WRITER.lock().write_str("|                                                           |", 8, y+3);
        kernel::WRITER.lock().write_str("|-----------------------------------------------------------|", 8, y+4);

        kernel::WRITER.lock().write_char(player, 33, y+2);
    }

    fn check_collisions(&mut self) {
        if self.ball.x == LEFT_PADDLE_X.try_into().unwrap() {
            for i in 0..PADDLE_HEIGHT {
                if self.ball.y == self.left_paddle.y + i as isize {
                    self.ball_vel.x *= -1;
                }
            }
        }

        if self.ball.x == RIGHT_PADDLE_X.try_into().unwrap() {
            for i in 0..PADDLE_HEIGHT {
                if self.ball.y == self.right_paddle.y + i as isize {
                    self.ball_vel.x *= -1;
                }
            }
        }
    }

    fn update(&mut self) {
        let writer = &mut kernel::WRITER.lock();
        writer.set_color(kernel::ColorCode::new(
            kernel::Color::White,
            kernel::Color::Black,
        ));
        writer.clear();

        for i in 0..kernel::BUFFER_HEIGHT {
            if i % 2 == 0 {
                writer.write_char('|', kernel::BUFFER_WIDTH / 2, i);
            }
        }

        writer.write_char(
            {
                match self.left_score {
                    0 => '0',
                    1 => '1',
                    2 => '2',
                    3 => '3',
                    4 => '4',
                    5 => '5',
                    6 => '6',
                    7 => '7',
                    8 => '8',
                    9 => '9',
                    _ => '_',
                }
            },
            0,
            0,
        );

        writer.write_char(
            {
                match self.right_score {
                    0 => '0',
                    1 => '1',
                    2 => '2',
                    3 => '3',
                    4 => '4',
                    5 => '5',
                    6 => '6',
                    7 => '7',
                    8 => '8',
                    9 => '9',
                    _ => '_',
                }
            },
            kernel::BUFFER_WIDTH - 1,
            0,
        );

        writer.write_char(
            b'\xFE' as char,
            self.ball.x.try_into().unwrap(),
            self.ball.y.try_into().unwrap(),
        );

        writer.set_color(kernel::ColorCode::new(
            kernel::Color::White,
            kernel::Color::White,
        ));
        for i in 0..PADDLE_HEIGHT {
            writer.write_char(
                ' ',
                <isize as TryInto<usize>>::try_into(self.left_paddle.x).unwrap(),
                <isize as TryInto<usize>>::try_into(self.left_paddle.y).unwrap() + i,
            );
            writer.write_char(
                ' ',
                <isize as TryInto<usize>>::try_into(self.right_paddle.x).unwrap(),
                <isize as TryInto<usize>>::try_into(self.right_paddle.y).unwrap() + i,
            );
        }

        self.ball.x += self.ball_vel.x;
        self.ball.y += self.ball_vel.y;

        self.check_collisions();

        if self.ball.x <= 0 {
            self.ball.x = (kernel::BUFFER_WIDTH / 2) as isize;
            self.ball.y = (kernel::BUFFER_HEIGHT / 2) as isize;
            self.left_score += 1;
        }

        if self.ball.x >= (kernel::BUFFER_WIDTH - 1).try_into().unwrap() {
            self.ball.x = (kernel::BUFFER_WIDTH / 2) as isize;
            self.ball.y = (kernel::BUFFER_HEIGHT / 2) as isize;
            self.left_score += 1;
        }

        if self.ball.y <= 0 || self.ball.y >= (kernel::BUFFER_HEIGHT - 1).try_into().unwrap() {
            self.ball_vel.y *= -1;
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    kernel::init();

    unsafe {
        kernel::KEY_CALLBACK = Mutex::new(|character, key| {
            let application = &mut APPLICATION.lock();
            match key {
                KeyCode::ArrowUp => application.right_paddle.move_up(),
                KeyCode::ArrowDown => application.right_paddle.move_down(),
                _ => {}
            }

            match character {
                'w' => application.left_paddle.move_up(),
                's' => application.left_paddle.move_down(),
                _ => {}
            }
        });
        kernel::CLOCK_CALLBACK = Mutex::new(|| {
            let application = &mut APPLICATION.lock();
            application.update();

            if application.right_score == 10 {
                kernel::WRITER.lock().set_color(kernel::ColorCode::new(
                    kernel::Color::White,
                    kernel::Color::Black,
                ));
                kernel::WRITER.lock().clear();
                application.draw_win('R', 10);
                kernel::hlt();
            }

            if application.left_score == 10 {
                kernel::WRITER.lock().set_color(kernel::ColorCode::new(
                    kernel::Color::White,
                    kernel::Color::Black,
                ));
                kernel::WRITER.lock().clear();
                application.draw_win('L', 10);
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
