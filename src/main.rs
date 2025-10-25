extern crate sdl2;

use core::{f32, panic};
use rand::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{FPoint, FRect};
use std::time::Duration;

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

fn collide(rect: &FRect, ball: &mut Ball) {
    match rect.intersect_line(ball.location, ball.next_location) {
        Some((first, _second)) => {
            let ball_left_collision = ball.location.x <= rect.left() + 1.0;
            let ball_right_collision = ball.location.x >= rect.right() - 1.0;
            let ball_top_collision = ball.location.y <= rect.top() + 1.0;
            let ball_bottom_collision = ball.location.y >= rect.bottom() - 1.0;

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

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let clear_color = Color::BLACK;
    canvas.set_draw_color(clear_color);
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    let mut padel_x: f32 = 0.0;
    let padel_width: f32 = 800.0;

    let padel_y: f32 = 500.0;
    let padel_height: f32 = 10.0;
    let padel_color = Color::WHITE;

    let ball_count = 50;
    let mut balls: Vec<Ball> = Vec::with_capacity(ball_count);

    let mut rng = rand::rng();
    for _i in 1..ball_count {
        let x = rng.random_range(50.0..750.0);
        let y = rng.random_range(0.0..300.0);
        let speed = rng.random_range(0.5..20.0);

        let angle: f32 = rng.random_range(0.0..f32::consts::TAU);
        let (x_dir, y_dir) = angle.sin_cos();
        let direction = FPoint::new(x_dir, y_dir);

        let ball = Ball {
            location: FPoint::new(x, y),
            next_location: FPoint::new(x, y),
            speed: direction * speed,
            size: 10.0,
        };
        balls.push(ball);
    }

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    padel_x -= 10.0;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    padel_x += 10.0;
                }
                _ => {}
            }
        }
        let padel_rect = FRect::new(padel_x, padel_y, padel_width, padel_height);

        for ball in &mut balls {
            if ball.location.x < 0.0 || ball.location.x > 800.0 {
                ball.speed.x *= -1.0;
            }

            if ball.location.y < 0.0 || ball.location.y > 600.0 {
                ball.speed.y *= -1.0;
            }

            ball.make_next_location();

            collide(&padel_rect, ball);
            ball.move_to_next();
        }

        canvas.set_draw_color(clear_color);
        canvas.clear();
        canvas.set_draw_color(padel_color);
        for ball in &mut balls {
            let ball_rect = ball.to_rect();
            canvas.fill_frect(ball_rect)?;
        }
        canvas.set_draw_color(padel_color);
        canvas.fill_frect(padel_rect)?;
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
