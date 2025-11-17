use raylib::{ffi::SetTargetFPS, prelude::*};

// Config
const WIDTH: i32 = 800;
const HEIGHT: i32 = 480;
const FPS: u32 = 60;

struct Ball {
    x: i32,
    y: i32,
    speed_x: i32,
    speed_y: i32,
    radius: f32,
    color: Color,
}


impl Ball {
    fn new() -> Self {
        return Self {
            x: WIDTH / 2 as i32,
            y: HEIGHT / 2 as i32,
            speed_x: 5,
            speed_y: 5,
            radius: 20.0,
            color: Color::WHITE,
        };
    }

    fn update(&mut self) {
        let r = self.radius as i32;
        if self.x + r >= WIDTH || self.x - r <= 0 {
            self.speed_x *= -1;
        }

        if self.y + r >= HEIGHT || self.y - r < 0 {
            self.speed_y *= -1;
        }

        self.x += self.speed_x;
        self.y += self.speed_y;
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle(self.x, self.y, self.radius, self.color);
    }
}


struct Board {
    x: i32,
    y: i32,
    speed: i32,
    width: i32,
    height: i32,
    color: Color,
}


impl Board {
    fn new(x:i32, y:i32) -> Self {
        return Self{
            x: x,
            y: y,
            speed: 5,
            width: 20,
            height: 120,
            color: Color::WHITE,
        };
    }

    fn update(&mut self, rl: &RaylibHandle) {
        if rl.is_key_down(KeyboardKey::KEY_UP) && (self.y - self.speed > 0) {
            self.y -= self.speed;
        }

        if rl.is_key_down(KeyboardKey::KEY_DOWN) && (self.y + self.height + self.speed) < HEIGHT {
            self.y += self.speed;
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle(self.x, self.y,self.width, self.height, self.color);
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Ping Pong")
        .build();
    rl.set_target_fps(FPS);

    let mut ball = Ball::new();
    let mut board_left = Board::new(10, HEIGHT / 2 - 60);
    let mut board_right = Board::new(WIDTH-30, HEIGHT / 2 - 60);

    while !rl.window_should_close() {
        
        // Updating the ball position
        ball.update();

        // Updating the board positions
        board_left.update(&rl);
        board_right.update(&rl);

        let mut d = rl.begin_drawing(&thread);
        
        // Drawing ball and boards
        ball.draw(&mut d);
        board_left.draw(&mut d);
        board_right.draw(&mut d);

        d.draw_line(WIDTH / 2 as i32, 0, WIDTH / 2 as i32, HEIGHT, Color::WHITE);
        d.clear_background(Color::BLACK);
    }
}
