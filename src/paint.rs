//! A module to handle computing the full output, and painting it to screen.

use egui::epaint::tessellator::Path;
use egui::epaint::{ClippedShape, ColorMode, Shape};
use egui::{
    ahash::HashMap, epaint::ImageDelta, output::OutputEvent, Context, FullOutput, OpenUrl,
    RawInput, TextureId,
};
use egui::{Mesh, Vec2};
use raylib::color::Color;
use raylib::drawing::RaylibScissorModeExt;
use raylib::ffi::Rectangle;
use raylib::math::Vector2;
use raylib::RaylibThread;
use raylib::{drawing::RaylibDraw, ffi::MouseCursor, RaylibHandle};

use raylib::texture::Image as rayImage;
use raylib::texture::{RaylibTexture2D, Texture2D as rayTexture};

use crate::util::ConvertRE;

/// Trait to handle egui's platform-specific output.
pub trait PlatformHandler {
    /// Egui wants to open `url`.
    fn open_url(&mut self, url: OpenUrl);
    /// Handle output events sent by Egui.
    fn output_events(&mut self, vec: &[OutputEvent]);
}

fn change_mouse_cursor(rl: &mut RaylibHandle, icon: egui::CursorIcon) {
    let raylib_icon = match icon {
        egui::CursorIcon::Default => MouseCursor::MOUSE_CURSOR_DEFAULT,
        egui::CursorIcon::None => {
            rl.hide_cursor();
            return;
        }
        egui::CursorIcon::ContextMenu => MouseCursor::MOUSE_CURSOR_ARROW,
        egui::CursorIcon::Help => MouseCursor::MOUSE_CURSOR_POINTING_HAND,
        egui::CursorIcon::PointingHand => MouseCursor::MOUSE_CURSOR_POINTING_HAND,
        egui::CursorIcon::Crosshair => MouseCursor::MOUSE_CURSOR_CROSSHAIR,
        egui::CursorIcon::Text => MouseCursor::MOUSE_CURSOR_IBEAM,
        egui::CursorIcon::VerticalText => MouseCursor::MOUSE_CURSOR_IBEAM,
        egui::CursorIcon::NoDrop => MouseCursor::MOUSE_CURSOR_NOT_ALLOWED,
        egui::CursorIcon::NotAllowed => MouseCursor::MOUSE_CURSOR_NOT_ALLOWED,
        egui::CursorIcon::Grab => MouseCursor::MOUSE_CURSOR_ARROW,
        egui::CursorIcon::Grabbing => MouseCursor::MOUSE_CURSOR_POINTING_HAND,
        egui::CursorIcon::ResizeHorizontal => MouseCursor::MOUSE_CURSOR_RESIZE_EW,
        egui::CursorIcon::ResizeNeSw => MouseCursor::MOUSE_CURSOR_RESIZE_NESW,
        egui::CursorIcon::ResizeNwSe => MouseCursor::MOUSE_CURSOR_RESIZE_NWSE,
        egui::CursorIcon::ResizeVertical => MouseCursor::MOUSE_CURSOR_RESIZE_NS,
        egui::CursorIcon::ResizeEast => MouseCursor::MOUSE_CURSOR_RESIZE_EW,
        egui::CursorIcon::ResizeSouthEast => MouseCursor::MOUSE_CURSOR_RESIZE_NWSE,
        egui::CursorIcon::ResizeSouth => MouseCursor::MOUSE_CURSOR_RESIZE_NS,
        egui::CursorIcon::ResizeSouthWest => MouseCursor::MOUSE_CURSOR_RESIZE_NESW,
        egui::CursorIcon::ResizeWest => MouseCursor::MOUSE_CURSOR_RESIZE_EW,
        egui::CursorIcon::ResizeNorthWest => MouseCursor::MOUSE_CURSOR_RESIZE_NWSE,
        egui::CursorIcon::ResizeNorth => MouseCursor::MOUSE_CURSOR_RESIZE_NS,
        egui::CursorIcon::ResizeNorthEast => MouseCursor::MOUSE_CURSOR_RESIZE_NESW,
        egui::CursorIcon::ResizeColumn => MouseCursor::MOUSE_CURSOR_RESIZE_ALL,
        egui::CursorIcon::ResizeRow => MouseCursor::MOUSE_CURSOR_RESIZE_ALL,
        _ => MouseCursor::MOUSE_CURSOR_DEFAULT,
    };
    if rl.is_cursor_hidden() {
        rl.show_cursor();
    }
    rl.set_mouse_cursor(raylib_icon);
}

