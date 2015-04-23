extern crate sfml;

use std::rc::Rc;
use std::cell::RefCell;
use sfml::audio::{SoundBuffer, rc};
use sfml::window::{ContextSettings, VideoMode, event, Close};
use sfml::window::event::MouseButtonPressed;
use sfml::system::{Vector2f, Clock, Time, sleep};
use sfml::graphics::{RenderWindow, RenderTarget, RectangleShape, Color};

mod button;

fn on_button(rect: &RectangleShape, x: i32, y: i32) -> bool {
	rect.get_global_bounds().contains(x as f32, y as f32)
}

fn make_button(size: &Vector2f) -> RectangleShape<'static> {
  	let mut rect = RectangleShape::new_init(size).expect("Error, cannot create button.");
	rect.set_outline_thickness(1.);
	rect.set_fill_color(&Color::new_rgb(134,179,44));
	rect.set_outline_color(&Color::new_rgb(0,70,70));
	rect
}

fn light_step(step: usize, pads: &Vec<RectangleShape>) {
	//let pad = &pads[step];
	//pad.set_fill_color(&Color::new_rgb(134,179,44));
}

fn main() {
	let mut window = RenderWindow::new(VideoMode::new_init(500, 400, 32),
		"Rust Audio",
		Close,
		&ContextSettings::default())
	.expect("Cannot create a new Render Window.");

	let width = 80;
	let height = 80;
	let size = Vector2f::new(width as f32, height as f32);
	let row_offset = 310.;
	let button_space = 10;

	let mut one = button::Button::new("one", size, true);
	one.rect.set_position(&Vector2f::new(10., 10.));

	let mut kick_btn = button::Button::new("kick", size, true);
	kick_btn.rect.set_position(&Vector2f::new(10., 100.));

	let mut hh_btn = button::Button::new("hh", size, true);
	hh_btn.rect.set_position(&Vector2f::new(10., 190.));

	let mut clap_btn = button::Button::new("clap", size, true);
	clap_btn.rect.set_position(&Vector2f::new(10., 280.));

	let mut tempo = 120.;
	let mut beat = ((60./tempo) * 1000.) as i32;
	let mut div = (beat/4);

	let mut instrument = Some("kick");

	let mut kick_hits:[bool;16] = [false;16];
	kick_hits[0] = true;
	kick_hits[4] = true;
	kick_hits[8] = true;
	kick_hits[12] = true;

	let mut is_playing = false;
	let mut clock = Clock::new();
	let mut step = 0;

	let kick_buffer = match SoundBuffer::new("Samples/kick-808.wav") {
			Some(buffer)    => Rc::new(RefCell::new(buffer)),
			None            => panic!("Error, cannot load sound buffer!")
		};

	let mut kick: rc::Sound = match rc::Sound::new_with_buffer(kick_buffer.clone()) {
			Some(sound)     => sound,
			None            => panic!("Error cannot create Sound")
		};

	kick.set_volume(90.);

	let hh_buffer = match SoundBuffer::new("Samples/hihat-electro.wav") {
		Some(buffer)    => Rc::new(RefCell::new(buffer)),
		None            => panic!("Error, cannot load sound buffer!")
	};

	let mut hh: rc::Sound = match rc::Sound::new_with_buffer(hh_buffer.clone()) {
		Some(sound)     => sound,
		None            => panic!("Error cannot create Sound")
	};

	let mut hh_hits:[bool;16] = [false;16];
	hh_hits[2] = true;
	hh_hits[6] = true;
	hh_hits[10] = true;
	hh_hits[14] = true;

	hh.set_volume(70.);

	let clap_buffer = match SoundBuffer::new("Samples/clap-808.wav") {
		Some(buffer)    => Rc::new(RefCell::new(buffer)),
		None            => panic!("Error, cannot load sound buffer!")
	};

	let mut clap: rc::Sound = match rc::Sound::new_with_buffer(clap_buffer.clone()) {
		Some(sound)     => sound,
		None            => panic!("Error cannot create Sound")
	};

	let mut clap_hits:[bool;16] = [false;16];
	clap_hits[4] = true;
	clap_hits[12] = true;
	clap_hits[15] = true;

	clap.set_volume(70.);

	while window.is_open() {

		for event in window.events() {
			match event {
					event::Closed => window.close(),
					MouseButtonPressed{button, x, y} => {
						if on_button(&one.rect, x, y) {
							if is_playing {
								println!("Stoping");
							}
							else {
								clock.restart();
								step = 0;
								println!("Starting");
							}
							is_playing = !is_playing;
						}
						if on_button(&kick_btn.rect, x, y) {
							instrument = Some("kick");
						}
						if on_button(&hh_btn.rect, x, y) {
							instrument = Some("hh");
						}
						if on_button(&clap_btn.rect, x, y) {
							instrument = Some("clap");
						}
						break;
					},
					_ => { /* do nothing */}
				}
		}

		let mut pads = vec![make_button(&size);16];

		let mut row = 0;
		let pad_offset = 120;
		for x in 0..16 {
			let col = x%4;
			let left_offset = ((width * col) + ((button_space * (col + 1)) + pad_offset)) as f32;
			let top_offset = ((height * row) + (button_space * (row + 1))) as f32;
			pads[x].set_position(&Vector2f::new(left_offset, top_offset));


			match instrument {
				Some("kick") => {
					if kick_hits[x] {
						pads[x].set_fill_color(&Color::new_rgb(181,158,44));
					}
				},
				Some("hh") => {
					if hh_hits[x] {
						pads[x].set_fill_color(&Color::new_rgb(181,158,44));
					}
				},
				Some("clap") => {
					if clap_hits[x] {
						pads[x].set_fill_color(&Color::new_rgb(181,158,44));
					}
				},
				Some(_) => {/* do nothing */}
				None => {/* do nothing */}
			}
			if is_playing && x == step {
				pads[x].set_fill_color(&Color::new_rgb(190,223,124));
			}
			if col == 3 { row += 1; }
		}

		if is_playing {
			let t = clock.get_elapsed_time().as_milliseconds();
			let remainder = t%div;
			match remainder {
				0 => {
					if kick_hits[step] {
						kick.play();
					}
					if hh_hits[step] {
						hh.play();
					}
					if clap_hits[step] {
						clap.play();
					}
					if step < 15 { step += 1; } else { step = 0 };
					sleep(Time::with_milliseconds(100));
				},
				_ => { /* do nothing */ }
			}
		}



		// Clear the window
		window.clear(&Color::new_rgb(29, 115, 115));

		// Draw the shape
		window.draw(&one.rect);
		window.draw(&kick_btn.rect);
		window.draw(&hh_btn.rect);
		window.draw(&clap_btn.rect);

		for x in 0..16 {
			window.draw(&pads[x]);
		}

		// Display things on screen
		window.display()
	}
}
