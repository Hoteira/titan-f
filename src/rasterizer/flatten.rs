use crate::rasterizer::point::{Contour, Point};
use crate::Vec;

#[derive(Debug, Copy, Clone)]
pub struct Line {
    pub coords: (f32, f32, f32, f32), // x0, y0, x1, y1
    pub slope: i8,
}

impl Line {
    pub const fn new() -> Line {
        Line { coords: (0.0, 0.0, 0.0, 0.0), slope: 0 }
    }
}

pub fn make_contour(points: &[Contour], scale: f32, y_max: f32, x_min: f32, lines: &mut Vec<Line>) {

    let estimated_lines: usize = points.iter()
        .map(|c| c.points.len() + c.points.len() / 2)
        .sum();

    lines.reserve(estimated_lines);

    for contour in points {
        for j in 0..contour.points.len() {
            let current = &contour.points[j];
            let next = &contour.points[(j + 1) % contour.points.len()];

            if current.on_curve && next.on_curve {
                check_line(current, next, lines, scale, y_max, x_min);
            } else if !current.on_curve {
                let previous = if j != 0 {
                    &contour.points[(j - 1) % contour.points.len()]
                } else {
                    &contour.points[contour.points.len() - 1]
                };

                flatten_quadratic(previous, current, next, lines, scale, y_max, x_min);
            }
        }
    }
}

#[inline(always)]
pub fn check_line(p0: &Point, p1: &Point, lines: &mut Vec<Line>, scale: f32, y_max: f32, x_min: f32) {

    let (p0x, p0y) = scale_point(p0, scale, y_max, x_min);
    let (p1x, p1y) = scale_point(p1, scale, y_max, x_min);

    let (coords, slope) = if (p0x - p0y).abs() < 1e-6 {
        return;
    } else if p0y < p1y {
        ((p0x, p0y, p1x, p1y), 1)
    } else {
        ((p1x, p1y, p0x, p0y), -1)
    };

    lines.push(Line { coords, slope });
}

#[inline(always)]
pub fn flatten_quadratic(p0: &Point, p1: &Point, p2: &Point, lines: &mut Vec<Line>, scale: f32, y_max: f32, x_min: f32) {
    let (p0x, p0y) = scale_point(p0, scale, y_max, x_min);
    let (p1x, p1y) = scale_point(p1, scale, y_max, x_min);
    let (p2x, p2y) = scale_point(p2, scale, y_max, x_min);

    let curvature = ((p1x - (p0x + p2x) * 0.5).abs() + (p1y - (p0y + p2y) * 0.5).abs()) * scale;

    let steps = (curvature * 0.25).clamp(2.0, 16.0) as usize;

    let increment = 1.0 / steps as f32;

    let ax = p0x - 2.0 * p1x + p2x;
    let bx = 2.0 * (p1x - p0x);
    let ay = p0y - 2.0 * p1y + p2y;
    let by = 2.0 * (p1y - p0y);

    let mut base_x = p0x;
    let mut base_y = p0y;

    for i in 1..=steps {
        let t = i as f32 * increment;

        let new_x = (ax * t + bx) * t + p0x;
        let new_y = (ay * t + by) * t + p0y;

        let (coords, slope) = if (base_y - new_y).abs() < 1e-6 {
            return;
        } else if base_y < new_y {
            ((base_x, base_y, new_x, new_y), 1)
        } else {
            ((new_x, new_y, base_x, base_y), -1)
        };

        lines.push(Line { coords, slope });

        base_x = new_x;
        base_y = new_y;
    }
}

#[inline]
fn scale_point(p: &Point, scale: f32, y_max: f32, x_min: f32) -> (f32, f32) {
    ((p.x as f32 - x_min) * scale, (y_max - p.y as f32) * scale)
}