extern crate sdl2;

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

    let mut padel_x: f32 = 400.0;
    let padel_width: f32 = 50.0;

    let padel_y: f32 = 500.0;
    let padel_height: f32 = 10.0;
    let padel_color = Color::WHITE;

    let mut ball = Ball {
        location: FPoint::new(400.0, 300.0),
        next_location: FPoint::new(400.0, 300.0),
        direction: FPoint::new(1.0, -1.0),
        size: 10.0,
        speed: 10.0,
    };

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
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    ball.speed += 1.0;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    if ball.speed > 1.0 {
                        ball.speed -= 1.0;
                    }
                }
                _ => {}
            }
        }
        let mut left_collision = false;
        let mut right_collision = false;
        let mut top_collision = false;
        let mut bottom_collision = false;
        let padel_rect = FRect::new(padel_x, padel_y, padel_width, padel_height);

        if ball.location.x < 0.0 {
            ball.direction.x = 1.0;
        } else if ball.location.x > 800.0 {
            ball.direction.x = -1.0;
        }

        if ball.location.y < 0.0 {
            ball.direction.y = 1.0;
        } else if ball.location.y > 600.0 {
            break 'running;
        }

        ball.make_next_location();

        let intersection = padel_rect.intersect_line(ball.location, ball.next_location);

        match intersection {
            Some((first, _second)) => {
                left_collision = (first.x - padel_x).abs() < 0.05;
                right_collision = (first.x - (padel_x + padel_width)).abs() < 0.05;
                top_collision = (first.y - padel_y).abs() < 0.05;
                bottom_collision = (first.y - (padel_y + padel_height)).abs() < 0.05;

                if left_collision || right_collision {
                    ball.direction.x = if left_collision { -1.0 } else { 1.0 };

                    let x_diff = ball.next_location.x - first.x;
                    ball.next_location.x -= 2.0 * x_diff;
                }

                if top_collision || bottom_collision {
                    ball.direction.y = if top_collision { -1.0 } else { 1.0 };

                    let y_diff = ball.next_location.y - first.y;
                    ball.next_location.y -= 2.0 * y_diff;
                }
            }
            None => {}
        }
        ball.move_to_next();

        canvas.set_draw_color(clear_color);
        canvas.clear();
        canvas.set_draw_color(padel_color);
        canvas.fill_frect(ball.to_rect())?;
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
