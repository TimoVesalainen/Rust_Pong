extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{FPoint, FRect};
use std::time::Duration;

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

    let clear_color =Color::BLACK;
    canvas.set_draw_color(clear_color);
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    let mut padel_x: f32 = 400.0;
    let padel_width: f32 = 50.0;

    let padel_y: f32 = 500.0;
    let padel_height: f32 = 10.0;
    let padel_color = Color::WHITE;

    let mut ball_x: f32 = 400.0;
    let mut ball_y: f32 = 300.0;
    let ball_size: f32 = 10.0;
    let mut ball_dx : f32 = 10.0;
    let mut ball_dy : f32 = -10.0;

    let mut padel_rect : FRect;

    let mut next_ball_x;
    let mut next_ball_y;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    padel_x -= 10.0;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    padel_x += 10.0;
                },
                _ => {}
            }
        }

        if ball_x < 0.0 {
            ball_dx = 10.0;
        } else if ball_x > 800.0 {
            ball_dx = -10.0;
        }

        if ball_y < 0.0 {
            ball_dy = 10.0;
        } else if ball_y > 600.0 {
            break 'running;
        }

        next_ball_y = ball_y + ball_dy;
        next_ball_x = ball_x + ball_dx;

        padel_rect = FRect::new(padel_x, padel_y, padel_width, padel_height);

        let intersection = padel_rect.intersect_line(
            FPoint::new(ball_x, ball_y),
             FPoint::new(next_ball_x, next_ball_y));

        match intersection {
            Some((first, second)) => {
                let left_collision = first.x == padel_x;
                let right_collision = first.x == padel_x + padel_width;
                let top_collision = first.y == padel_y;
                let bottom_collision =  first.y == padel_y + padel_height;

                if left_collision || right_collision {
                    ball_dx = if left_collision{ -10.0 } else { 10.0 };

                    let x_diff = next_ball_x - first.x;
                    next_ball_x -= 2.0 * x_diff;
                }

                if top_collision || bottom_collision {
                    ball_dy = if top_collision { -10.0 } else { 10.0 };

                    let y_diff = next_ball_y - first.y;
                    next_ball_y -= 2.0 * y_diff;
                }
            },
            None => {}              
        }

        canvas.set_draw_color(clear_color);
        canvas.clear();
        canvas.set_draw_color(padel_color);
        canvas.fill_frect(FRect::new(next_ball_x - ball_size, next_ball_y - ball_size, ball_size * 2.0, ball_size * 2.0))?;
        canvas.fill_frect(padel_rect)?;
        canvas.present();
        ball_y = next_ball_y;
        ball_x = next_ball_x;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
