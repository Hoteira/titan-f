use crate::tables::glyf::{
    CompositeComponent,
    Glyph,
    ProtoGlyph,
    ARGS_ARE_XY_VALUES,
    WE_HAVE_AN_X_AND_Y_SCALE,
    WE_HAVE_A_SCALE,
    WE_HAVE_A_TWO_BY_TWO
};

use crate::Vec;
use crate::F32NoStd;
use crate::font::TrueTypeFont;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: i16,
    pub y: i16,
    pub on_curve: bool,
}

#[derive(Debug, Clone)]
pub struct Contour {
    pub points: Vec<Point>,
}

impl Contour {
    pub fn new(size: usize) -> Self {
        Contour { points: Vec::with_capacity(size) }
    }
}

impl TrueTypeFont {
    pub fn load_points(&self, glyph: &mut ProtoGlyph, font: &TrueTypeFont, font_bytes: &[u8]) -> Glyph {
        match glyph {
            ProtoGlyph::Simple(g) => {
                let num_points = g.end_pts_of_contours.last().map(|&e| (e + 1) as usize).unwrap_or(0);
                let expanded_flags = expand_flags(&g.flags, num_points);

                g.points.reserve(g.end_pts_of_contours.len());

                let mut contour_start = 0;
                for i in 0..g.end_pts_of_contours.len() {
                    let contour_size = if i == 0 {
                        g.end_pts_of_contours[i] as usize + 1
                    } else {
                        (g.end_pts_of_contours[i] - g.end_pts_of_contours[i - 1]) as usize
                    };
                    // Add +1 to capacity for the closing point
                    let mut contour = Contour::new(contour_size + 1);

                    for j in contour_start..=g.end_pts_of_contours[i] as usize {
                        contour.points.push(Point {
                            x: g.x_coordinates[j],
                            y: g.y_coordinates[j],
                            on_curve: (expanded_flags[j] & 0x01) != 0,
                        });
                    }

                    // Append the first point as the last point to close the contour
                    if !contour.points.is_empty() {
                        let first_point = contour.points[0];
                        contour.points.push(first_point);
                    }

                    contour_start = g.end_pts_of_contours[i] as usize + 1;
                    g.points.push(contour);
                }

                insert_midpoints(&mut g.points);
            }

            ProtoGlyph::Composite(g) => {
                load_from_parent(&mut g.points, &g.components, font, font_bytes);
                insert_midpoints(&mut g.points);
            }

            ProtoGlyph::Empty => {}
        }

        glyph.finalize()
    }
}

pub fn load_from_parent(master: &mut Vec<Contour>, comps: &Vec<CompositeComponent>, font: &TrueTypeFont, font_bytes: &[u8]) {
    for component in comps.iter() {
        let real_glyph = &mut font.get_glyph(font_bytes, component.glyph_index as u32);

        match real_glyph {
            ProtoGlyph::Simple(g) => {
                let num_points = g.end_pts_of_contours.last().map(|&e| (e + 1) as usize).unwrap_or(0);
                let expanded_flags = expand_flags(&g.flags, num_points);

                g.points.reserve(g.end_pts_of_contours.len());

                let mut contour_start = 0;
                for i in 0..g.end_pts_of_contours.len() {
                    let contour_size = if i == 0 {
                        g.end_pts_of_contours[i] as usize + 1
                    } else {
                        (g.end_pts_of_contours[i] - g.end_pts_of_contours[i - 1]) as usize
                    };
                    
                    let mut contour = Contour::new(contour_size + 1);

                    for j in contour_start..=g.end_pts_of_contours[i] as usize {
                        contour.points.push(Point {
                            x: g.x_coordinates[j],
                            y: g.y_coordinates[j],
                            on_curve: expanded_flags[j] & 0x01 != 0,
                        });
                    }
                    
                    if !contour.points.is_empty() {
                        let first_point = contour.points[0];
                        contour.points.push(first_point);
                    }

                    transform_points(&mut contour.points, component);

                    contour_start = g.end_pts_of_contours[i] as usize + 1;
                    master.push(contour);
                }
            }

            ProtoGlyph::Composite(g) => {
                load_from_parent(master, &g.components, font, font_bytes);
            }

            ProtoGlyph::Empty => {}
        }
    }
}

pub fn insert_midpoints(points: &mut Vec<Contour>) {
    for contour in points.iter_mut() {

        if contour.points.len() <= 1 {
            continue;
        }

        let mut c = 0;
        while c < contour.points.len() {
            let len = contour.points.len();
            let next_idx = (c + 1) % len;

            if !contour.points[c].on_curve && !contour.points[next_idx].on_curve {
                let x = (contour.points[c].x + contour.points[next_idx].x) / 2;
                let y = (contour.points[c].y + contour.points[next_idx].y) / 2;
                let midpoint = Point {
                    x,
                    y,
                    on_curve: true,
                };

                contour.points.insert(c + 1, midpoint);
                c += 2;
            } else {
                c += 1;
            }
        }
    }
}


fn transform_points(points: &mut [Point], component: &CompositeComponent) {
    let (x_scale, y_scale, scale_01, scale_10) = if component.flags & WE_HAVE_A_TWO_BY_TWO != 0 {
        (
            component.x_scale.unwrap_or(1.0),
            component.y_scale.unwrap_or(1.0),
            component.scale_01.unwrap_or(0.0),
            component.scale_10.unwrap_or(0.0),
        )
    } else if component.flags & WE_HAVE_AN_X_AND_Y_SCALE != 0 {
        (component.x_scale.unwrap_or(1.0), component.y_scale.unwrap_or(1.0), 0.0, 0.0)
    } else if component.flags & WE_HAVE_A_SCALE != 0 {
        let s = component.scale.unwrap_or(1.0);
        (s, s, 0.0, 0.0)
    } else {
        (1.0, 1.0, 0.0, 0.0)
    };

    for p in points.iter_mut() {
        let old_x = p.x as f32;
        let old_y = p.y as f32;
        let nx = old_x * x_scale + old_y * scale_10;
        let ny = old_x * scale_01 + old_y * y_scale;
        p.x = nx.round() as i16;
        p.y = ny.round() as i16;
    }

    if component.flags & ARGS_ARE_XY_VALUES != 0 {
        let dx = component.argument1;
        let dy = component.argument2;
        for p in points.iter_mut() {
            p.x = p.x.wrapping_add(dx);
            p.y = p.y.wrapping_add(dy);
        }
    }
}

fn expand_flags(raw_flags: &[u8], num_points: usize) -> Vec<u8> {
    let mut expanded = Vec::with_capacity(num_points);
    let mut i = 0;
    while expanded.len() < num_points && i < raw_flags.len() {
        let flag = raw_flags[i];
        expanded.push(flag);
        i += 1;

        if flag & 0x08 != 0 {
            if i >= raw_flags.len() {
                break;
            }
            let repeat = raw_flags[i] as usize;
            i += 1;
            for _ in 0..repeat {
                if expanded.len() >= num_points {
                    break;
                }
                expanded.push(flag);
            }
        }
    }

    expanded
}