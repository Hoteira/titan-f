use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use std::hint::black_box;
use ab_glyph::{Font, PxScale, ScaleFont};
use trueforge::TrueTypeFont;
use rusttype::{Scale, point};

fn benchmark_fonts(c: &mut Criterion) {
    let mut group = c.benchmark_group("glyph_rendering");

    group.sample_size(10);
    group.warm_up_time(std::time::Duration::from_secs(1));

    let mut font_0 = TrueTypeFont::load_font(include_bytes!("../Roboto-Medium.ttf"));

    let font_1 = std::fs::read("Roboto-Medium.ttf").unwrap();
    let font_1 = fontdue::Font::from_bytes(font_1.as_slice(), Default::default()).unwrap();

    let font_2 = std::fs::read("Roboto-Medium.ttf").unwrap();
    let font_2 = rusttype::Font::try_from_vec(font_2).unwrap();

    let font_3 = std::fs::read("Roboto-Medium.ttf").unwrap();
    let font_3 = ab_glyph::FontRef::try_from_slice(&font_3).unwrap();

    let text = "The quick brown fox jumps over the lazy dog. 0123456789. ,;!?-:()[]{}<>|/@#$%^&*~`+=\\'\".
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus nec dui vel tortor interdum euismod. Integer vitae justo eu orci auctor maximus at vitae mauris. Cras sit amet odio pharetra, posuere est gravida, convallis libero. Phasellus tincidunt velit ante, vel fermentum lorem tempus vel. Sed consectetur massa eget facilisis maximus. Suspendisse potenti. Nam non diam sit amet magna mollis placerat id nec arcu. Nulla mattis viverra imperdiet. Aliquam molestie vulputate sapien, in rutrum ex hendrerit sit amet. Integer tempor lorem sed turpis finibus gravida. Duis vel dapibus urna. Etiam tempus vulputate turpis, et tempus orci ullamcorper a. Nam at velit.";

    // Test different sizes
    for size in [1000, 12, 16, 24, 48, 72, 120, 250].iter() {
        group.bench_with_input(BenchmarkId::new("fontdue", size), size, |b, &size| {
            b.iter(|| {
                for c in text.chars() {
                    let (metrics, bitmap) = font_1.rasterize(black_box(c), black_box(size as f32));
                    black_box(&bitmap);
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
                    let (metrics, b) = font_0.get_char::<false>(black_box(c), black_box(size));
                    black_box(&b);
                }
            });
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_fonts);
criterion_main!(benches);