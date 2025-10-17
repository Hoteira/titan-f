use crate::rasterizer::fill::fill_span;
use crate::rasterizer::flatten::{add_line_edge, flatten_quadratic};
use crate::rasterizer::point::Contour;

pub(crate) struct Edge {
    pub(crate) x: f32,
    pub(crate) dx: f32,
    pub(crate) y_min: f32,
    pub(crate) y_max: f32,
    pub(crate) wind: i32,
}

pub(crate) fn rasterize(
    contours: &[Contour],
    scale: f32,
    y_max: f32,
    x_min: f32,
    width: usize,
    height: usize,
    bitmap: &mut [u8]
) {
    let scale_y_max = y_max * scale;
    let x_offset = x_min * scale;

    let mut edges = Vec::new();

    for contour in contours {
        let num_points = contour.points.len();
        if num_points < 2 {
            continue;
        }

        let mut i = 0;
        while i < num_points {
            let current = &contour.points[i];
            let next_idx = (i + 1) % num_points;
            let next = &contour.points[next_idx];

            let curr_x = (current.x as f32 * scale) - x_offset;
            let curr_y = scale_y_max - (current.y as f32 * scale);
            let next_x = (next.x as f32 * scale) - x_offset;
            let next_y = scale_y_max - (next.y as f32 * scale);

            if current.on_curve && next.on_curve {
                add_line_edge(&mut edges, curr_x, curr_y, next_x, next_y);
                i += 1;
            } else if current.on_curve && !next.on_curve {
                let next_next_idx = (i + 2) % num_points;
                let next_next = &contour.points[next_next_idx];

                let control_x = next_x;
                let control_y = next_y;
                let end_x = (next_next.x as f32 * scale) - x_offset;
                let end_y = scale_y_max - (next_next.y as f32 * scale);

                let dx1 = control_x - curr_x;
                let dy1 = control_y - curr_y;
                let dx2 = end_x - curr_x;
                let dy2 = end_y - curr_y;

                let line_len_sq = dx2 * dx2 + dy2 * dy2;
                let perp_dist = if line_len_sq > 1e-6 {
                    ((dx1 * dy2 - dy1 * dx2).abs() / line_len_sq.sqrt()).abs()
                } else {
                    0.0
                };

                if perp_dist < 0.5 {
                    add_line_edge(&mut edges, curr_x, curr_y, end_x, end_y);
                } else {
                    flatten_quadratic(&mut edges, curr_x, curr_y, control_x, control_y, end_x, end_y);
                }
                i += 2;
            } else {
                i += 1;
            }
        }
    }

    edges.sort_by(|a, b| a.y_min.partial_cmp(&b.y_min).unwrap());

    let mut active_edges: Vec<Edge> = Vec::new();
    let mut edge_index = 0;

    for y in 0..height {
        let yf = y as f32;

        active_edges.retain(|edge| edge.y_max > yf);

        while edge_index < edges.len() && edges[edge_index].y_min <= yf {
            let edge = &edges[edge_index];
            if edge.y_max > yf {
                let x_at_y = edge.x + (yf - edge.y_min) * edge.dx;
                active_edges.push(Edge {
                    x: x_at_y,
                    dx: edge.dx,
                    y_min: edge.y_min,
                    y_max: edge.y_max,
                    wind: edge.wind,
                });
            }
            edge_index += 1;
        }

        active_edges.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

        let mut winding = 0_i32;
        let mut i = 0;

        while i < active_edges.len() {
            let prev_winding = winding;
            winding += active_edges[i].wind;

            if prev_winding == 0 && winding != 0 {
                let start_x = active_edges[i].x;

                let mut j = i + 1;
                while j < active_edges.len() {
                    winding += active_edges[j].wind;
                    if winding == 0 {
                        break;
                    }
                    j += 1;
                }

                if j < active_edges.len() {
                    let end_x = active_edges[j].x;
                    fill_span(bitmap, y, width, start_x, end_x);
                }

                i = j;
            }

            i += 1;
        }

        for edge in &mut active_edges {
            edge.x += edge.dx;
        }
    }
}
