#![warn(missing_docs)]

//! A crate that implements a dead-simple [raylib](https://github.com/deltaphc/raylib-rs) integration for [egui](https://github.com/emilk/egui)
//! # Example Usage
//! ```rust
//! use raylib::prelude::*;
//! use egui_raylib::RlEgui;
//!
//! let (mut rl, thread) = raylib::init()
//!     .size(640, 480)
//!     .title("Hello, World")
//!     .build();
//!
//! let mut gui = RlEgui::default();
//!  
//! while !rl.window_should_close() {
//!     
//!     // Create all UI components and prepare them for drawing.
//!     gui.prepare(&mut rl, &thread, |ctx| {
//!         egui::CentralPanel::default().show(&ctx, |ui| {
//!            ui.label("Hello world!");
//!            if ui.button("Click me").clicked() {
//!                // take some action here
//!            }
//!        });
//!     });
//!     
//!     let mut d = rl.begin_drawing(&thread);
//!  
//!     d.clear_background(Color::WHITE);
//!     d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
//!     
//!     // Draw the gui     
//!     gui.draw(&mut d);
//!     
//! }
//! ```

use input::{gather_input, InputOptions};
use paint::{Painter, PlatformHandler};
use raylib::{
    drawing::{RaylibDraw, RaylibScissorModeExt},
    RaylibHandle, RaylibThread,
};

/// Re-export egui crate for convenience.
pub use egui;

pub mod input;
pub mod paint;
pub mod util;

#[cfg(test)]
mod tests;

/// A no-op implementor for [paint::PlatformHandler]
pub struct DummyHandler;

impl paint::PlatformHandler for DummyHandler {
    fn open_url(&mut self, _url: egui::OpenUrl) {}
    fn output_events(&mut self, _vec: &[egui::output::OutputEvent]) {}
}

#[derive(Default)]
/// A structure to simplify use of [egui] with [raylib]
pub struct RlEgui {
    /// The underlying [egui::Context] owned by this struct.
    pub ctx: egui::Context,
    inopt: InputOptions,
    prs: Option<paint::PreparedShapes>,
    painter: paint::Painter,
}

impl RlEgui {
    /// Constructor.
    pub fn new(inopt: InputOptions, ctx: egui::Context) -> RlEgui {
        Self {
            ctx,
            inopt,
            prs: None,
            painter: Painter::default(),
        }
    }

    /// Perform all pre-draw steps such as loading and freeing textures, and prepare the shapes to be drawn.
    /// A [DummyHandler] is used for handling platform events (no-op).
    pub fn prepare<F>(&mut self, rl: &mut RaylibHandle, rthread: &RaylibThread, run_ui: F)
    where
        F: FnOnce(&egui::Context),
    {
        self.prepare_with(rl, rthread, run_ui, &mut DummyHandler);
    }

    /// Perform all pre-draw steps and prepare shapes to be drawn. Use the provided handler for handling platform events.
    pub fn prepare_with<F, H>(
        &mut self,
        rl: &mut RaylibHandle,
        rthread: &RaylibThread,
        run_ui: F,
        handler: &mut H,
    ) where
        F: FnOnce(&egui::Context),
        H: PlatformHandler,
    {
        let raw_input = gather_input(&self.inopt, &self.ctx, rl);
        let output = paint::full_output(rl, raw_input, &self.ctx, run_ui, handler);
        let prepared = self.painter.predraw(output, rl, rthread);
        self.prs.replace(prepared);
    }

    /// Draw the previosly prepared shapes.
    /// # Panics
    /// If [RlEgui::prepare] was never called after the last draw.
    pub fn draw<D>(&mut self, d: &mut D)
    where
        D: RaylibDraw + RaylibScissorModeExt,
    {
        let prepared_shapes = self
            .prs
            .take()
            .expect("GUI should be prepared before drawing. There are no prepared shapes now.");
        self.painter.paint(prepared_shapes, d);
    }
}
