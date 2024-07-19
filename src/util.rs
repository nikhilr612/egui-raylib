//! Some handy conversion stuff between Raylib and Egui types.

/// A trait to convert between raylib and egui types.
pub trait ConvertRE<T> {
    /// Construct the corresponding type from this type's data.
    fn convert(&self) -> T;
}

impl ConvertRE<raylib::prelude::Vector2> for egui::Pos2 {
    fn convert(&self) -> raylib::prelude::Vector2 {
        raylib::prelude::Vector2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl ConvertRE<egui::Pos2> for raylib::prelude::Vector2 {
    fn convert(&self) -> egui::Pos2 {
        egui::Pos2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl ConvertRE<raylib::math::Rectangle> for egui::Rect {
    fn convert(&self) -> raylib::math::Rectangle {
        raylib::math::Rectangle {
            x: self.min.x,
            y: self.min.y,
            width: self.width(),
            height: self.height(),
        }
    }
}

impl ConvertRE<egui::Rect> for raylib::math::Rectangle {
    fn convert(&self) -> egui::Rect {
        egui::Rect {
            min: egui::Pos2 {
                x: self.x,
                y: self.y,
            },
            max: egui::Pos2 {
                x: self.x + self.width,
                y: self.y + self.height,
            },
        }
    }
}

impl ConvertRE<raylib::prelude::Color> for egui::Color32 {
    fn convert(&self) -> raylib::prelude::Color {
        let v = self.to_srgba_unmultiplied();
        raylib::prelude::Color {
            r: v[0],
            g: v[1],
            b: v[2],
            a: v[3],
        }
    }
}

impl ConvertRE<Option<raylib::prelude::MouseCursor>> for egui::CursorIcon {
    fn convert(&self) -> Option<raylib::consts::MouseCursor> {
        let v = match self {
            egui::CursorIcon::Default => raylib::prelude::MouseCursor::MOUSE_CURSOR_DEFAULT,
            egui::CursorIcon::ContextMenu => raylib::prelude::MouseCursor::MOUSE_CURSOR_ARROW,
            egui::CursorIcon::Help => raylib::prelude::MouseCursor::MOUSE_CURSOR_POINTING_HAND,
            egui::CursorIcon::PointingHand => {
                raylib::prelude::MouseCursor::MOUSE_CURSOR_POINTING_HAND
            }
            egui::CursorIcon::Crosshair => raylib::prelude::MouseCursor::MOUSE_CURSOR_CROSSHAIR,
            egui::CursorIcon::Text => raylib::prelude::MouseCursor::MOUSE_CURSOR_IBEAM,
            egui::CursorIcon::VerticalText => raylib::prelude::MouseCursor::MOUSE_CURSOR_IBEAM,
            egui::CursorIcon::NoDrop => raylib::prelude::MouseCursor::MOUSE_CURSOR_NOT_ALLOWED,
            egui::CursorIcon::NotAllowed => raylib::prelude::MouseCursor::MOUSE_CURSOR_NOT_ALLOWED,
            egui::CursorIcon::Grab => raylib::prelude::MouseCursor::MOUSE_CURSOR_ARROW,
            egui::CursorIcon::Grabbing => raylib::prelude::MouseCursor::MOUSE_CURSOR_POINTING_HAND,
            egui::CursorIcon::ResizeHorizontal => {
                raylib::prelude::MouseCursor::MOUSE_CURSOR_RESIZE_EW
            }
            egui::CursorIcon::ResizeNeSw => raylib::prelude::MouseCursor::MOUSE_CURSOR_RESIZE_NESW,
            egui::CursorIcon::ResizeNwSe => raylib::prelude::MouseCursor::MOUSE_CURSOR_RESIZE_NWSE,
            egui::CursorIcon::ResizeVertical => {
                raylib::prelude::MouseCursor::MOUSE_CURSOR_RESIZE_NS
            }
            egui::CursorIcon::ResizeEast => raylib::prelude::MouseCursor::MOUSE_CURSOR_RESIZE_EW,
            egui::CursorIcon::ResizeSouthEast => {
                raylib::prelude::MouseCursor::MOUSE_CURSOR_RESIZE_NWSE
            }
            egui::CursorIcon::ResizeSouth => raylib::prelude::MouseCursor::MOUSE_CURSOR_RESIZE_NS,
            egui::CursorIcon::ResizeSouthWest => {
                raylib::prelude::MouseCursor::MOUSE_CURSOR_RESIZE_NESW
            }
            egui::CursorIcon::ResizeWest => raylib::prelude::MouseCursor::MOUSE_CURSOR_RESIZE_EW,
            egui::CursorIcon::ResizeNorthWest => {
                raylib::prelude::MouseCursor::MOUSE_CURSOR_RESIZE_NWSE
            }
            egui::CursorIcon::ResizeNorth => raylib::prelude::MouseCursor::MOUSE_CURSOR_RESIZE_NS,
            egui::CursorIcon::ResizeNorthEast => {
                raylib::prelude::MouseCursor::MOUSE_CURSOR_RESIZE_NESW
            }
            egui::CursorIcon::ResizeColumn => raylib::prelude::MouseCursor::MOUSE_CURSOR_RESIZE_ALL,
            egui::CursorIcon::ResizeRow => raylib::prelude::MouseCursor::MOUSE_CURSOR_RESIZE_ALL,

            egui::CursorIcon::None => {
                return None;
            }
            _ => raylib::prelude::MouseCursor::MOUSE_CURSOR_DEFAULT,
        };
        Some(v)
    }
}

// Implement ConvertRE trait for converting from KeyboardKey to Key
impl ConvertRE<Option<egui::Key>> for raylib::prelude::KeyboardKey {
    fn convert(&self) -> Option<egui::Key> {
        use egui::Key;
        use raylib::prelude::KeyboardKey;
        let v = match *self {
            KeyboardKey::KEY_NULL => Key::Space,
            KeyboardKey::KEY_APOSTROPHE => Key::Quote,
            KeyboardKey::KEY_COMMA => Key::Comma,
            KeyboardKey::KEY_MINUS => Key::Minus,
            KeyboardKey::KEY_PERIOD => Key::Period,
            KeyboardKey::KEY_SLASH => Key::Slash,
            KeyboardKey::KEY_ZERO => Key::Num0,
            KeyboardKey::KEY_ONE => Key::Num1,
            KeyboardKey::KEY_TWO => Key::Num2,
            KeyboardKey::KEY_THREE => Key::Num3,
            KeyboardKey::KEY_FOUR => Key::Num4,
            KeyboardKey::KEY_FIVE => Key::Num5,
            KeyboardKey::KEY_SIX => Key::Num6,
            KeyboardKey::KEY_SEVEN => Key::Num7,
            KeyboardKey::KEY_EIGHT => Key::Num8,
            KeyboardKey::KEY_NINE => Key::Num9,
            KeyboardKey::KEY_SEMICOLON => Key::Semicolon,
            KeyboardKey::KEY_EQUAL => Key::Equals,
            KeyboardKey::KEY_A => Key::A,
            KeyboardKey::KEY_B => Key::B,
            KeyboardKey::KEY_C => Key::C,
            KeyboardKey::KEY_D => Key::D,
            KeyboardKey::KEY_E => Key::E,
            KeyboardKey::KEY_F => Key::F,
            KeyboardKey::KEY_G => Key::G,
            KeyboardKey::KEY_H => Key::H,
            KeyboardKey::KEY_I => Key::I,
            KeyboardKey::KEY_J => Key::J,
            KeyboardKey::KEY_K => Key::K,
            KeyboardKey::KEY_L => Key::L,
            KeyboardKey::KEY_M => Key::M,
            KeyboardKey::KEY_N => Key::N,
            KeyboardKey::KEY_O => Key::O,
            KeyboardKey::KEY_P => Key::P,
            KeyboardKey::KEY_Q => Key::Q,
            KeyboardKey::KEY_R => Key::R,
            KeyboardKey::KEY_S => Key::S,
            KeyboardKey::KEY_T => Key::T,
            KeyboardKey::KEY_U => Key::U,
            KeyboardKey::KEY_V => Key::V,
            KeyboardKey::KEY_W => Key::W,
            KeyboardKey::KEY_X => Key::X,
            KeyboardKey::KEY_Y => Key::Y,
            KeyboardKey::KEY_Z => Key::Z,
            KeyboardKey::KEY_LEFT_BRACKET => Key::OpenBracket,
            KeyboardKey::KEY_BACKSLASH => Key::Backslash,
            KeyboardKey::KEY_RIGHT_BRACKET => Key::CloseBracket,
            KeyboardKey::KEY_GRAVE => Key::Backtick,
            KeyboardKey::KEY_SPACE => Key::Space,
            KeyboardKey::KEY_ESCAPE => Key::Escape,
            KeyboardKey::KEY_ENTER => Key::Enter,
            KeyboardKey::KEY_TAB => Key::Tab,
            KeyboardKey::KEY_BACKSPACE => Key::Backspace,
            KeyboardKey::KEY_INSERT => Key::Insert,
            KeyboardKey::KEY_DELETE => Key::Delete,
            KeyboardKey::KEY_RIGHT => Key::ArrowRight,
            KeyboardKey::KEY_LEFT => Key::ArrowLeft,
            KeyboardKey::KEY_DOWN => Key::ArrowDown,
            KeyboardKey::KEY_UP => Key::ArrowUp,
            KeyboardKey::KEY_PAGE_UP => Key::PageUp,
            KeyboardKey::KEY_PAGE_DOWN => Key::PageDown,
            KeyboardKey::KEY_HOME => Key::Home,
            KeyboardKey::KEY_END => Key::End,
            KeyboardKey::KEY_F1 => Key::F1,
            KeyboardKey::KEY_F2 => Key::F2,
            KeyboardKey::KEY_F3 => Key::F3,
            KeyboardKey::KEY_F4 => Key::F4,
            KeyboardKey::KEY_F5 => Key::F5,
            KeyboardKey::KEY_F6 => Key::F6,
            KeyboardKey::KEY_F7 => Key::F7,
            KeyboardKey::KEY_F8 => Key::F8,
            KeyboardKey::KEY_F9 => Key::F9,
            KeyboardKey::KEY_F10 => Key::F10,
            KeyboardKey::KEY_F11 => Key::F11,
            KeyboardKey::KEY_F12 => Key::F12,
            KeyboardKey::KEY_KP_0 => Key::Num0,
            KeyboardKey::KEY_KP_1 => Key::Num1,
            KeyboardKey::KEY_KP_2 => Key::Num2,
            KeyboardKey::KEY_KP_3 => Key::Num3,
            KeyboardKey::KEY_KP_4 => Key::Num4,
            KeyboardKey::KEY_KP_5 => Key::Num5,
            KeyboardKey::KEY_KP_6 => Key::Num6,
            KeyboardKey::KEY_KP_7 => Key::Num7,
            KeyboardKey::KEY_KP_8 => Key::Num8,
            KeyboardKey::KEY_KP_9 => Key::Num9,
            KeyboardKey::KEY_KP_DECIMAL => Key::Period,
            KeyboardKey::KEY_KP_DIVIDE => Key::Slash,
            KeyboardKey::KEY_KP_SUBTRACT => Key::Minus,
            KeyboardKey::KEY_KP_ADD => Key::Plus,
            KeyboardKey::KEY_KP_ENTER => Key::Enter,
            KeyboardKey::KEY_KP_EQUAL => Key::Equals,
            KeyboardKey::KEY_BACK => Key::Backspace,
            _ => {
                return None;
            }
        };
        Some(v)
    }
}

/// Convert raw image (Uncompressed RGBA) of size `size`, stored in `rgba` into raylib [Image](raylib::texture::Image)
/// # Safety
/// Unsafe behaviour occurs if image created did not allocate enough pixels for RGBA writing.
/// However, this function uses Raylib's `gen_image_color` to allocate an image before writing.
/// Currently, Raylib's `GenImageColor` function will `calloc` for `size[0]*size[1]*4` bytes in RGBA format itself.
/// Thus, hypothetically this function is always safe.
#[allow(dead_code)]
pub fn rl_image_from_rgba(size: [usize; 2], rgba: &[u8]) -> raylib::prelude::Image {
    use raylib::prelude::{Color, Image};
    let mut img = Image::gen_image_color(size[0] as i32, size[1] as i32, Color::BLACK.alpha(0.0));
    img.set_format(raylib::ffi::PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8A8);
    let raw = img.to_raw();
    let len = (raw.width * raw.height * 4) as usize;
    let rawptr = raw.data as *mut u8;
    unsafe {
        std::ptr::copy(rgba.as_ptr(), rawptr, len);
        Image::from_raw(raw)
    }
}
