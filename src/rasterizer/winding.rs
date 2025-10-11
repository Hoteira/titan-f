use crate::rasterizer::flatten::Line;
use crate::F32NoStd;

#[inline]
pub fn get_winding(lines: &[Line], width: usize, height: usize, winding_buffer: &mut [f32]) {
    let width_f = width as f32;

    for line in lines {
        calculate_winding(
            line.coords.0, line.coords.1,
            line.coords.2, line.coords.3,
            line.slope,
            width, height,
            width_f,
            winding_buffer
        );
    }
}

#[inline]
fn calculate_winding(
    x0: f32, y0: f32,
    x1: f32, y1: f32,
    slope: i8,
    width: usize,
    height: usize,
    width_f: f32,
    winding: &mut [f32]
) {
    let dy = y1 - y0;

    let dy_inv = 1.0 / dy;
    let dx = x1 - x0;
    let dir = slope as f32;

    let y_start = y0.floor() as i32;
    let y_end = y1.ceil() as i32;

    let y_start = y_start.max(0).min(height as i32);
    let y_end = y_end.max(0).min(height as i32);

    for y in y_start..y_end {
        let y_f = y as f32;
        let y_bottom = y_f;
        let y_top = y_f + 1.0;

        let y_enter = y0.max(y_bottom);
        let y_exit = y1.min(y_top);
        let coverage = y_exit - y_enter;

        let y_mid = (y_enter + y_exit) * 0.5;
        let t = (y_mid - y0) * dy_inv;
        let x = x0 + t * dx;

        if coverage > 1e-6 && x >= 0.0 && x < width_f {
            let xi = x as usize;
            let idx = y as usize * width + xi;

            unsafe {
                *winding.get_unchecked_mut(idx) += dir * coverage;
            }
        }
    }
}