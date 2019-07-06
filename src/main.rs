#![warn(rust_2018_idioms)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use std::thread::sleep;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

type RGBValue = [u8; 3];

static WINDOW_WIDTH: u32 = 800;
static WINDOW_HEIGHT: u32 = 600;
static INDICATOR_SIZE: u32 = 50;
static COLOR_STEP_SIZE: u8 = 5;

macro_rules! rect(
    ($x: expr, $y: expr, $w: expr, $h: expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

fn update_color_indicator(
	canvas: &mut Canvas<Window>,
	bg_color_index: usize,
) -> Result<(), String> {
	let color: RGBValue = match bg_color_index {
		0 => [255, 0, 0],
		1 => [0, 255, 0],
		2 => [0, 0, 255],
		_ => panic!("invalid color index"),
	};
	canvas.set_draw_color(Color::RGB(color[0], color[1], color[2]));
	canvas.fill_rect(rect!(
		WINDOW_WIDTH - INDICATOR_SIZE,
		WINDOW_HEIGHT - INDICATOR_SIZE,
		INDICATOR_SIZE,
		INDICATOR_SIZE
	))?;
	Ok(())
}

fn redraw(
	canvas: &mut Canvas<Window>,
	bg_color: RGBValue,
	bg_color_index: usize,
) -> Result<(), String> {
	canvas.set_draw_color(Color::RGB(bg_color[0], bg_color[1], bg_color[2]));
	canvas.clear();
	update_color_indicator(canvas, bg_color_index)?;
	canvas.present();
	Ok(())
}

fn run() -> Result<(), String> {
	let sdl_context = sdl2::init()?;
	let mut event_pump = sdl_context.event_pump()?;
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
	let mut bg_color_index: usize = 0;
	redraw(&mut canvas, bg_color, bg_color_index)?;

	'main_loop: loop {
		for event in event_pump.poll_iter() {
			match event {
				// Increases the color value of one color
				Event::KeyDown {
					keycode: Some(Keycode::Up),
					..
				} => {
					if bg_color[bg_color_index] < 255 {
						bg_color[bg_color_index] += COLOR_STEP_SIZE;
						redraw(&mut canvas, bg_color, bg_color_index)?;
					}
				}

				// Decreases the color value of one color
				Event::KeyDown {
					keycode: Some(Keycode::Down),
					..
				} => {
					if bg_color[bg_color_index] > 0 {
						bg_color[bg_color_index] -= COLOR_STEP_SIZE;
						redraw(&mut canvas, bg_color, bg_color_index)?;
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
					update_color_indicator(&mut canvas, bg_color_index)?;
					canvas.present();
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
					update_color_indicator(&mut canvas, bg_color_index)?;
					canvas.present();
				}

				// Resets to black
				Event::KeyDown {
					keycode: Some(Keycode::Space),
					..
				} => {
					bg_color = [0, 0, 0];
					redraw(&mut canvas, bg_color, bg_color_index)?;
				}

				// Quits the program
				Event::KeyDown {
					keycode: Some(Keycode::Escape),
					..
				}
				| Event::KeyDown {
					keycode: Some(Keycode::Q),
					..
				}
				| Event::Quit { .. } => break 'main_loop,

				// Sleep so 20 events can happen per second
				_ => sleep(Duration::new(0, 50_000_000)),
			}
		}
	}

	Ok(())
}

fn main() {
	if let Err(e) = run() {
		eprintln!("Error: {}", e);
		std::process::exit(1);
	}
}
