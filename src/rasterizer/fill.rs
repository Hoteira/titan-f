
pub(crate) fn filler(width: usize, height: usize, windings: &[i16], bitmap: &mut [u8]) {

    if windings.len() == 0 {
        return;
    }

    let mut idx = 0;
    for y in 0..height {
        let mut sum = 0i32;
        for x in 0..width {
            sum += windings[idx] as i32;

            let alpha = (sum.abs() as f32 / 256.0).min(1.0);
            bitmap[idx] = (alpha * 255.0) as u8;
            idx += 1;
        }
    }
}