extern crate sdl2;

use core::f32;
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
    direction: FPoint,
    speed: f32,
    colliding_left: bool,
    colliding_right: bool,
    colliding_top: bool,
    colliding_bottom: bool,
}

impl Ball {
    fn make_next_location(&mut self) {
        self.next_location.x = self.location.x + self.direction.x * self.speed;
        self.next_location.y = self.location.y + self.direction.y * self.speed;
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
            direction: direction,
            size: 10.0,
            speed: speed,
            colliding_top: false,
            colliding_bottom: false,
            colliding_left: false,
            colliding_right: false,
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
        let mut left_collision = false;
        let mut right_collision = false;
        let mut top_collision = false;
        let mut bottom_collision = false;
        let padel_rect = FRect::new(padel_x, padel_y, padel_width, padel_height);

        for ball in &mut balls {
            if ball.location.x < 0.0 || ball.location.x > 800.0 {
                ball.direction.x *= -1.0;
            }

            if ball.location.y < 0.0 || ball.location.y > 600.0 {
                ball.direction.y *= -1.0;
            }

            ball.make_next_location();

            let intersection = padel_rect.intersect_line(ball.location, ball.next_location);

            match intersection {
                Some((first, second)) => {
                    let ball_left_collision = (first.x - padel_x).abs() < 0.05;
                    let ball_right_collision = (first.x - (padel_x + padel_width)).abs() < 0.05;
                    let ball_top_collision = (first.y - padel_y).abs() < 0.05;
                    let ball_bottom_collision = (first.y - (padel_y + padel_height)).abs() < 0.05;

                    if ball_left_collision || ball_right_collision {
                        ball.direction.x *= -1.0;

                        let x_diff = ball.next_location.x - first.x;
                        ball.next_location.x -= 2.0 * x_diff;
                    }

                    if ball_top_collision || ball_bottom_collision {
                        ball.direction.y *= -1.0;

                        let y_diff = ball.next_location.y - first.y;
                        ball.next_location.y -= 2.0 * y_diff;
                    }

                    ball.colliding_top = ball_bottom_collision;
                    ball.colliding_bottom = ball_top_collision;
                    ball.colliding_right = ball_left_collision;
                    ball.colliding_left = ball_right_collision;
                    left_collision |= ball_left_collision;
                    right_collision |= ball_right_collision;
                    top_collision |= ball_top_collision;
                    bottom_collision |= ball_bottom_collision;
                }
                None => {
                    ball.colliding_bottom = false;
                    ball.colliding_top = false;
                    ball.colliding_left = false;
                    ball.colliding_right = false;
                }
            }
            ball.move_to_next();
        }

        canvas.set_draw_color(clear_color);
        canvas.clear();
        for ball in &mut balls {
            canvas.set_draw_color(padel_color);
            let ball_rect = ball.to_rect();
            canvas.fill_frect(ball_rect)?;
            canvas.set_draw_color(Color::RED);
            if ball.colliding_bottom {
                canvas.draw_fline(ball_rect.bottom_left(), ball_rect.bottom_right())?;
            }
            if ball.colliding_top {
                canvas.draw_fline(ball_rect.top_left(), ball_rect.top_right())?;
            }
            if ball.colliding_left {
                canvas.draw_fline(ball_rect.bottom_left(), ball_rect.top_left())?;
            }
            if ball.colliding_right {
                canvas.draw_fline(ball_rect.top_right(), ball_rect.bottom_right())?;
            }
        }
        canvas.set_draw_color(padel_color);
        canvas.fill_frect(padel_rect)?;

        canvas.set_draw_color(Color::RED);
        if left_collision {
            canvas.draw_fline(padel_rect.bottom_left(), padel_rect.top_left())?
        }
        if right_collision {
            canvas.draw_fline(padel_rect.bottom_right(), padel_rect.top_right())?
        }
        if top_collision {
            canvas.draw_fline(padel_rect.top_left(), padel_rect.top_right())?
        }
        if bottom_collision {
            canvas.draw_fline(padel_rect.bottom_left(), padel_rect.bottom_right())?
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
