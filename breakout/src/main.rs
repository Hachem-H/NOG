#![no_std]
#![no_main]

use kernel::*;
use pc_keyboard::KeyCode;
use spin::Mutex;

const PADDLE_WIDTH: isize = 10;
const PADDLE_Y: isize = kernel::BUFFER_HEIGHT as isize - 3;

lazy_static::lazy_static! {
    static ref APPLICATION: Mutex<Application> = Mutex::new(Application::new());
}

struct Vec2 {
    x: isize,
    y: isize,
}

#[derive(Copy, Clone)]
struct Block {
    x: isize,
    y: isize,
    width: isize,
    height: isize,
    hit: bool,
}

struct Application {
    ball_pos: Vec2,
    ball_vel: Vec2,
    paddle_pos: Vec2,
    blocks: [Block; 4 * 9],
    score: u8,
}

impl Application {
    fn new() -> Application {
        let mut blocks: [Block; 4 * 9] = [Block {
            x: 0,
            y: 0,
            width: 8,
            height: 2,
            hit: false,
        }; 4 * 9];

        for x in 0..9 {
            for y in 0..4 {
                let mut block = &mut blocks[x + y * 9];
                block.x = (block.width + 1) * x as isize;
                block.y = (block.height + 1) * y as isize;
            }
        }

        Application {
            ball_pos: Vec2 {
                x: (kernel::BUFFER_WIDTH as isize - PADDLE_WIDTH) / 2,
                y: PADDLE_Y - 2,
            },
            ball_vel: Vec2 { x: 1, y: 1 },
            blocks: blocks,
            score: 0,
            paddle_pos: Vec2 {
                x: (kernel::BUFFER_WIDTH as isize - PADDLE_WIDTH) / 2,
                y: PADDLE_Y,
            },
        }
    }

    // NOTE(Hachem): there is no to string cause its in the std, so I have to do this
    //               God help me...
    fn score_to_text(&self) -> &str {
        match self.score {
            0 => "0",
            1 => "1",
            2 => "2",
            3 => "3",
            4 => "4",
            5 => "5",
            6 => "6",
            7 => "7",
            8 => "8",
            9 => "9",
            10 => "10",
            11 => "11",
            12 => "12",
            13 => "13",
            14 => "14",
            15 => "15",
            16 => "16",
            17 => "17",
            18 => "18",
            19 => "19",
            20 => "20",
            21 => "21",
            22 => "22",
            23 => "23",
            24 => "24",
            25 => "25",
            26 => "26",
            27 => "27",
            28 => "28",
            29 => "29",
            30 => "20",
            30 => "30",
            31 => "31",
            32 => "32",
            33 => "33",
            34 => "34",
            35 => "35",
            36 => "36",
            _ => " ",
        }
    }

    fn update(&mut self) {
        self.ball_pos.x += self.ball_vel.x;
        self.ball_pos.y += self.ball_vel.y;

        if self.ball_pos.x <= 0 || self.ball_pos.x >= (kernel::BUFFER_WIDTH - 1).try_into().unwrap()
        {
            self.ball_vel.x *= -1;
        } else if self.ball_pos.y <= 0
            || self.ball_pos.y >= (kernel::BUFFER_HEIGHT - 1).try_into().unwrap()
        {
            self.ball_vel.y *= -1;
        }

        if self.ball_pos.y == PADDLE_Y {
            for i in 0..PADDLE_WIDTH {
                if self.ball_pos.x == self.paddle_pos.x + i {
                    self.ball_vel.y *= -1;
                }
            }
        }

        for block in &mut self.blocks {
            if !block.hit {
                if self.ball_pos.x >= block.x
                    && self.ball_pos.x <= block.x + block.width
                    && self.ball_pos.y >= block.y
                    && self.ball_pos.y <= block.y + block.height
                {
                    self.ball_vel.x *= -1;
                    self.ball_vel.y *= -1;
                    self.score += 1;
                    block.hit = true;
                }
            }
        }
    }
    
    #[rustfmt::skip]
    fn draw_win(&self, y: usize) {
        kernel::WRITER.lock().write_str("|-----------------------------------------------------------|", 8, y+0);
        kernel::WRITER.lock().write_str("|                                                           |", 8, y+1);
        kernel::WRITER.lock().write_str("|                      You won the game!                    |", 8, y+2);
        kernel::WRITER.lock().write_str("|                                                           |", 8, y+3);
        kernel::WRITER.lock().write_str("|-----------------------------------------------------------|", 8, y+4);
    }

    fn render(&mut self) {
        let writer = &mut kernel::WRITER.lock();
        writer.set_color(kernel::ColorCode::new(
            kernel::Color::White,
            kernel::Color::Black,
        ));
        writer.clear();

        writer.write_char(
            b'\xFE' as char,
            self.ball_pos.x as usize,
            self.ball_pos.y as usize,
        );

        writer.write_str(self.score_to_text(), 0, kernel::BUFFER_HEIGHT - 1);

        for i in 0..PADDLE_WIDTH {
            writer.write_char(
                b'\xC4' as char,
                (self.paddle_pos.x + i) as usize,
                self.paddle_pos.y as usize,
            );
        }

        writer.set_color(kernel::ColorCode::new(
            kernel::Color::LightRed,
            kernel::Color::LightRed,
        ));

        for block in &self.blocks {
            if !block.hit {
                for x in 0..block.width {
                    for y in 0..block.height {
                        writer.write_char(' ', (block.x + x) as usize, (block.y + y) as usize);
                    }
                }
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() {
    kernel::init();

    unsafe {
        kernel::KEY_CALLBACK = Mutex::new(|_character, key| {
            let application = &mut APPLICATION.lock();
            match key {
                KeyCode::ArrowLeft => {
                    if application.paddle_pos.x > 0 {
                        application.paddle_pos.x -= 1;
                    }
                }
                KeyCode::ArrowRight => {
                    if application.paddle_pos.x + PADDLE_WIDTH < kernel::BUFFER_WIDTH as isize {
                        application.paddle_pos.x += 1;
                    }
                }
                _ => {}
            }
        });

        kernel::CLOCK_CALLBACK = Mutex::new(|| {
            let application = &mut APPLICATION.lock();
            application.render();
            application.update();

            if application.score == 36 {
                kernel::WRITER.lock().set_color(kernel::ColorCode::new(
                    kernel::Color::White,
                    kernel::Color::Black,
                ));
                kernel::WRITER.lock().clear();
                application.draw_win(10);
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
