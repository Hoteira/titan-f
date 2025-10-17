use crate::rasterizer::aet::Edge;

pub(crate) fn flatten_quadratic(
    edges: &mut Vec<Edge>,
    p0x: f32, p0y: f32,
    p1x: f32, p1y: f32,
    p2x: f32, p2y: f32,
) {
    let mut points = Vec::new();
    points.push((p0x, p0y));

    let mut stack = vec![(p0x, p0y, p1x, p1y, p2x, p2y)];

    while let Some((ax, ay, bx, by, cx, cy)) = stack.pop() {
        let chord_x = cx - ax;
        let chord_y = cy - ay;
        let chord_sq = chord_x * chord_x + chord_y * chord_y;

        if chord_sq < 1e-12 {
            continue;
        }

        let control_x = bx - ax;
        let control_y = by - ay;
        let t = (control_x * chord_x + control_y * chord_y) / chord_sq;
        let proj_x = ax + t * chord_x;
        let proj_y = ay + t * chord_y;
        let dx = bx - proj_x;
        let dy = by - proj_y;
        let dist_sq = dx * dx + dy * dy;

        if dist_sq <= 0.0625 {
            if points.is_empty() || (points.last().unwrap().0 - cx).abs() > 1e-6 || (points.last().unwrap().1 - cy).abs() > 1e-6 {
                points.push((cx, cy));
            }
        } else {
            let mid_ab_x = (ax + bx) * 0.5;
            let mid_ab_y = (ay + by) * 0.5;
            let mid_bc_x = (bx + cx) * 0.5;
            let mid_bc_y = (by + cy) * 0.5;
            let mid_x = (mid_ab_x + mid_bc_x) * 0.5;
            let mid_y = (mid_ab_y + mid_bc_y) * 0.5;

            stack.push((mid_x, mid_y, mid_bc_x, mid_bc_y, cx, cy));
            stack.push((ax, ay, mid_ab_x, mid_ab_y, mid_x, mid_y));
        }
    }

    for i in 0..points.len() - 1 {
        add_line_edge(edges, points[i].0, points[i].1, points[i + 1].0, points[i + 1].1);
    }
}

pub(crate) fn add_line_edge(edges: &mut Vec<Edge>, x0: f32, y0: f32, x1: f32, y1: f32) {
    if (y1 - y0).abs() < 0.001 {
        return;
    }

    let (y_start, y_end, x_start, wind) = if y0 < y1 {
        (y0, y1, x0, 1)
    } else {
        (y1, y0, x1, -1)
    };

    let dx = (x1 - x0) / (y1 - y0);

    edges.push(Edge {
        x: x_start,
        dx,
        y_min: y_start,
        y_max: y_end,
        wind,
    });
}
