use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use std::hint::black_box;
use ab_glyph::{Font, PxScale, ScaleFont};
use trueforge::TrueTypeFont;
use rusttype::{Scale, point};

fn benchmark_fonts(c: &mut Criterion) {
    let mut group = c.benchmark_group("glyph_rendering");

    // Configure the group
    group.sample_size(10000);           // Number of samples Criterion takes
    group.warm_up_time(std::time::Duration::from_secs(3));  // Warm-up time

    let mut font_0 = TrueTypeFont::load_font(include_bytes!("../Roboto-Medium.ttf"));
    font_0.can_cache = false;

    let font_1 = std::fs::read("Roboto-Medium.ttf").unwrap();
    let font_1 = fontdue::Font::from_bytes(font_1.as_slice(), Default::default()).unwrap();

    let font_2 = std::fs::read("Roboto-Medium.ttf").unwrap();
    let font_2 = rusttype::Font::try_from_vec(font_2).unwrap();

    let font_3 = std::fs::read("Roboto-Medium.ttf").unwrap();
    let font_3 = ab_glyph::FontRef::try_from_slice(&font_3).unwrap();

    let text = "The quick brown fox jumps over the lazy dog. 0123456789";

    // Test different sizes
    for size in [16, 24, 48, 72, 120].iter() {
        group.bench_with_input(BenchmarkId::new("fontdue", size), size, |b, &size| {
            b.iter(|| {
                for c in text.chars() {
                    let (metrics, bitmap) = font_1.rasterize(black_box(c), black_box(size as f32));
                }
            });
        });

        group.bench_with_input(BenchmarkId::new("rusttype", size), size, |b, &size| {
            b.iter(|| {

                for c in text.chars() {
                    let glyph = font_2.glyph(c)
                        .scaled(Scale::uniform(size as f32))
                        .positioned(point(0.0, 0.0));

                    if let Some(bb) = glyph.pixel_bounding_box() {
                        let mut bitmap = vec![0u8; (bb.width() * bb.height()) as usize];

                        glyph.draw(|x, y, v| {
                            let idx = (y * bb.width() as u32 + x) as usize;
                            if idx < bitmap.len() {
                                bitmap[idx] = (v * 255.0) as u8;
                            }
                        });
                    }
                }
            });
        });

        group.bench_with_input(BenchmarkId::new("ab_glyph", size), size, |b, &size| {
            b.iter(|| {
                for c in text.chars() {
                    let scaled_font = font_3.as_scaled(PxScale::from(size as f32));
                    let glyph = scaled_font.scaled_glyph(c);

                    if let Some(outlined) = font_3.outline_glyph(glyph) {
                        let bounds = outlined.px_bounds();
                        let width = bounds.width() as usize;
                        let height = bounds.height() as usize;

                        let mut bitmap = vec![0u8; width * height];

                        outlined.draw(|x, y, v| {
                            let idx = y as usize * width + x as usize;
                            if idx < bitmap.len() {
                                bitmap[idx] = (v * 255.0) as u8;
                            }
                        });
                    }
                }
            });
        });

        group.bench_with_input(BenchmarkId::new("tforge", size), size, |b, &size| {
            b.iter(|| {
                for c in text.chars() {
                    let (metrics, bitmap) = font_0.get_char(black_box(c), black_box(size));
                }
            });
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_fonts);
criterion_main!(benches);