#![deny(clippy::all)]
#![forbid(unsafe_code)]

use error_iter::ErrorIter as _;
use log::error;
use piet_common::RenderContext;
use pixels::{Error, Pixels, SurfaceTexture};
use std::time::Instant;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;

// struct ResizeableBitmap<'a> {
//     bitmap: Option<piet_common::BitmapTarget<'a>>,
//     device: piet_common::Device,
// }

// impl<'a> ResizeableBitmap<'a> {
//     fn new() -> Self {
//         let mut device = piet_common::Device::new().unwrap();
//         Self {
//             bitmap: None,//device.bitmap_target(1usize, 1_usize, 1.0).unwrap(),
//             device,
//         }
//     }
// }

// use std::sync::LazyLock;

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Winit + Piet")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)?;

    let mut now = Instant::now();

    let mut device = piet_common::Device::new().unwrap();

    // let mut bitmap = device.bitmap_target(WIDTH as usize, HEIGHT as usize, 1.0).unwrap();

    event_loop.run(move |event, _, control_flow| {
        let width = window.inner_size().width;
        let height = window.inner_size().height;
        let mut bitmap = device
            .bitmap_target(width as usize, height as usize, window.scale_factor())
            .unwrap();

        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            draw_piet_canvas(&mut bitmap, width as f64, height as f64);

            _ = bitmap.copy_raw_pixels(
                piet_common::ImageFormat::RgbaPremul,
                &mut pixels.frame_mut(),
            );

            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                *control_flow = ControlFlow::Exit;
                return;
            }

            let then = now;
            now = Instant::now();
            println!("Time since last frame: {:?}", now - then);
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                // bitmap = device.bitmap_target(size.width as usize, size.height as usize,1.0).unwrap();

                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_error("pixels.resize_surface", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                if let Err(err) = pixels.resize_buffer(size.width, size.height) {
                    log_error("pixels.resize_buffer", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Update internal state and request a redraw
            // shapes.draw(now.elapsed().as_secs_f32());
            window.request_redraw();
        }
    });
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}

use piet_common::kurbo::{Affine, BezPath, Point, Rect, Size};
use piet_common::{Color, FontFamily, ImageFormat, InterpolationMode, Text, TextLayoutBuilder};

fn draw_piet_canvas(bitmap: &mut piet_common::BitmapTarget, width: f64, height: f64) {
    let mut ctx = bitmap.render_context();

    let data = "Hello from Piet + Winit";
    let size = Size::new(width, height);
    let rect = size.to_rect();
    ctx.fill(rect, &Color::WHITE);

    // // We can paint with a Z index, this indicates that this code will be run
    // // after the rest of the painting. Painting with z-index is done in order,
    // // so first everything with z-index 1 is painted and then with z-index 2 etc.
    // // As you can see this(red) curve is drawn on top of the green curve
    // ctx.paint_with_z_index(1, move |ctx| {
    let mut path = BezPath::new();
    path.move_to((0.0, size.height));
    path.quad_to((40.0, 50.0), (size.width, 0.0));
    // Create a color
    let stroke_color = Color::rgb8(128, 0, 0);
    // Stroke the path with thickness 1.0
    ctx.stroke(path, &stroke_color, 5.0);
    // });

    // Create an arbitrary bezier path
    let mut path = BezPath::new();
    path.move_to(Point::ORIGIN);
    path.quad_to((40.0, 50.0), (size.width, size.height));
    // Create a color
    let stroke_color = Color::rgb8(0, 128, 0);
    // Stroke the path with thickness 5.0
    ctx.stroke(path, &stroke_color, 5.0);

    // Rectangles: the path for practical people
    let rect = Rect::from_origin_size((10.0, 10.0), (100.0, 100.0));
    // Note the Color:rgba8 which includes an alpha channel (7F in this case)
    let fill_color = Color::rgba8(0x00, 0x00, 0x00, 0x7F);
    ctx.fill(rect, &fill_color);

    // // Text is easy; in real use TextLayout should either be stored in the
    // // widget and reused, or a label child widget to manage it all.
    // // This is one way of doing it, you can also use a builder-style way.
    // let mut layout = TextLayout::from_text(data);
    // layout.set_font(FontDescriptor::new(FontFamily::SERIF).with_size(24.0));
    // layout.set_text_color(fill_color);
    // layout.rebuild_if_needed(ctx.text(), 8);
    let layout = ctx
        .text()
        .new_text_layout(data.clone())
        .font(FontFamily::SANS_SERIF, 24.0)
        .text_color(fill_color) //Color::rgb8(128, 0, 0))
        .build()
        .unwrap();

    // // Let's rotate our text slightly. First we save our current (default) context:
    _ = ctx.with_save(|ctx| {
        // Now we can rotate the context (or set a clip path, for instance):
        // This makes it so that anything drawn after this (in the closure) is
        // transformed.
        // The transformation is in radians, but be aware it transforms the canvas,
        // not just the part you are drawing. So we draw at (80.0, 40.0) on the rotated
        // canvas, this is NOT the same position as (80.0, 40.0) on the original canvas.
        ctx.transform(Affine::rotate(std::f64::consts::FRAC_PI_4));
        ctx.draw_text(&layout, (80.0, 40.0));
        Ok(())
    });
    // // When we exit with_save, the original context's rotation is restored

    // This is the builder-style way of drawing text.
    let text = ctx.text();
    let layout = text
        .new_text_layout(data.clone())
        .font(FontFamily::SANS_SERIF, 24.0)
        .text_color(Color::rgb8(128, 0, 0))
        .build()
        .unwrap();
    ctx.draw_text(&layout, (100.0, 25.0));

    // Let's burn some CPU to make a (partially transparent) image buffer
    let image_data = make_image_data(256, 256);
    let image = ctx
        .make_image(256, 256, &image_data, ImageFormat::RgbaSeparate)
        .unwrap();
    // The image is automatically scaled to fit the rect you pass to draw_image
    ctx.draw_image(&image, size.to_rect(), InterpolationMode::Bilinear);

    _ = ctx.finish();
}

fn make_image_data(width: usize, height: usize) -> Vec<u8> {
    let mut result = vec![0; width * height * 4];
    for y in 0..height {
        for x in 0..width {
            let ix = (y * width + x) * 4;
            result[ix] = x as u8;
            result[ix + 1] = y as u8;
            result[ix + 2] = !(x as u8);
            result[ix + 3] = 127;
        }
    }
    result
}
