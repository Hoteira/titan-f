<div align="center">
  <img src="img/icon.png" alt="TitanF Logo" width="120" height="120">
  
  # TitanF
  
  **A fast, lightweight font rasterizer written in pure Rust**
  
  [![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
  [![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
  [![no_std](https://img.shields.io/badge/no__std-compatible-success.svg)](https://docs.rust-embedded.org/book/)
  
</div>

---

## Overview

TitanForg (in short TitanF) is a high-performance CPU-based font rasterizer designed for exceptional scalability. While most rasterizers experience severe performance degradation when rendering large batches of text, TitanF maintains near-constant per-character performance whether rendering 1 glyph or 1 million.

To stay dependency-free, it includes its own minimal TrueType parser handling CMAP, GLYF, HEAD, HHEA, HMTX, KERN, LOCA, and MAXP tables.

## Features

- 🚀 **Exceptionally Fast** — Optimized hot paths with smart buffer reuse
- 📈 **Linear Scaling** — Maintains per-character performance at any batch size
- 🦀 **Zero Dependencies** — Pure Rust implementation with built-in TTF parser
- 📦 **`no_std` Compatible** — Works in bare-metal environments with just `alloc`
- 🎯 **Accurate** — Winding number algorithm for precise fill operations
- 🎨 **Anti-aliased** — Smooth, high-quality glyph rendering
- 💯 **Stable Rust** — No nightly features required

## Performance

### Extreme Scale Benchmark: 12 Million Characters

TitanF demonstrates exceptional performance at extreme scales where other rasterizers experience catastrophic degradation:

**Test conditions:**
- 12 million characters without caching
- Varied characters: `['@', 'A', 'g', 'W', 'i', 'M', 'j', 'Q']`
- Varied sizes: `[12, 14, 16, 24, 58, 12, 25, 40]pt`

| Rasterizer | Time | Characters/sec | vs TitanF |
|------------|------|----------------|-----------|
| **TitanF** | **~1.2s** | **~10M/s** | **1× (baseline)** |
| fontdue | ~23s | ~520K/s | **~19× slower** |
| FreeType | ~240s | ~50-200K/s | **~50-192× slower** |

**Key insights:**
- TitanF maintains **consistent performance** even with varied character dimensions and sizes
- Buffer reuse strategy eliminates allocation overhead while gracefully handling size changes
- Competitors' allocation-per-glyph approach causes **20-200× performance collapse** at production scales

### Batch Rendering: 770 Characters (Roboto Medium)

| Size | TitanF | fontdue | rusttype | ab_glyph |
|------|---------|---------|----------|----------|
| 12pt | 348µs | 1.33ms | 3.00ms | 2.89ms |
| 16pt | 439µs | 1.46ms | 3.46ms | 2.98ms |
| 24pt | 367µs | 1.65ms | 3.67ms | 3.40ms |
| 48pt | 424µs | 2.45ms | 6.23ms | 6.00ms |
| 72pt | 234µs | 3.25ms | 8.94ms | 5.16ms |
| 120pt | 895µs | 3.14ms | 9.84ms | 15.0ms |
| 250pt | 1.60ms | 19.7ms | 48.6ms | 44.2ms |

At common text sizes (12-24pt), TitanF is **3-10× faster** than alternatives. The performance advantage increases dramatically with batch size due to superior memory management.

### Why TitanF Scales Better

Unlike traditional rasterizers that allocate fresh buffers per glyph, TitanF:

1. **Reuses buffers** across same-sized glyphs (zero allocation cost after warmup)
2. **Smart reallocation** only grows buffers when encountering larger glyphs, never shrinks
3. **Cache-optimal layout** with sequential memory access patterns that CPUs love
4. **Simple winding algorithm** that branch predictors and prefetchers can optimize effectively

## Installation

Add this to your `Cargo.toml`:
```toml
[dependencies]
titanforge = "0.1.0"
```

## Usage
```rust
use titanf::TrueTypeFont;

fn main() {
    let font_data = include_bytes!("path/to/font.ttf");
    let mut font = TrueTypeFont::load_font(font_data);
    
    // Render a single character at 16pt
    let (metrics, bitmap) = font.get_char::<false>('A', 16);
    
    // Enable caching for repeated renders of the same character
    let (metrics, bitmap) = font.get_char::<true>('B', 16);
    
    println!("Width: {}, Height: {}", metrics.width, metrics.height);
    println!("Advance: {}, Baseline: {}", metrics.advance_width, metrics.base_line);
}
```

## Technical Details

### Winding Number Algorithm

TitanF uses a winding number approach for filling:
1. Rasterize glyph contours into a winding buffer with subpixel precision
2. Accumulate winding numbers across each scanline
3. Apply non-zero winding rule for final coverage values
4. Convert coverage to anti-aliased alpha values in a single pass

This provides accurate results while maintaining simple, cache-friendly code with predictable memory access patterns.

### Buffer Management

The secret to TitanF's scalability:
- Winding and bitmap buffers are allocated once per font instance
- Same-sized glyphs reuse existing allocations with fast `memset` clear
- Buffers only grow (never shrink) to accommodate larger glyphs as needed (can be resized if necessary by the user if necessary)
- Zero per-glyph allocation overhead after encountering the largest glyph
- This eliminates the 20-50× allocation overhead that plagues other rasterizers

## `no_std` Support

TitanF works in `no_std` environments with just `alloc`:
```toml
[dependencies]
titanforge = { version = "0.1.0", default-features = false, features = ["alloc"] }
```

Perfect for:
- Embedded systems
- Operating system kernels
- Bootloaders
- WebAssembly
- Any bare-metal environment with a heap allocator

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please open an issue or PR on GitHub.

---

<div align="center">
  <sub>Built with performance in mind. Zero dependencies. Pure Rust.</sub>
</div>
