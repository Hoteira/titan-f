
pub(crate) fn fill_span(bitmap: &mut [u8], y: usize, width: usize, start_x: f32, end_x: f32) {
    let s = start_x.max(0.0);
    let e = end_x.min(width as f32);

    if s >= e {
        return;
    }

    let start_pixel = s.floor() as usize;
    let end_pixel = e.floor() as usize;

    if start_pixel == end_pixel {
        let coverage = (e - s).min(1.0);
        let idx = y * width + start_pixel;
        if start_pixel < width {
            bitmap[idx] = (coverage * 255.0) as u8;
        }
        return;
    }

    let first_coverage = ((start_pixel + 1) as f32 - s).min(1.0);
    let idx = y * width + start_pixel;
    if start_pixel < width {
        bitmap[idx] = (first_coverage * 255.0) as u8;
    }

    for x in (start_pixel + 1)..end_pixel.min(width) {
        let idx = y * width + x;
        bitmap[idx] = 255;
    }

    if end_pixel < width && end_pixel > start_pixel {
        let last_coverage = e - (end_pixel as f32);
        let idx = y * width + end_pixel;
        bitmap[idx] = (last_coverage * 255.0) as u8;
    }
}
