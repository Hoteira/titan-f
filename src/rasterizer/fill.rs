use crate::Vec;

pub fn filler(width: usize, height: usize, windings: &[f32], bitmap: &mut [u8]) {
    for y in 0..height {
        let row_start = y * width;
        let mut sum = 0.0;

        for x in 0..width {
            let idx = row_start + x;
            sum += windings[idx];

            if sum.abs() < 0.05 {
                sum = 0.0;
            }

            let alpha = sum.abs().min(1.0);
            bitmap[idx] = (alpha * 255.0) as u8;
        }
    }
}