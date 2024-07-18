# egui-raylib
[Raylib](https://github.com/raysan5/raylib) integration for [egui](https://github.com/emilk/egui).
The primary use case for this crate is a drop-in GUI library for 2D games made in Raylib.

## Example Usage
1. Add this crate as a dependency.
2. Get coding!
```rust
use raylib::prelude::*;
use egui_raylib::RlEgui;

fn main() {
	let (mut rl, thread) = raylib::init()
		.size(640, 480)
		.title("Hello, World")
		.build();

	// Use default input options.
	let mut gui = RlEgui::default();

	while !rl.window_should_close() {

		// Create all UI components and prepare them for drawing.
		gui.prepare(&mut rl, &thread, |ctx| {
			// UI goes here...
			egui::CentralPanel::default().show(&ctx, |ui| {
				ui.label("Hello world!");
				if ui.button("Click me").clicked() {
					eprintln("You clicked me!");
				}
			});
		});

		let mut d = rl.begin_drawing(&thread);

		d.clear_background(Color::WHITE);
		d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);

		// Draw the gui     
		gui.draw(&mut d);

	}
}
```
## Screenshot

The screenshot of another example.

![Screenshot of an example program](screenshot.png)

## Dependencies
1. [raylib-rs](https://github.com/raylib-rs/raylib-rs): Rust-bindings for Raylib.
2. egui

# Unimplemented Features
The following features will not be supported in this integration:
1. Rendering arbitrary meshes.
2. [Paint callbacks](https://docs.rs/epaint/0.28.1/epaint/struct.PaintCallback.html).
 
The primary reason behind this is that this integration does not rely on egui to tessellate its entire UI-mesh, but rather traverses the output shape tree and calls corresponding raylib functions on a draw handle. If necessary, these features can be obtained by using egui's built-in tessellation functionality to generate primitives that can be rendered directly. This approach was not chosen to allow the ui to be rendered on any draw handle that supports clipping.
