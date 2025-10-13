use std::time::Instant;
use crate::font::TrueTypeFont;
use crate::rasterizer::fill::filler;
use crate::rasterizer::flatten;
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
        self.winding_buffer.clear();
        self.bitmap_buffer.clear();

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

        let width = ((glyph.x_max - glyph.x_min) as f32 * scale).ceil() as usize;
        let height = ((glyph.y_max - glyph.y_min) as f32 * scale).ceil() as usize;
        let baseline = (glyph.y_min as f32 * scale) as isize;

        self.winding_buffer.resize(width * height, 0.0);

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

        filler(width, height, &self.winding_buffer, &mut self.bitmap_buffer);

        if self.can_cache {
            self.cache.set(*id, size, metrics.clone(), self.bitmap_buffer.clone());
        }

        (metrics, self.bitmap_buffer.clone())
    }



    pub fn get_char_test<const CACHE: bool>(&mut self, c: char, size: usize) -> (Metrics, Vec<u8>) {
        self.winding_buffer.clear();
        self.bitmap_buffer.clear();

        let start_total = Instant::now();

        let dpi = 96.0;
        let pixels = size as f32 * dpi / 72.0;
        let scale = pixels / self.head.units_per_em as f32;

        let id = self.glyph_id_table.get(&c).unwrap_or(&0);

        let start_cache = Instant::now();
        if CACHE {
            let is_cached = self.cache.get(*id, size);
            if let Some(cached) = is_cached {
                let elapsed_total = start_total.elapsed().as_nanos();
                println!("[get_char_test] Total time: {elapsed_total} ns (cache hit)");
                return cached.clone();
            }
        }
        let time_cache = start_cache.elapsed().as_nanos();

        let start_lookup = Instant::now();
        let glyph = self
            .glyph_data_table
            .get(&id)
            .unwrap_or(self.glyph_data_table.get(&0).unwrap());

        let time_lookup = start_lookup.elapsed().as_nanos();

        //Dunno why this takes 2us ALONE
        let start_sizecalc = Instant::now();
        let width = ((glyph.x_max - glyph.x_min) as f32 * scale).ceil() as usize;
        let height = ((glyph.y_max - glyph.y_min) as f32 * scale).ceil() as usize;
        let baseline = (glyph.y_min as f32 * scale) as isize;
        let time_sizecalc = start_sizecalc.elapsed().as_nanos();

        self.winding_buffer = vec![0.0f32; width * height];

        let start_flatten = Instant::now();
        flatten::make_contour(
            &glyph.points,
            scale,
            glyph.y_max as f32,
            glyph.x_min as f32,
            width,
            height,
            &mut self.winding_buffer,
        );
        let time_flatten = start_flatten.elapsed().as_nanos();

        let start_metrics = Instant::now();
        let extra = self.get_metrics(id, scale);
        let metrics = Metrics {
            width,
            height,
            advance_width: extra.0,
            left_side_bearing: extra.1,
            base_line: baseline,
        };
        let time_metrics = start_metrics.elapsed().as_nanos();

        let start_fill = Instant::now();
        filler(width, height, &self.winding_buffer, &mut self.bitmap_buffer);
        let time_fill = start_fill.elapsed().as_nanos();

        let start_cache_write = Instant::now();
        if CACHE {
            self.cache.set(*id, size, metrics.clone(), self.bitmap_buffer.clone());
        }
        let time_cache_write = start_cache_write.elapsed().as_nanos();

        let total_elapsed = start_total.elapsed().as_nanos();

        let subtotal = (time_cache
            + time_lookup
            + time_sizecalc
            + time_flatten
            + time_metrics
            + time_fill
            + time_cache_write)
            as f64;

        let percent = |x: u128| if subtotal > 0.0 { (x as f64 / subtotal) * 100.0 } else { 0.0 };

        println!("--- [get_char_test] Timing (ns) ---");
        println!("cache lookup:      {:6} ns ({:5.1}%)", time_cache, percent(time_cache));
        println!("glyph lookup:      {:6} ns ({:5.1}%)", time_lookup, percent(time_lookup));
        println!("size calc/setup:   {:6} ns ({:5.1}%)", time_sizecalc, percent(time_sizecalc));
        println!("flatten contour:   {:6} ns ({:5.1}%)", time_flatten, percent(time_flatten));
        println!("metrics compute:   {:6} ns ({:5.1}%)", time_metrics, percent(time_metrics));
        println!("filler:            {:6} ns ({:5.1}%)", time_fill, percent(time_fill));
        println!("cache write:       {:6} ns ({:5.1}%)", time_cache_write, percent(time_cache_write));
        println!("------------------------------------");
        println!("Total (excluding prints): {total_elapsed} ns\n");

        (metrics, self.bitmap_buffer.clone())
    }

}


