use raylib::{ffi::{CheckCollisionCircleRec, ColorAlpha, GetRandomValue, Rectangle}, prelude::*};
use::rand;

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

struct Paddle {
    x: i32,
    y: i32,
    speed: i32,
    width: i32,
    height: i32,
    color: Color,

}

struct CPUPaddle {
    base: Paddle,
    ball_dist_thres: i32,
}

impl Ball {
    fn new() -> Self {
        return Self {
            x: WIDTH / 2 as i32,
            y: HEIGHT / 2 as i32,
            speed_x: 5,
            speed_y: 5,
            radius: 20.0,
            color: Color::YELLOW,
        };
    }

    fn update(&mut self, player_score: &mut i32, cpu_score: &mut i32) {
        let r = self.radius as i32;
        if self.x + r > WIDTH {
            *cpu_score += 1;
            self.reset();
        } 
        if self.x - r < 0 {
            *player_score += 1;
            self.reset();
        }

        if self.y + r > HEIGHT || self.y - r < 0 {
            self.speed_y *= -1;
        }

        self.x += self.speed_x;
        self.y += self.speed_y;
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle(self.x, self.y, self.radius, self.color);
    }


    fn reset(&mut self) {
        self.x = WIDTH / 2 as i32;
        self.y = HEIGHT / 2 as i32;

        let x_random = rand::random_range(-1..=1);
        let y_random: i32 = rand::random_range(-1..=1);
        let mut x_factor = 1;
        let mut y_factor = 1;
        if x_random < 0 {
            x_factor = -1;
        }

        if y_random < 0 {
            y_factor = -1;
        }
        self.speed_x *= x_factor;
        self.speed_y *= y_factor;
    }
}

impl Paddle {
    fn new(x:i32, y:i32) -> Self {
        return Self{
            x: x,
            y: y,
            speed: 6,
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

impl CPUPaddle {
    fn new(x:i32, y:i32) -> Self {
        return Self{
            base: Paddle::new(x, y),
            ball_dist_thres: 5,
        };
    }

    fn update(&mut self, ball:&Ball) {
        let y_center = self.base.y + (self.base.height / 2 as i32);

        if (y_center - ball.y > self.ball_dist_thres) && (self.base.y - self.base.speed > 0) {
            self.base.y -= self.base.speed;
        }

        if (ball.y - y_center > self.ball_dist_thres) && (self.base.y + self.base.height + self.base.speed) < HEIGHT {
            self.base.y += self.base.speed;
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        self.base.draw(d);
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Ping Pong")
        .build();
    rl.set_target_fps(FPS);

    let mut ball = Ball::new();
    let mut player_paddle = Paddle::new(WIDTH-30, HEIGHT / 2 - 60);
    let mut cpu_paddle = CPUPaddle::new(10, HEIGHT / 2 - 60);

    let mut player_score = 0;
    let mut cpu_score = 0;

    while !rl.window_should_close() {
        
        // Updating the ball position
        ball.update(&mut player_score, &mut cpu_score);

        // Updating the board positions
        player_paddle.update(&rl);
        cpu_paddle.update(&ball);

        let mut d = rl.begin_drawing(&thread);

        // Checking for collisions
        unsafe {
            if CheckCollisionCircleRec(
                raylib::ffi::Vector2{x: ball.x as f32, y: ball.y as f32},
                ball.radius, 
                Rectangle{x: player_paddle.x as f32, y: player_paddle.y as f32, width: player_paddle.width as f32, height: player_paddle.height as f32}
            ) {
                ball.speed_x *= -1;
            }

            if CheckCollisionCircleRec(
                raylib::ffi::Vector2{x: ball.x as f32, y: ball.y as f32},
                ball.radius, 
                Rectangle{x: cpu_paddle.base.x as f32, y: cpu_paddle.base.y as f32, width: cpu_paddle.base.width as f32, height: cpu_paddle.base.height as f32}
            ) {
                ball.speed_x *= -1;
            }
        }
        
        // Draws
        d.clear_background(Color::BLACK.alpha(0.5));
        d.draw_line(WIDTH / 2 as i32, 0, WIDTH / 2 as i32, HEIGHT, Color::WHITE);
        d.draw_rectangle(WIDTH/2 as i32, 0, WIDTH/2, HEIGHT, Color::BLUEVIOLET);
        ball.draw(&mut d);
        player_paddle.draw(&mut d);
        cpu_paddle.draw(&mut d);

        d.draw_text(cpu_score.to_string().as_str(), (WIDTH / 4 as i32) - 50, 0, 30, Color::ORANGE);
        d.draw_text(player_score.to_string().as_str(), (3*WIDTH / 4 as i32) + 50, 0, 30, Color::CHOCOLATE);
    }
}