/// Obtain the full output of `ctx.run`, and process platform outputs.
/// The handler's methods are invoked to handle url-open, or output events sent by egui.
pub fn full_output<F, H>(
    rl: &mut RaylibHandle,
    raw_input: RawInput,
    ctx: &egui::Context,
    run_ui: F,
    handler: &mut H,
) -> FullOutput
where
    F: FnOnce(&Context),
    H: PlatformHandler,
{
    let fout = ctx.run(raw_input, run_ui);
    change_mouse_cursor(rl, fout.platform_output.cursor_icon);
    if !fout.platform_output.copied_text.is_empty() {
        if let Err(e) = rl.set_clipboard_text(&fout.platform_output.copied_text) {
            eprintln!(
                "egui-raylib: Failed to copy text \"{}\" to clipborad,\n\tdetail: {e}",
                fout.platform_output.copied_text
            );
        }
    }
    if let Some(ref s) = fout.platform_output.open_url {
        handler.open_url(s.to_owned())
    }
    handler.output_events(&fout.platform_output.events);
    fout
}

/// Create a raylib image from pixels.
/// Same as [crate::utils::rl_image_from_rgba], except uses slice of pixels instead of an iterator of bytes.
/// # Safety
/// Hypothetically safe, see safety condition for `rl_image_from_rgba`.
fn rimg_from_pixels(size: [usize; 2], pixels: impl Iterator<Item = [u8; 4]>) -> rayImage {
    let mut img =
        rayImage::gen_image_color(size[0] as i32, size[1] as i32, Color::BLACK.alpha(0.0));
    img.set_format(raylib::ffi::PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8A8);
    let raw = img.to_raw();
    let len = (raw.width * raw.height * 4) as usize;
    let mut rawptr = raw.data as *mut u8;

    unsafe {
        for c in pixels.take(len) {
            std::ptr::copy_nonoverlapping(c.as_ptr(), rawptr, 4);
            rawptr = rawptr.wrapping_add(4);
        }
        rayImage::from_raw(raw)
    }
}

#[derive(Default)]
/// Struct to manage [textures](raylib::texture::Texture2D) and handle drawing shapes.
pub(crate) struct Painter {
    textures: HashMap<TextureId, rayTexture>,
    fonttex: Option<TextureId>,
}

fn color_mode_to_color(c: &ColorMode) -> Color {
    match c {
        ColorMode::Solid(c) => c.convert(),
        ColorMode::UV(_) => {
            eprintln!("egui-raylib: UV color mode for paths and lines is not yet implemented! Falling back to WHITE.");
            Color::WHITE
        }
    }
}

impl Painter {
    fn process_image_delta(
        &mut self,
        mapid: TextureId,
        delta: &ImageDelta,
        rthread: &RaylibThread,
        rl: &mut RaylibHandle,
    ) {
        let mut img = match &delta.image {
            egui::ImageData::Color(c) => {
                let px = c.pixels.iter().map(|c| c.to_srgba_unmultiplied());
                rimg_from_pixels(c.size, px)
            }
            egui::ImageData::Font(fontimg) => {
                let px = fontimg
                    .srgba_pixels(None)
                    .map(|c| c.to_srgba_unmultiplied());
                self.fonttex.replace(mapid);
                rimg_from_pixels(fontimg.size, px)
            }
        };
        let tex = match delta.pos {
            Some(pos) => {
                // See if this section of code can be better.
                /* --------------------- */
                let tex = self
                    .textures
                    .get_mut(&mapid)
                    .expect("ImageDelta updates should be accompanied by valid TextureId.");
                let mut old_img = tex
                    .load_image()
                    .expect("You should be able to retrieve image from texture.");
                let size = delta.image.size();
                for x in 0..size[0] {
                    for y in 0..size[1] {
                        old_img.draw_pixel(
                            (x + pos[0]) as i32,
                            (y + pos[1]) as i32,
                            img.get_color(x as i32, y as i32),
                        )
                    }
                }
                /* -------------------- */
                rl.load_texture_from_image(rthread, &old_img).expect(
                    "Image data should easily be sent to GPU. Texture could not be created.",
                )
            }
            None => rl
                .load_texture_from_image(rthread, &img)
                .expect("Image data should easily be sent to GPU. Texture could not be created."),
        };

        let wrap_mode = match delta.options.wrap_mode {
            egui::TextureWrapMode::ClampToEdge => raylib::ffi::TextureWrap::TEXTURE_WRAP_CLAMP,
            egui::TextureWrapMode::Repeat => raylib::ffi::TextureWrap::TEXTURE_WRAP_REPEAT,
            egui::TextureWrapMode::MirroredRepeat => {
                raylib::ffi::TextureWrap::TEXTURE_WRAP_MIRROR_REPEAT
            }
        };
        tex.set_texture_wrap(rthread, wrap_mode);

        // TODO: Figure out how to configure raylib to use different filters for minification and magnification.
        let filter_mode = match delta.options.magnification {
            egui::TextureFilter::Nearest => raylib::ffi::TextureFilter::TEXTURE_FILTER_POINT,
            egui::TextureFilter::Linear => raylib::ffi::TextureFilter::TEXTURE_FILTER_BILINEAR,
        };
        tex.set_texture_filter(rthread, filter_mode);

        self.textures.insert(mapid, tex); // If there was anything here before, it would be dropped.
    }

