use crate::rasterizer::point::{Contour, Point};
use crate::F32NoStd;

#[inline(always)]
pub fn make_contour(
    points: &[Contour],
    scale: f32,
    y_max: f32,
    x_min: f32,
    width: usize,
    height: usize,
    winding: &mut [i16]
) {
    if winding.len() == 0 {
        return;
    }

    let scale_y_max = y_max * scale;
    let width_f = width as f32;
    let height_i32 = height as i32;
    let x_offset = x_min * scale;

    for contour in points {
        let num_points = contour.points.len();
        if num_points == 0 { continue; }

        for j in 0..num_points {
            let current = &contour.points[j];
            let next_idx = (j + 1) % num_points;
            let next = &contour.points[next_idx];

            if current.on_curve && next.on_curve {

                let p0x = (current.x as f32 * scale) - x_offset;
                let p0y = scale_y_max - (current.y as f32 * scale);
                let p1x = (next.x as f32 * scale) - x_offset;
                let p1y = scale_y_max - (next.y as f32 * scale);

                let dy = p1y - p0y;
                if dy * dy >= 1e-12 {
                    if p0y < p1y {
                        draw_line_fixed(p0x, p0y, p1x, p1y, 1.0, width, width_f, height_i32, winding);
                    } else {
                        draw_line_fixed(p1x, p1y, p0x, p0y, -1.0, width, width_f, height_i32, winding);
                    }
                }
            } else if !current.on_curve {
                let previous = if j != 0 {
                    &contour.points[j - 1]
                } else {
                    &contour.points[num_points - 1]
                };

                flatten_quadratic_optimized(
                    previous, current, next, scale, scale_y_max, x_offset,
                    width, width_f, height_i32, winding
                );
            }
        }
    }
}

#[inline(always)]
fn flatten_quadratic_optimized(
    p0: &Point, p1: &Point, p2: &Point,
    scale: f32, scale_y_max: f32, x_offset: f32,
    width: usize, width_f: f32, height_i32: i32,
    winding: &mut [i16]
) {
    let p0x = (p0.x as f32 * scale) - x_offset;
    let p0y = scale_y_max - (p0.y as f32 * scale);
    let p1x = (p1.x as f32 * scale) - x_offset;
    let p1y = scale_y_max - (p1.y as f32 * scale);
    let p2x = (p2.x as f32 * scale) - x_offset;
    let p2y = scale_y_max - (p2.y as f32 * scale);

    let curvature = (p1x - (p0x + p2x) * 0.5).abs() + (p1y - (p0y + p2y) * 0.5).abs();
    let steps = (curvature * 0.25).max(1.0) as usize;

    let ax = p0x - 2.0 * p1x + p2x;
    let ay = p0y - 2.0 * p1y + p2y;
    let bx = 2.0 * (p1x - p0x);
    let by = 2.0 * (p1y - p0y);

    let delta = 1.0 / steps as f32;
    let delta2 = delta * delta;
    let ddx = 2.0 * ax * delta2;
    let ddy = 2.0 * ay * delta2;

    let mut dx = bx * delta + ddx * 0.5;
    let mut dy = by * delta + ddy * 0.5;

    let mut prev_x = p0x;
    let mut prev_y = p0y;

    for _ in 0..steps {
        let x = prev_x + dx;
        let y = prev_y + dy;

        let seg_dy = y - prev_y;
        if seg_dy * seg_dy > 1e-12 {
            if prev_y < y {
                draw_line_fixed(prev_x, prev_y, x, y, 1.0, width, width_f, height_i32, winding);
            } else {
                draw_line_fixed(x, y, prev_x, prev_y, -1.0, width, width_f, height_i32, winding);
            }
        }

        prev_x = x;
        prev_y = y;
        dx += ddx;
        dy += ddy;
    }
}

#[inline(always)]
fn draw_line_fixed(
    x0: f32, y0: f32,
    x1: f32, y1: f32,
    dir: f32,
    width: usize,
    width_f: f32,
    height_i32: i32,
    winding: &mut [i16]
) {
    let dy = y1 - y0;
    let dx = x1 - x0;
    let dy_inv = 1.0 / dy;
    let x_per_y = dx * dy_inv;
    let x_base = x0 - y0 * x_per_y;

    let y_start = (y0.floor() as i32).max(0).min(height_i32 - 1);
    let y_end = (y1.ceil() as i32).max(0).min(height_i32);

    let dir_256 = dir * 256.0;
    let width_minus_1_i32 = (width - 1) as i32;

    let mut y = y_start;
    while y < y_end {
        let yf = y as f32;
        let y_lo = if y0 > yf { y0 } else { yf };
        let y_hi = if y1 < yf + 1.0 { y1 } else { yf + 1.0 };
        let coverage_y = y_hi - y_lo;

        let x = x_base + yf * x_per_y;

        if x < -1.0 || x > width_f {
            y += 1;
            continue;
        }

        let frac = x - x.floor();
        let xi0 = x as i32;
        let xi1 = xi0 + 1;

        let cov0 = (1.0 - frac) * coverage_y;
        let cov1 = frac * coverage_y;

        let cov0_fixed = (cov0 * dir_256).round() as i16;
        let cov1_fixed = (cov1 * dir_256).round() as i16;

        let xi0_clamped = xi0.max(0).min(width_minus_1_i32);
        let xi1_clamped = xi1.max(0).min(width_minus_1_i32);

        let y_offset = (y as usize) * width;
        let idx0 = y_offset + xi0_clamped as usize;
        let idx1 = y_offset + xi1_clamped as usize;

        winding[idx0] += cov0_fixed;
        winding[idx1] += cov1_fixed;

        y += 1;
    }
}