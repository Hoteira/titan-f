use crate::font::TrueTypeFont;
use crate::rasterizer::fill::filler;
use crate::F32NoStd;
use crate::rasterizer::flatten;
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
    pub fn get_char<const CACHE: bool>(&mut self, c: char, size: usize) -> (Metrics, Vec<u8>) {

        let dpi = 96.0;
        let pixels = size as f32 * dpi / 72.0;
        let scale = pixels / self.head.units_per_em as f32;

        let id = self.glyph_id_table.get(&c).unwrap_or(&0);

        if CACHE {
            let is_cached = self.cache.get(*id, size);
            if let Some(cached) = is_cached {
                return cached.clone();
            }
        }

        let glyph = self
            .glyph_data_table
            .get(&id)
            .unwrap_or(self.glyph_data_table.get(&0).unwrap());

        let width = (((glyph.x_max - glyph.x_min) as f32 * scale) as usize) + 2;
        let height = (((glyph.y_max - glyph.y_min) as f32 * scale) as usize) + 2;
        let baseline = -(glyph.y_max as f32 * scale) as isize;

        let required_size = width * height;

        if self.winding_buffer.len() < required_size {
            self.winding_buffer.resize(required_size, 0);
        } else {
            self.winding_buffer[..required_size].fill(0);
        }

        flatten::make_contour(
            &glyph.points,
            scale,
            glyph.y_max as f32,
            glyph.x_min as f32,
            width,
            height,
            &mut self.winding_buffer,
        );

        let extra = self.get_metrics(id, scale);
        let metrics = Metrics {
            width,
            height,
            advance_width: extra.0,
            left_side_bearing: extra.1,
            base_line: baseline,
        };

        if self.bitmap_buffer.len() < required_size {
            self.bitmap_buffer.resize(required_size, 0);
        } else {
            self.bitmap_buffer[..required_size].fill(0);
        }

        filler(width, height, &self.winding_buffer, &mut self.bitmap_buffer);

        if CACHE {
            self.cache.set(*id, size, metrics.clone(), self.bitmap_buffer.clone());
        }

        (metrics, self.bitmap_buffer.clone())
    }
}


