// src/lib.rs

// #![no_std]

// #[panic_handler]
// fn handle_panic(_: &core::panic::PanicInfo) -> ! {
//     loop {}
// }

use core::sync::atomic::{AtomicU32, Ordering};
use std::f32::consts::PI;

static FRAME: AtomicU32 = AtomicU32::new(0);

const WIDTH: usize = 400;
const HEIGHT: usize = 400;

#[no_mangle]
static mut BUFFER: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];

#[no_mangle]
pub unsafe extern "C" fn go(color_scheme: usize) {
    // This is called from JavaScript, and should *only* be
    // called from JavaScript. If you maintain that condition,
    // then we know that the &mut we're about to produce is
    // unique, and therefore safe.
    render_frame_safe(&mut BUFFER, color_scheme)
}

fn map_range(from_range: (f32, f32), to_range: (f32, f32), s: f32) -> f32 {
    // This function is used to convert between
    // different coordinate and color formats
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

// We split this out so that we can escape 'unsafe' as quickly
// as possible.
fn render_frame_safe(buffer: &mut [u32; WIDTH * HEIGHT], color_scheme: usize) {
    let f = FRAME.fetch_add(1, Ordering::Relaxed);
    let uv_range: (f32, f32) = (-0.5, 0.5);
    let x_coord_range: (f32, f32) = (0.0, WIDTH as f32);
    let y_coord_range: (f32, f32) = (0.0, HEIGHT as f32);
    let rgb_unit_range: (f32, f32) = (-1.0, 1.0);
    let rgb_range: (f32, f32) = (0.0, 255.0);

    let ff: f32 = f as f32 / 16.;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let x_unit: f32 = map_range(x_coord_range, uv_range, x as f32);
            let y_unit: f32 = map_range(y_coord_range, uv_range, y as f32);
            let cx: f32 = x_unit + 0.5 * (ff / 5.).sin();
            let cy: f32 = y_unit + 0.5 * (ff / 3.).cos();

            let mut v = (x_unit * 10. + ff).sin();
            v = v + (10. * (x_unit * (ff / 2.).sin() + y_unit * (ff / 3.).cos()) + ff).sin();
            v = v + ((100. * (cx.powi(2) + cy.powi(2)) + 1.).sqrt() + ff).sin();
            // v = v / 2.;

            let (red, green, blue): (f32, f32, f32) = match color_scheme {
                1 => (
                    // Color scheme #1
                    (v * PI).sin(),
                    (v * PI).cos(),
                    -1.0,
                ),
                2 => (
                    // Color scheme #2
                    1.0,
                    (v * PI).cos(),
                    (v * PI).sin(),
                ),
                // Color scheme #3
                3 => (
                    (v * PI).sin(),
                    (v * PI + 2. * PI / 3.).sin(),
                    (v * PI + 4. * PI / 3.).sin(),
                ),
                _ => (
                    // Color scheme #4
                    (v * PI * 5.).sin(),
                    (v * PI * 5.).sin(),
                    (v * PI * 5.).sin(),
                ),
            };
            let red: u32 = (map_range(rgb_unit_range, rgb_range, red)) as u32;
            let green: u32 = (map_range(rgb_unit_range, rgb_range, green) as u32) << 8;
            let blue: u32 = (map_range(rgb_unit_range, rgb_range, blue) as u32) << 16;

            buffer[y * WIDTH + x] = red | green | blue | 0xFF_00_00_00;
        }
    }
}