    fn paint_shape(&self, pxpp: f32, shape: Shape, d: &mut impl RaylibDraw) {
        match shape {
		    egui::Shape::Noop => { /* Do nothing */ },
		    egui::Shape::Vec(v) => {
		    	// Recursively draw out shapes.
		    	for e in v { self.paint_shape(pxpp, e, d); }
		    },
		    egui::Shape::Circle(c) => {
		    	// Draw this shape by drawing two concentric circles.

		    	let center_x = (c.center.x * pxpp) as i32;
		    	let center_y = (c.center.y * pxpp) as i32;
		    	let r2 = c.radius * pxpp;
		    	let r1 = (c.radius + c.stroke.width) * pxpp;

		    	// First draw stroke, then draw the real circle concentric to it.
		    	d.draw_circle(center_x, center_y, r1, c.stroke.color.convert());
		    	d.draw_circle(center_x, center_y, r2, c.fill.convert());
		    },
		    egui::Shape::Ellipse(es) => {
		    	// Similar to circle.

		    	let center_x = (es.center.x * pxpp) as i32;
		    	let center_y = (es.center.y * pxpp) as i32;
		    	let axes1 = es.radius + Vec2::new(es.stroke.width, es.stroke.width);
		    	let axes2 = es.radius;

		    	d.draw_ellipse(center_x, center_y, axes1.x, axes1.y, es.stroke.color.convert());
		    	d.draw_ellipse(center_x, center_y, axes2.x, axes2.y, es.fill.convert());
		    },
		    egui::Shape::LineSegment { points, stroke } => {
		    	let start_pos = points[0].convert().scale_by(pxpp);
		    	let end_pos = points[1].convert().scale_by(pxpp);
		    	let thick = stroke.width * pxpp;
		    	d.draw_line_ex(start_pos, end_pos, thick, color_mode_to_color(&stroke.color))
		    },

		    egui::Shape::Path(ps) => {
                if ps.closed {
                    let mut out = Mesh::default();
                    let mut p = Path::default();
                    let fill = ps.fill.convert();
                    p.add_line_loop(&ps.points);
                    p.fill(0.2, ps.fill, &mut out);
                    for verts in out.indices.chunks_exact(3) {
                        let p0 = out.vertices[verts[0] as usize].pos.convert().scale_by(pxpp);
                        let p1 = out.vertices[verts[1] as usize].pos.convert().scale_by(pxpp);
                        let p2 = out.vertices[verts[2] as usize].pos.convert().scale_by(pxpp);
                        d.draw_triangle(p0, p1, p2, fill);
                    }
                } else {
                    let lines = ps.points.iter()
                        .zip(ps.points.iter().skip(1))
                        .map(|(a,b)| 
                            (a.convert().scale_by(pxpp), 
                             b.convert().scale_by(pxpp))
                            );
                    let thick = ps.stroke.width * pxpp;
                    let color = color_mode_to_color(&ps.stroke.color);

                    for (start_pos, end_pos) in lines {
                        d.draw_line_ex(start_pos, end_pos, thick, color)
                    }
                }
            },

		    egui::Shape::Rect(rs) => {
                // TODO: Implement rounding of edges and blur for drawing `RectShape`
                let rrect = Rectangle {
                    x: rs.rect.min.x * pxpp,
                    y: rs.rect.min.y * pxpp,
                    width: rs.rect.width() * pxpp,
                    height: rs.rect.height() * pxpp,
                };
                let swidth = rs.stroke.width * pxpp;
                let rrect2 = Rectangle {
                    x: rrect.x - swidth,
                    y: rrect.y - swidth,
                    width: rrect.width + 2.0 * swidth,
                    height: rrect.height + 2.0 * swidth
                };
                let fill_color = rs.fill.convert();
                let stroke_color = rs.stroke.color.convert();
                d.draw_rectangle_rec(rrect2, stroke_color);

                if rs.uv == egui::Rect::ZERO {
                    // No texture here.
                    d.draw_rectangle_rec(rrect, fill_color);
                } else {
                    // Draw textured rectangle.
                    if let Some(texture) = self.textures.get(&rs.fill_texture_id) {
                        let source_rec = Rectangle {
                            x: rs.uv.min.x * texture.width as f32,
                            y: rs.uv.max.y * texture.height as f32,
                            width: rs.uv.width(),
                            height: rs.uv.height()
                        };
                        d.draw_texture_pro(texture, source_rec, rrect, Vector2::zero(), 0.0, fill_color)
                    } else {
                        d.draw_rectangle_rec(rrect, fill_color)
                    }
                }
            },

		    egui::Shape::Text(ts) => {
                // TODO: Implement drawing text.
                let origin = Vector2::new(ts.pos.x, ts.pos.y).scale_by(pxpp);
                let font_texture = self.fonttex.and_then(|t| self.textures.get(&t)).expect("Font texture should have been sent as an ImageDelta by now..");

                for row in ts.galley.rows.iter() {
                    for g in row.glyphs.iter() {
                        let color = ts.override_text_color.unwrap_or_else(|| ts.galley.job.sections[g.section_index as usize].format.color);
                        let tint = color.convert();
                        let dst_rect = Rectangle {
                            x: origin.x + (g.pos.x + g.uv_rect.offset.x) * pxpp,
                            y: origin.y + (g.pos.y + g.uv_rect.offset.y) * pxpp,
                            width: g.uv_rect.size.x * pxpp,
                            height: g.uv_rect.size.y * pxpp
                        };
                        let uv_rect = Rectangle {
                            x: g.uv_rect.min[0] as f32,
                            y: g.uv_rect.min[1] as f32,
                            width: (g.uv_rect.max[0] - g.uv_rect.min[0]) as f32,
                            height: (g.uv_rect.max[1] - g.uv_rect.min[1]) as f32,
                        };
                        d.draw_texture_pro(font_texture, uv_rect, dst_rect, Vector2::zero(), 0.0, tint);
                    }
                }

                // d.draw_texture(font_texture, 0, 0, Color::WHITE);
		    },
		    egui::Shape::QuadraticBezier(qbez) => {
		    	let points: [Vector2; 3] = [
		    		qbez.points[0].convert().scale_by(pxpp),
		    		qbez.points[1].convert().scale_by(pxpp),
		    		qbez.points[2].convert().scale_by(pxpp)
		    	];
		    	let thick = qbez.stroke.width * pxpp;
		    	d.draw_spline_bezier_quadratic(points.as_slice(), thick, qbez.fill.convert())
		    },
		    egui::Shape::CubicBezier(cbez) => {
		    	let points: [Vector2; 4] = [
		    		cbez.points[0].convert().scale_by(pxpp),
		    		cbez.points[1].convert().scale_by(pxpp),
		    		cbez.points[2].convert().scale_by(pxpp),
		    		cbez.points[3].convert().scale_by(pxpp)
		    	];
		    	let thick = cbez.stroke.width * pxpp;
		    	d.draw_spline_bezier_cubic(points.as_slice(), thick, cbez.fill.convert());
		    },
		    egui::Shape::Mesh(_) => unimplemented!("Haven't implemented drawing arbitrary meshes as there is no immediately obvious way of doing it using raylib."),
		    egui::Shape::Callback(_) => unimplemented!("Implement support for PaintCallbacks."),
		}
    }

