use egui::{Color32, Context, Visuals};
use raylib::prelude::{Color, RaylibDraw};

use crate::{
    input::{gather_input, InputOptions},
    paint::{full_output, Painter},
    DummyHandler,
};

fn run_ui(ctx: &Context) {
    egui::Window::new("Test window")
        .default_width(300.0)
        .default_height(480.0)
        .open(&mut true)
        .resizable([true, false])
        .show(ctx, |ui| {
            ui.heading("egui-raylib");
            ui.label("egui-raylib provides egui integration for raylib.");

            ui.add_space(12.0);
            ui.separator();
            ui.heading("Info");

            ui.label(
                "egui is an immediate mode gui, designed to be easy to use, portable, and fast.",
            );
            ui.add_space(12.0);
            ui.label("raylib is a simple and easy-to-use library to enjoy videogames programming.");

            ui.add_space(12.0);
            ui.label("egui + raylib = perfect match");

            ui.separator();
            ui.button("Yay!").on_hover_text("This is a tool-tip");
        });
}

#[test]
fn it_works() {
    let (mut rl, thread) = raylib::init().size(1024, 768).title("Hello, World").build();
    let ctx = Context::default();
    ctx.set_visuals(Visuals {
        override_text_color: Some(Color32::WHITE),
        ..Visuals::dark()
    });
    let iopt = InputOptions {
        native_pixels_per_point: 1.25,
        ..Default::default()
    };
    let mut painter = Painter::default();

    while !rl.window_should_close() {
        let raw_input = gather_input(&iopt, &ctx, &mut rl);
        let output = full_output(&mut rl, raw_input, &ctx, run_ui, &mut DummyHandler);
        let prepared = painter.predraw(output, &mut rl, &thread);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 0, 0, 20, Color::BLACK);
        painter.paint(prepared, &mut d);
    }
}
