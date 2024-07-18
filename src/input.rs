//! A module to handle input from Raylib and create `RawInput` for `egui`.

use std::collections::HashMap;
use std::iter;
use std::path::PathBuf;

use egui::Rect as egRect;
use egui::{DroppedFile, Event, Key, Modifiers, Pos2, RawInput, Vec2, ViewportId, ViewportInfo};
use raylib::ffi::{KeyboardKey, MouseButton};
use raylib::prelude::Rectangle as rayRect;
use raylib::RaylibHandle;

/// Struct to store values
pub struct InputOptions {
    /// 'Point' to _native pixel_ conversion ratio. 'Points' are `egui`'s logical pixels.
    pub native_pixels_per_point: f32,
    /// Maximum texture size supported on GPU.
    pub max_texture_size: Option<usize>,
    /// Region of window allocated for egui to use.
    pub region: Option<rayRect>,
    /// Map raylib's non-character keys to their egui counterparts.
    pub key_map: HashMap<KeyboardKey, Key>,
}

impl Default for InputOptions {
    fn default() -> Self {
        let mut key_map = HashMap::new();
        key_map.insert(KeyboardKey::KEY_ENTER, Key::Enter);
        key_map.insert(KeyboardKey::KEY_BACKSPACE, Key::Backspace);
        key_map.insert(KeyboardKey::KEY_UP, Key::ArrowUp);
        key_map.insert(KeyboardKey::KEY_DOWN, Key::ArrowDown);
        key_map.insert(KeyboardKey::KEY_LEFT, Key::ArrowLeft);
        key_map.insert(KeyboardKey::KEY_RIGHT, Key::ArrowRight);
        Self {
            native_pixels_per_point: 1.0,
            max_texture_size: None,
            region: None,
            key_map,
        }
    }
}

fn conv_rect(r: rayRect) -> egRect {
    egRect {
        min: Pos2::new(r.x, r.y),
        max: Pos2::new(r.x + r.width, r.y + r.height),
    }
}

/// Using the provided input options, gather all required input for egui.
pub fn gather_input(opt: &InputOptions, ctx: &egui::Context, rl: &mut RaylibHandle) -> RawInput {
    let monitor_id = raylib::window::get_current_monitor();
    let (mw, mh) = (
        raylib::window::get_monitor_width(monitor_id),
        raylib::window::get_monitor_height(monitor_id),
    );
    let pixels_per_point = ctx.zoom_factor() * opt.native_pixels_per_point;

    let monitor_size = Vec2::new(mw as f32 / pixels_per_point, mh as f32 / pixels_per_point);
    let window_size = Some(egRect::from_min_max(
        Pos2::ZERO,
        Pos2::new(
            rl.get_screen_width() as f32 / pixels_per_point,
            rl.get_screen_height() as f32 / pixels_per_point,
        ),
    ));

    let viewport = ViewportInfo {
        parent: None,
        title: None,
        events: Default::default(),
        native_pixels_per_point: Some(opt.native_pixels_per_point),
        monitor_size: Some(monitor_size),
        inner_rect: window_size,
        outer_rect: window_size,
        minimized: Some(rl.is_window_minimized()),
        maximized: None,
        fullscreen: Some(rl.is_window_fullscreen()),
        focused: Some(rl.is_window_focused()),
    };

    let screen_rect = opt.region.map(conv_rect).or(window_size);

    let modifiers = Modifiers {
        alt: rl.is_key_down(KeyboardKey::KEY_LEFT_ALT)
            || rl.is_key_down(KeyboardKey::KEY_RIGHT_ALT),
        ctrl: rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL)
            || rl.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL),
        shift: rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT)
            || rl.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT),
        mac_cmd: false,
        command: false,
    };

    let mut events: Vec<_> = opt
        .key_map
        .iter()
        .filter_map(|(&kk, &key)| {
            if rl.is_key_pressed(kk) {
                Some(Event::Key {
                    key,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers,
                })
            } else if rl.is_key_released(kk) {
                Some(Event::Key {
                    key,
                    physical_key: None,
                    pressed: false,
                    repeat: false,
                    modifiers,
                })
            } else {
                None
            }
        })
        .collect();

    if let Some(key) = rl.get_char_pressed().and_then(|ch| {
        let mut s = String::new();
        s.push(ch);
        Key::from_name(&s)
    }) {
        events.push(Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers,
        })
    }

    if rl.is_key_pressed(KeyboardKey::KEY_C) && modifiers.ctrl {
        events.push(Event::Copy)
    } else if rl.is_key_pressed(KeyboardKey::KEY_V) && modifiers.ctrl {
        match rl.get_clipboard_text() {
			Ok(s) => events.push(Event::Paste(s)),
			Err(e) => eprintln!("egui-raylib: Expect clipboard to have utf8 text, cannot paste otherwise\n\tdetail: {e}")
		}
    }

    let mouse_delta = rl.get_mouse_delta().scale_by(1.0 / pixels_per_point);
    let mouse_position = rl.get_mouse_position().scale_by(1.0 / pixels_per_point);
    if mouse_delta.x > 0.0 || mouse_delta.y > 0.0 {
        events.push(Event::MouseMoved(Vec2::new(mouse_delta.x, mouse_delta.y)));
        events.push(Event::PointerMoved(Pos2::new(
            mouse_position.x,
            mouse_position.y,
        )));
    }

    if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
        let pos = rl.get_mouse_position();
        let pos = Pos2::new(pos.x / pixels_per_point, pos.y / pixels_per_point);
        events.push(Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers,
        })
    } else if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
        let pos = rl.get_mouse_position();
        let pos = Pos2::new(pos.x / pixels_per_point, pos.y / pixels_per_point);
        events.push(Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers,
        })
    }

    if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT) {
        let pos = rl.get_mouse_position();
        let pos = Pos2::new(pos.x / pixels_per_point, pos.y / pixels_per_point);
        events.push(Event::PointerButton {
            pos,
            button: egui::PointerButton::Secondary,
            pressed: true,
            modifiers,
        })
    } else if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_RIGHT) {
        let pos = rl.get_mouse_position();
        let pos = Pos2::new(pos.x / pixels_per_point, pos.y / pixels_per_point);
        events.push(Event::PointerButton {
            pos,
            button: egui::PointerButton::Secondary,
            pressed: false,
            modifiers,
        })
    }

    let dropped_files = if rl.is_file_dropped() {
        rl.load_dropped_files()
            .paths()
            .iter()
            .map(|&path| {
                let path = PathBuf::from(path);
                let name = path
                    .file_name()
                    .expect("Expect dropped file to have file name.")
                    .to_string_lossy()
                    .into_owned();
                DroppedFile {
                    path: Some(path),
                    name,
                    mime: "application/octet-stream".to_owned(),
                    last_modified: None,
                    bytes: None,
                }
            })
            .collect()
    } else {
        Vec::new()
    };

    // if !events.is_empty() { println!("Events: {events:?}"); }

    RawInput {
        viewport_id: ViewportId::ROOT,
        viewports: iter::once((ViewportId::ROOT, viewport)).collect(),
        screen_rect,
        max_texture_side: None,
        time: Some(rl.get_time()),
        predicted_dt: 1.0 / 60.0,
        modifiers: Modifiers::default(),
        events,
        hovered_files: Default::default(),
        dropped_files,
        focused: rl.is_window_focused(),
    }
}
