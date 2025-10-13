use crate::rasterizer::point::{Contour, Point};
use crate::rasterizer::winding;

pub fn make_contour(points: &[Contour], scale: f32, y_max: f32, x_min: f32, width: usize, height: usize, winding: &mut [f32]) {

    for contour in points {
        for j in 0..contour.points.len() {
            let current = &contour.points[j];
            let next = &contour.points[(j + 1) % contour.points.len()];

            if current.on_curve && next.on_curve {
                check_line(current, next, scale, y_max, x_min, width, height, winding);
            } else if !current.on_curve {
                let previous = if j != 0 {
                    &contour.points[(j - 1) % contour.points.len()]
                } else {
                    &contour.points[contour.points.len() - 1]
                };

                flatten_quadratic(previous, current, next, scale, y_max * scale, x_min, width, height, winding);
            }
        }
    }
}

#[inline(always)]
pub fn check_line(p0: &Point, p1: &Point, scale: f32, y_max: f32, x_min: f32, width: usize, height: usize, winding: &mut [f32]) {

    let p0x = (p0.x as f32 - x_min) * scale;
    let p0y = (y_max - p0.y as f32) * scale;
    let p1x = (p1.x as f32 - x_min) * scale;
    let p1y = (y_max - p1.y as f32) * scale;

    if (p0y - p1y).abs() < 1e-6 {
        return;
    } else if p0y < p1y {
        winding::calculate_winding(p0x, p0y, p1x, p1y, 1, width, height as i32, width as f32, winding);
    } else {
        winding::calculate_winding(p1x, p1y, p0x, p0y, -1, width, height as i32, width as f32, winding);
    };
}

#[inline(always)]
pub fn flatten_quadratic(
    p0: &Point, p1: &Point, p2: &Point,
    scale: f32,
    scale_y_max: f32,
    x_min: f32,
    width: usize,
    height: usize,
    winding: &mut [f32]
) {
    let p0x = (p0.x as f32 - x_min) * scale;
    let p0y = scale_y_max - (p0.y as f32 * scale);
    let p1x = (p1.x as f32 - x_min) * scale;
    let p1y = scale_y_max - (p1.y as f32 * scale);
    let p2x = (p2.x as f32 - x_min) * scale;
    let p2y = scale_y_max - (p2.y as f32 * scale);

    let curvature = (p1x - (p0x + p2x) * 0.5).abs() + (p1y - (p0y + p2y) * 0.5).abs();

    let steps = (curvature * 0.25).max(2.0) as usize;

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

    let mut x = p0x;
    let mut y = p0y;

    for _ in 0..steps {
        let prev_x = x;
        let prev_y = y;

        x += dx;
        y += dy;
        dx += ddx;
        dy += ddy;

        let diff_y = prev_y - y;
        if diff_y * diff_y > 1e-12 {

            if prev_y < y {
                winding::calculate_winding(prev_x, prev_y, x, y, 1, width, height as i32, width as f32, winding);
            } else {
                winding::calculate_winding(x, y, prev_x, prev_y, -1, width, height as i32, width as f32, winding);
            };
        }
    }
}