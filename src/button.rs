extern crate sfml;

use sfml::graphics::{RectangleShape, Color};
use sfml::system::Vector2f;

pub struct Button {
	pub rect: RectangleShape<'static>,
	pub label: &'static str,
	pub size: Vector2f,
	pub enabled: bool
}

impl Button {
	pub fn new(label: &'static str, size: Vector2f, enabled: bool) -> Button {
		let mut rect = RectangleShape::new_init(&size).expect("Error, cannot create button.");
		rect.set_outline_thickness(1.);
		rect.set_fill_color(&Color::new_rgb(134,179,44));
		rect.set_outline_color(&Color::new_rgb(0,70,70));

		Button {
			rect: rect,
			label: label,
			size: size,
			enabled: enabled
		}
	}
}
