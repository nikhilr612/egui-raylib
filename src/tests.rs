use egui::{Color32, Context, Label, RichText, Visuals};
use raylib::prelude::{Color, RaylibDraw};

use crate::{
    input::{gather_input, InputOptions},
    paint::{full_output, Painter},
    DummyHandler, RlEgui,
};

#[derive(PartialEq)]
#[derive(Debug)]
enum TestEnum {
    First,
    Second,
    Third
}

/// UI to test widgets.
/// Code taken from [egui demo](https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/widget_gallery.rs)
struct TestUi {
    enabled: bool,
    visible: bool,
    opacity: f32,
    boolean: bool,
    radio: TestEnum,
    scalar: f32,
    string: String,
    color: Color32,
    animate_progress_bar: bool
}

// Omitted - 
//  1. Image (for now)
//  2. ColorPicker (requires Meshes)
//  3. 

fn doc_link_label(a: &str, _b: &str) -> Label {
    Label::new(RichText::new(a).color(Color32::BLUE))
}

impl TestUi {
    fn gallery_grid_contents(&mut self, ui: &mut egui::Ui) {
        let Self {
            enabled: _,
            visible: _,
            opacity: _,
            boolean,
            radio,
            scalar,
            string,
            color,
            animate_progress_bar
        } = self;

        ui.add(doc_link_label("Label", "label"));
        ui.label("Welcome to the widget gallery!");
        ui.end_row();

        ui.add(doc_link_label("Hyperlink", "Hyperlink"));
        use egui::special_emojis::GITHUB;
        ui.hyperlink_to(
            format!("{GITHUB} egui on GitHub"),
            "https://github.com/emilk/egui",
        );
        ui.end_row();

        ui.add(doc_link_label("TextEdit", "TextEdit"));
        ui.add(egui::TextEdit::singleline(string).hint_text("Write something here"));
        ui.end_row();

        ui.add(doc_link_label("Button", "button"));
        if ui.button("Click me!").clicked() {
            *boolean = !*boolean;
        }
        ui.end_row();

        ui.add(doc_link_label("Link", "link"));
        if ui.link("Click me!").clicked() {
            *boolean = !*boolean;
        }
        ui.end_row();

        ui.add(doc_link_label("Checkbox", "checkbox"));
        ui.checkbox(boolean, "Checkbox");
        ui.end_row();

        ui.add(doc_link_label("RadioButton", "radio"));
        ui.horizontal(|ui| {
            ui.radio_value(radio, TestEnum::First, "First");
            ui.radio_value(radio, TestEnum::Second, "Second");
            ui.radio_value(radio, TestEnum::Third, "Third");
        });
        ui.end_row();

        ui.add(doc_link_label("SelectableLabel", "SelectableLabel"));
        ui.horizontal(|ui| {
            ui.selectable_value(radio, TestEnum::First, "First");
            ui.selectable_value(radio, TestEnum::Second, "Second");
            ui.selectable_value(radio, TestEnum::Third, "Third");
        });
        ui.end_row();

        ui.add(doc_link_label("ComboBox", "ComboBox"));

        egui::ComboBox::from_label("Take your pick")
            .selected_text(format!("{radio:?}"))
            .show_ui(ui, |ui| {
                ui.selectable_value(radio, TestEnum::First, "First");
                ui.selectable_value(radio, TestEnum::Second, "Second");
                ui.selectable_value(radio, TestEnum::Third, "Third");
            });
        ui.end_row();

        ui.add(doc_link_label("Slider", "Slider"));
        ui.add(egui::Slider::new(scalar, 0.0..=360.0).suffix("°"));
        ui.end_row();

        ui.add(doc_link_label("DragValue", "DragValue"));
        ui.add(egui::DragValue::new(scalar).speed(1.0));
        ui.end_row();

        ui.add(doc_link_label("ProgressBar", "ProgressBar"));
        let progress = *scalar / 360.0;
        let progress_bar = egui::ProgressBar::new(progress)
            .show_percentage()
            .animate(*animate_progress_bar);
        *animate_progress_bar = ui
            .add(progress_bar)
            .on_hover_text("The progress bar can be animated!")
            .hovered();
        ui.end_row();

        ui.add(doc_link_label("Separator", "separator"));
        ui.separator();
        ui.end_row();

        ui.add(doc_link_label("CollapsingHeader", "collapsing"));
        ui.collapsing("Click to see what is hidden!", |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("It's a ");
                ui.add(doc_link_label("Spinner", "spinner"));
                ui.add_space(4.0);
                ui.add(egui::Spinner::new());
            });
        });
        ui.end_row();
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.add_enabled_ui(self.enabled, |ui| {
            if !self.visible {
                ui.set_invisible();
            }
            ui.multiply_opacity(self.opacity);

            egui::Grid::new("my_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    self.gallery_grid_contents(ui);
                });
        });

        ui.separator();

        ui.horizontal(|ui| {
            ui.checkbox(&mut self.visible, "Visible")
                .on_hover_text("Uncheck to hide all the widgets.");
            if self.visible {
                ui.checkbox(&mut self.enabled, "Interactive")
                    .on_hover_text("Uncheck to inspect how the widgets look when disabled.");
                (ui.add(
                    egui::DragValue::new(&mut self.opacity)
                        .speed(0.01)
                        .range(0.0..=1.0),
                ) | ui.label("Opacity"))
                .on_hover_text("Reduce this value to make widgets semi-transparent");
            }
        });

        ui.separator();

        ui.vertical_centered(|ui| {
            let tooltip_text = "The full egui documentation.\nYou can also click the different widgets names in the left column.";
            ui.hyperlink("https://docs.rs/egui/").on_hover_text(tooltip_text);
        });
    }

    fn run(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new("Test Widgets")
            .open(open)
            .resizable([true, false])
            .default_width(280.0)
            .show(ctx, |ui| {
                self.ui(ui);
            });
    }
}

#[test]
fn it_works() {
    let (mut rl, thread) = raylib::init().size(768, 1024).title("Hello, World").build();
    let ctx = Context::default();

    ctx.set_visuals(Visuals {
        override_text_color: Some(Color32::WHITE),
        hyperlink_color: Color32::BLUE,
        ..Visuals::dark()
    });

    let inopt = InputOptions {
        native_pixels_per_point: 1.25,
        ..Default::default()
    };

    let mut test_ui = TestUi {
        enabled: true,
        visible: true,
        radio: TestEnum::First,
        opacity: 1.0,
        boolean: false,
        scalar: 0.0,
        string: String::new(),
        color: Color32::WHITE,
        animate_progress_bar: true,
        
    };

    let mut bool_flag = true;

    let mut gui = RlEgui::new(inopt, ctx);

    while !rl.window_should_close() {
        gui.prepare(&mut rl, &thread, |c| test_ui.run(c, &mut bool_flag));        

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 0, 0, 20, Color::BLACK);

        gui.draw(&mut d);
    }
}