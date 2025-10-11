use crate::font::TrueTypeFont;
use crate::rasterizer::fill::filler;
use crate::rasterizer::flatten;
use crate::rasterizer::winding::get_winding;
use crate::F32NoStd;

use crate::Vec;
use crate::vec;

#[derive(Clone, Debug)]
pub struct Metrics {
    pub width: usize,
    pub height: usize,
    pub left_side_bearing: isize,
    pub advance_width: usize,
    pub base_line: isize,
}

impl TrueTypeFont {
    pub fn get_char(&mut self, c: char, size: usize) -> (Metrics, Vec<u8>) {

        let dpi = 96.0;
        let pixels = size as f32 * dpi / 72.0;
        let scale = pixels / self.head.units_per_em as f32;
        let id = self.glyph_id_table.get(&c).unwrap_or(&0);

        if self.can_cache {
            let is_cached = self.cache.get(*id, size);

            if is_cached.is_some() {
                return is_cached.unwrap().clone();
            }
        }


        let glyph = self.glyph_data_table.get(&id).unwrap_or(self.glyph_data_table.get(&0).unwrap());
        let mut lines = Vec::new();
        lines.reserve(glyph.points.len() * 2);

        flatten::make_contour(
            &glyph.points,
            scale,
            glyph.y_max as f32,
            glyph.x_min as f32,
            &mut lines,
        );

        let width = ((glyph.x_max - glyph.x_min) as f32 * scale).ceil() as usize;
        let height = ((glyph.y_max - glyph.y_min) as f32 * scale).ceil() as usize;
        let baseline = (glyph.y_min as f32 * scale) as isize;
        let mut winding_buffer = vec![0.0_f32; width * height];

        let extra = self.get_metrics(id, scale);
        let metrics = Metrics {
            width,
            height,
            advance_width: extra.0,
            left_side_bearing: extra.1,
            base_line: baseline,
        };

        get_winding(&lines, width, height, &mut winding_buffer);

        let bitmap = filler(width, height, &winding_buffer);

        if self.can_cache {
            self.cache.set(*id, size, metrics.clone(), bitmap.clone());
        }

        (metrics, bitmap)
    }
}


