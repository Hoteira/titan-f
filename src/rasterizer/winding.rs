
#[inline(always)]
pub fn calculate_winding(
    x0: f32, y0: f32,
    x1: f32, y1: f32,
    slope: i8,
    width: usize,
    height_i32: i32,
    width_f: f32,
    winding: &mut [f32]
) {
    let dy = y1 - y0;

    if dy * dy < 1e-12 {
        return;
    }

    let dx = x1 - x0;
    let dir = slope as f32;
    let dy_inv = 1.0 / dy;

    let y_start_f = y0.floor();
    let y_end_f = y1.ceil();
    let y_start = (y_start_f as i32).max(0).min(height_i32);
    let y_end = (y_end_f as i32).max(0).min(height_i32);

    if y_start >= y_end {
        return;
    }

    let x_base = x0 - y0 * dx * dy_inv;
    let x_slope = dx * dy_inv;

    for y in y_start..y_end {
        let y_f = y as f32;
        let y_bottom = y_f;
        let y_top = y_f + 1.0;

        let y_enter = if y0 > y_bottom { y0 } else { y_bottom };
        let y_exit = if y1 < y_top { y1 } else { y_top };
        let coverage = y_exit - y_enter;

        if coverage < 1e-6 {
            continue;
        }

        let y_mid = (y_enter + y_exit) * 0.5;
        let x = x_base + y_mid * x_slope;

        if x >= 0.0 && x < width_f {
            let xi = x as usize;

            if xi < width {
                let idx = (y as usize) * width + xi;

                unsafe {
                    *winding.get_unchecked_mut(idx) += dir * coverage;
                }
            }
        }
    }
}