    /// Perform pre-paint steps dealing with loading and freeing textures, then generate shapes.
    pub fn predraw(
        &mut self,
        output: FullOutput,
        rl: &mut RaylibHandle,
        rthread: &RaylibThread,
    ) -> PreparedShapes {
        for (id, delta) in output.textures_delta.set {
            self.process_image_delta(id, &delta, rthread, rl)
        }
        for id in output.textures_delta.free {
            self.textures.remove(&id);
        }
        PreparedShapes {
            shapes: output.shapes,
            pxpp: output.pixels_per_point,
        }
    }

    /// Draw shapes prepared from pre-draw step using handle `d`.
    pub fn paint<D>(
        &self,
        // ctx: &Context,
        prs: PreparedShapes,
        d: &mut D,
    ) where
        D: RaylibDraw + RaylibScissorModeExt,
    {
        let pxpp = prs.pxpp;
        let shapes = prs.shapes;
        // Hereafter everything uses points, instead of pixels.

        for clipped_shape in shapes {
            let cx = (clipped_shape.clip_rect.min.x * pxpp) as i32;
            let cy = (clipped_shape.clip_rect.min.y * pxpp) as i32;
            let cw = (clipped_shape.clip_rect.width() * pxpp) as i32;
            let ch = (clipped_shape.clip_rect.height() * pxpp) as i32;
            {
                let mut d = d.begin_scissor_mode(cx, cy, cw, ch);
                self.paint_shape(pxpp, clipped_shape.shape, &mut d);
            } // Scissor mode ends here on drop.
        }
    }
}

/// A struct to contain all shapes generated by egui after predraw-step.
pub struct PreparedShapes {
    /// All clipped shapes obtained from full-output.
    shapes: Vec<ClippedShape>,
    /// Pixels from point obtained from full-output.
    pxpp: f32,
}
