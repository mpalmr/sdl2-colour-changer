#![warn(rust_2018_idioms)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use std::thread::sleep;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

type RGBValue = [u8; 3];

static WINDOW_WIDTH: u32 = 800;
static WINDOW_HEIGHT: u32 = 600;

fn update_bg(canvas: &mut Canvas<Window>, bg_color: RGBValue) {
    canvas.set_draw_color(Color::RGB(bg_color[0], bg_color[1], bg_color[2]));
    canvas.clear();
    canvas.present();
}

fn run() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("SDL Tutorial", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let mut bg_color: RGBValue = [0, 0, 0];
    update_bg(&mut canvas, bg_color);

    let mut event_pump = sdl_context.event_pump()?;
    let mut bg_color_index = 0;

    'mainloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                // Increases the color value of one colour
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    if bg_color[bg_color_index] < 255 {
                        bg_color[bg_color_index] += 5;
                        update_bg(&mut canvas, bg_color);
                    }
                }

                // Decreases the color value of one colour
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    if bg_color[bg_color_index] > 0 {
                        bg_color[bg_color_index] -= 5;
                        update_bg(&mut canvas, bg_color);
                    }
                }

                // Switches the color to be modified
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    if bg_color_index < 2 {
                        bg_color_index += 1;
                    } else {
                        bg_color_index = 0;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    if bg_color_index > 0 {
                        bg_color_index -= 1;
                    } else {
                        bg_color_index = 2;
                    }
                }

                // Quits the program
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::Quit { .. } => break 'mainloop,

                _ => {}
            }
        }

        sleep(Duration::new(0, 1000));
    }

    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}
