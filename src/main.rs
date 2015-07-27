extern crate sfml;

use std::rc::Rc;
use std::cell::RefCell;
use sfml::audio::{SoundBuffer, rc};
use sfml::window::{ContextSettings, VideoMode, event, Close};
use sfml::window::event::MouseButtonPressed;
use sfml::system::{Vector2f, Clock};
use sfml::graphics::{RenderWindow, RenderTarget, RectangleShape, Color};

mod button;

fn main() {
	let mut window = RenderWindow::new(VideoMode::new_init(500, 400, 32),
		"Rust Audio",
		Close,
		&ContextSettings::default())
	.expect("Cannot create a new Render Window.");

	// Initialize size values for pad UI elements
	let width = 80;
	let height = 80;
	let size = Vector2f::new(width as f32, height as f32);
	let button_space = 10;

	// Start/Stop button
	let mut one = button::Button::new("one", size);
	one.rect.set_position(&Vector2f::new(10., 10.));

	// Select "kick" instrument button
	let mut kick_btn = button::Button::new("kick", size);
	kick_btn.rect.set_position(&Vector2f::new(10., 100.));

	// Select "hh" instrument button
	let mut hh_btn = button::Button::new("hh", size);
	hh_btn.rect.set_position(&Vector2f::new(10., 190.));

	// Select "clap" instrument button
	let mut clap_btn = button::Button::new("clap", size);
	clap_btn.rect.set_position(&Vector2f::new(10., 280.));

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

	let hh_buffer = match SoundBuffer::new("Samples/hihat-plain.wav") {
		Some(buffer)    => Rc::new(RefCell::new(buffer)),
		None            => panic!("Error, cannot load sound buffer!")
	};

	let mut hh: rc::Sound = match rc::Sound::new_with_buffer(hh_buffer.clone()) {
		Some(sound)     => sound,
		None            => panic!("Error cannot create Sound")
	};

	let mut hh_hits:[bool;16] = [false;16];
	hh_hits[0] = true;
	hh_hits[2] = true;
	hh_hits[4] = true;
	hh_hits[6] = true;
	hh_hits[8] = true;
	hh_hits[10] = true;
	hh_hits[11] = true;
	hh_hits[13] = true;
	hh_hits[14] = true;

	hh.set_volume(90.);

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

	// Initialize time signature math vars
	let tempo = 120.;
	let beat = ((60./tempo) * 1000.) as i32;
	let div = beat/4;

	let mut redraw = true;

	while window.is_open() {

		let mut advance = false;

		// Set up responses to UI events
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
						redraw = true;
					}
					if on_button(&hh_btn.rect, x, y) {
						instrument = Some("hh");
						redraw = true;
					}
					if on_button(&clap_btn.rect, x, y) {
						instrument = Some("clap");
						redraw = true;
					}
					break;
				},
				_ => { /* do nothing */}
			}
		}

		if is_playing {
			let mut remainder = 0;

			// Wait until it's time to play a sound
			let tick = clock.get_elapsed_time().as_milliseconds();
			loop { if tick != clock.get_elapsed_time().as_milliseconds() {break;} }

			if tick % div == 0 {
				redraw = true;
				advance = true;
				if kick_hits[step] {kick.play();}
				if hh_hits[step] {hh.play();}
				if clap_hits[step] {clap.play();}
			}
		}

		// Initialize vars for grid-drawing loop
		let mut row = 0;
		let pad_offset = 120;

		// Initialize pad shapes for grid
		let mut pads = vec![make_button(&size);16];

		for (i, pad) in pads.iter_mut().enumerate() {

			let col = (i as i32) % (4 as i32);
			let position = calculate_position(col, row, width, height, button_space, pad_offset);

			pad.set_position(&position);

			let on_color   = Color::new_rgb(181,89,44);
			let step_color = Color::new_rgb(190,223,124);

			if is_playing && i == step { pad.set_fill_color(&step_color); }
			else {
				match instrument {
					Some("kick") => {
						if kick_hits[i] { pad.set_fill_color(&on_color); }
					},
					Some("hh") => {
						if hh_hits[i] 	{ pad.set_fill_color(&on_color); }
					},
					Some("clap") => {
						if clap_hits[i] { pad.set_fill_color(&on_color); }
					},
					Some(_) => { }
					None => { }
				}
			}
			if col == 3 { row += 1; }
		}

		// Clear the window
		if redraw {
			window.clear(&Color::new_rgb(29, 115, 115));

			// Draw the left buttons
			window.draw(&one.rect);
			window.draw(&kick_btn.rect);
			window.draw(&hh_btn.rect);
			window.draw(&clap_btn.rect);

			// Draw the grid
			for pad in pads { window.draw(&pad);}

			window.display();

			redraw = false;
		}
		if advance { step = next_step(step) }
	}
}

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

fn calculate_left_offset(width: i32, col: i32, button_space: i32, pad_offset: i32) -> f32 {
	((width * col) + ((button_space * (col + 1)) + pad_offset)) as f32
}

fn calculate_top_offset(height: i32, row: i32, button_space: i32) -> f32 {
	((height * row) + (button_space * (row + 1))) as f32
}

fn calculate_position(col: i32, row: i32, col_width: i32, row_height: i32, gutter: i32, left_offset: i32) -> Vector2f {
	let left = calculate_left_offset(col_width, col, gutter, left_offset);
	let top  = calculate_top_offset(row_height, row, gutter);
	Vector2f::new(left, top)
}

fn next_step(step: usize) -> usize {
	if step < 15 { return step + 1} else { return 0 };
}
