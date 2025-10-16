<div align="center">
  <img src="img/icon.png" alt="TitanF Logo" width="120" height="120">
  
  # TitanF
  
  **The font rasterizer that doesn't slow down**
  
  [![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
  [![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)
  [![no_std](https://img.shields.io/badge/no__std-compatible-success.svg)](https://docs.rust-embedded.org/book/)

</div>

---

## Quick Start
```rust
use titanf::TrueTypeFont;

fn main() {
    let font_data = include_bytes!("Roboto-Medium.ttf");
    let mut font = TrueTypeFont::load_font(font_data);
    
    // Render a character!
    let (metrics, bitmap) = font.get_char::<false>('A', 16);
    
    //Enable the built-in cache
    let (metrics, bitmap) = font.get_char::<true>('B', 16);
    //                                      ^^^^
}
```

**Add to your `Cargo.toml`:**
```toml
[dependencies]
titanf = { git = "https://github.com/Hoteira/titan-f/" }

//It's not on crates.io as of yet so you cannot just add it as:
//titanf = "0.1.0"
```

---



### Batch Rendering: 1000 Characters (CJK + L)

Criterion benchmark rendering with NotoSansSC-Medium:

| Size | TitanF | fontdue | rusttype | ab_glyph | 
|------|--------|---------|----------|----------|
| 12pt | **174µs** | 708µs | 2.71ms | 2.99ms |
| 16pt | **190µs** | 1.01ms | 3.64ms | 2.99ms |
| 24pt | **203µs** | 1.48ms | 3.75ms | 2.60ms |
| 48pt | **250µs** | 1.89ms | 5.12ms | 6.00ms |
| 72pt | **1.00ms** | 4.20ms | 10.22ms | 10.55ms |
| 120pt | **806.57µs** | 7.63ms | 17.022ms | 14.7ms |
| 250pt | **4.6ms** | 29.36ms | 50.18ms | 50.51ms |

**The performance gap widens with batch size.** While competitors slow down exponentially, TitanF maintains near-constant per-character performance up until 1-10 million glyphs.

---

## Why TitanF Is Fast

### 1. Smart Buffer Reuse
- Buffers allocated **once per font instance**
- Same-sized glyphs: just `memset` and reuse (near-zero cost)
- Larger glyphs: grow buffer once, never shrink
- **Result: Zero allocation overhead after warmup**

### 2. Cache-Optimal Memory Layout
- Sequential memory access patterns
- No pointer chasing or indirection
- CPU prefetcher works perfectly
- Data stays hot in L1/L2 cache

### 3. Simple, Predictable Algorithm
- Winding number algorithm with single-pass filling
- No complex data structures
- Branch predictor loves it
- Compiler can optimize aggressively

---

## Features

- 🚀 **Blazingly Fast** 
- 📈 **Linear Scaling** — Performance doesn't "blow up" whether rendering 10 or 10 million glyphs
- 🦀 **Zero Dependencies** — Pure Rust, no external crates
- 📦 **`no_std` Compatible** — Originally built for my own OS, it works fine in baremetal environments (just needs `alloc`)
- 🎨 **Subpixel Anti-aliasing** — Smooth, high-quality glyph rendering
- 💯 **Stable Rust** — No nightly features, no unsafe code
- 🔧 **Built-in TrueType Parser** — Handles CMAP, GLYF, HEAD, HHEA, HMTX, KERN, LOCA, MAXP and keeps it dependency free

---

## Benchmarking Notes

**Hardware:** All benchmarks run on the same machine with consistent methodology.

**Methodology:**
- Each rasterizer called with identical parameters
- Results wrapped in `black_box()` to prevent optimization
- Multiple runs averaged for consistency
- No caching enabled

**Reproducibility:** Benchmark code available in the repo. Run it yourself:
```bash
cargo bench
```

---

## License

licensed under MIT

See [LICENSE-MIT](LICENSE-MIT) for details.

---

## Contributing

Found a bug? Have a performance improvement? Contributions are welcome!

Please open an issue or PR on GitHub.

---

<div align="center">
  
  **TitanF: Built for speed. Designed for scale. Zero compromises.**
  
  <sub> Pure Rust  •  Zero Dependencies  •  no_std</sub>
  
</div>


