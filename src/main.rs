extern crate sfml;

use std::rc::Rc;
use std::cell::RefCell;
use sfml::audio::{SoundBuffer, rc};
use sfml::window::{ContextSettings, VideoMode, event, Close};
use sfml::window::event::MouseButtonPressed;
use sfml::system::{Vector2f, Clock, Time, sleep};
use sfml::graphics::{RenderWindow, RenderTarget, RectangleShape, Color};

mod button;

fn on_button(button: &button::Button, x: i32, y: i32) -> bool {
	button.rect.get_global_bounds().contains(x as f32, y as f32)
}

fn make_button(size: &Vector2f) -> RectangleShape<'static> {
  	let mut rect = RectangleShape::new_init(size).expect("Error, cannot create button.");
	rect.set_outline_thickness(1.);
	rect.set_fill_color(&Color::new_rgb(134,179,44));
	rect.set_outline_color(&Color::new_rgb(0,70,70));
	rect
}

fn main() {
	let mut window = RenderWindow::new(VideoMode::new_init(800, 400, 32),
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
	one.rect.set_position(&Vector2f::new(button_space as f32, row_offset));

	let mut tempo = 120.;
	let mut beat = ((60./tempo) * 1000.) as i32;
	let mut div = (beat/4);

	let mut kick_hits:[bool;16] = [false;16];
	kick_hits[0] = true;
	kick_hits[4] = true;
	kick_hits[8] = true;
	kick_hits[12] = true;

	let mut hh_hits:[bool;16] = [false;16];
	hh_hits[2] = true;
	hh_hits[6] = true;
	hh_hits[10] = true;
	hh_hits[14] = true;

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

	hh.set_volume(50.);

	while window.is_open() {

		for event in window.events() {
			match event {
					event::Closed => window.close(),
					MouseButtonPressed{button, x, y} => {
						if on_button(&one, x, y) {
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
						break;
					},
					_ => { /* do nothing */}
				}
		}

		if is_playing {
			let t = clock.get_elapsed_time().as_milliseconds();
			let remainder = t%div;
			match remainder {
				0 => {
					if kick_hits[step] == true {
						kick.play();
					}
					if hh_hits[step] == true {
						hh.play();
					}
					println!("{}", step);
					if step < 15 { step += 1; } else { step = 0 };
					sleep(Time::with_milliseconds(2));
				},
				_ => { /* do nothing */ }
			}
		}

		// Clear the window
		window.clear(&Color::new_rgb(29, 115, 115));

		// Draw the shape
		window.draw(&one.rect);

		// Display things on screen
		window.display()
	}
}
