extern crate sdl2;

use core::{f32, panic};
use rand::prelude::*;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{FPoint, FRect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
struct Ball {
    location: FPoint,
    size: f32,
    next_location: FPoint,
    speed: FPoint,
}

impl Ball {
    fn make_next_location(&mut self) {
        self.next_location.x = self.location.x + self.speed.x;
        self.next_location.y = self.location.y + self.speed.y;
    }

    fn move_to_next(&mut self) {
        self.location.x = self.next_location.x;
        self.location.y = self.next_location.y;
    }

    fn to_rect(&self) -> FRect {
        FRect::new(
            self.location.x - self.size,
            self.location.y - self.size,
            self.size * 2.0,
            self.size * 2.0,
        )
    }
}

#[derive(Debug, Clone)]
struct Rectangle {
    rectangle: FRect,
    next_rectangle: FRect,
    speed: FPoint,
}

impl Rectangle {
    fn make_next_location(&mut self) {
        self.next_rectangle.x = self.rectangle.x + self.speed.x;
        self.next_rectangle.y = self.rectangle.y + self.speed.y;
    }

    fn move_to_next(&mut self) {
        self.rectangle.x = self.next_rectangle.x;
        self.rectangle.y = self.next_rectangle.y;
    }
}

fn collide(rect: &Rectangle, ball: &mut Ball) {
    match rect
        .next_rectangle
        .intersect_line(ball.location, ball.next_location)
    {
        Some((first, _second)) => {
            let ball_left_collision = ball.location.x <= rect.next_rectangle.left() + 1.0;
            let ball_right_collision = ball.location.x >= rect.next_rectangle.right() - 1.0;
            let ball_top_collision = ball.location.y <= rect.next_rectangle.top() + 1.0;
            let ball_bottom_collision = ball.location.y >= rect.next_rectangle.bottom() - 1.0;

            if ball_left_collision || ball_right_collision {
                ball.speed.x *= -1.0;
                ball.next_location.x -= 2.0 * (ball.next_location.x - first.x);
            }

            if ball_top_collision || ball_bottom_collision {
                ball.speed.y *= -1.0;
                ball.next_location.y -= 2.0 * (ball.next_location.y - first.y);
            }

            if !(ball_left_collision
                || ball_right_collision
                || ball_top_collision
                || ball_bottom_collision)
            {
                panic!(
                    "No collision {:?} {:?} {:?}",
                    rect, ball.location, ball.next_location
                )
            }
        }
        None => {}
    }
}

struct Game {
    balls: Vec<Ball>,
    padel: Rectangle,
}

impl Game {
    fn init(ball_count: usize) -> Game {
        let padel_x: f32 = 0.0;
        let padel_width: f32 = 100.0;

        let padel_y: f32 = 500.0;
        let padel_height: f32 = 10.0;

        let mut game = Game {
            balls: Vec::with_capacity(ball_count),
            padel: Rectangle {
                rectangle: FRect::new(padel_x, padel_y, padel_width, padel_height),
                next_rectangle: FRect::new(padel_x, padel_y, padel_width, padel_height),
                speed: FPoint::new(0.0, 0.0),
            },
        };
        let mut rng = rand::rng();
        for _i in 0..ball_count {
            let x = rng.random_range(50.0..750.0);
            let y = rng.random_range(0.0..300.0);
            let speed = rng.random_range(0.5..10.0);

            let angle: f32 = rng.random_range(0.0..f32::consts::TAU);
            let (x_dir, y_dir) = angle.sin_cos();
            let direction = FPoint::new(x_dir, y_dir);

            let ball = Ball {
                location: FPoint::new(x, y),
                next_location: FPoint::new(x, y),
                speed: direction * speed,
                size: 10.0,
            };
            game.balls.push(ball);
        }
        return game;
    }

    fn handle_events(&mut self, event_pump: &mut EventPump) -> bool {
        self.padel.speed = FPoint::new(0.0, 0.0);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    self.padel.speed.x = -10.0;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    self.padel.speed.x = 10.0;
                }
                _ => {}
            }
        }
        return true;
    }

    fn update(&mut self) {
        self.padel.make_next_location();
        for ball in &mut self.balls {
            if ball.location.x < 0.0 || ball.location.x > 800.0 {
                ball.speed.x *= -1.0;
            }

            if ball.location.y < 0.0 || ball.location.y > 600.0 {
                ball.speed.y *= -1.0;
            }

            ball.make_next_location();

            collide(&self.padel, ball);
            ball.move_to_next();
        }
        self.padel.move_to_next();
    }

    fn draw(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(CLEAR_COLOR);
        canvas.clear();
        canvas.set_draw_color(PADEL_COLOR);
        for ball in &mut self.balls {
            let ball_rect = ball.to_rect();
            canvas.fill_frect(ball_rect)?;
        }
        canvas.set_draw_color(PADEL_COLOR);
        canvas.fill_frect(self.padel.rectangle)?;
        canvas.present();
        Ok(())
    }
}

static PADEL_COLOR: Color = Color::WHITE;
static CLEAR_COLOR: Color = Color::BLACK;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Pong", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut game = Game::init(2);

    let fps = 60;
    let default_frame_length = Duration::from_nanos(1_000_000_000u64 / fps);
    'running: loop {
        let frame_start = Instant::now();
        if !game.handle_events(&mut event_pump) {
            break 'running;
        }
        game.update();
        game.draw(&mut canvas)?;

        let frame_end = Instant::now();
        let frame_length = frame_end - frame_start;
        if frame_length < default_frame_length {
            ::std::thread::sleep(default_frame_length - frame_length);
        }
    }

    Ok(())
}
