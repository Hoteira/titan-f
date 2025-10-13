<div align="center">
  <img src="img/icon.png" alt="Fontkit Logo" width="120" height="120">
  
  # TrueForge
  
  **A fast, lightweight font rasterizer written in pure Rust**
  
  [![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
  [![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
  [![no_std](https://img.shields.io/badge/no__std-compatible-success.svg)](https://docs.rust-embedded.org/book/)
  
</div>

---

## Overview

TForge is a high-performance CPU-based font rasterizer designed for speed and simplicity. It uses winding number algorithms for accurate glyph rendering with adaptive bezier curve tessellation, making it ideal for both desktop applications and embedded systems.
To stay dependency free it also includes its own basic parser for True Type Font files which handles:
- CMAP
- GLYF
- HEAD
- HHEA
- HMTX
- KERN
- LOCA
- MAXP

## Features

- 🚀 **Fast**
- 🦀 **Dependency free**
- 🧠 **Simple kerning, metrics and caching**
- 🎯 **Accurate** — Winding number algorithm for precise fill operations
- 📦 **`no_std` Compatible** — Works in bare-metal environments out of the box!
- ⚡ **Adaptive Tessellation** — Smart curve flattening based on complexity
- 🎨 **Anti-aliased** — Smooth, high-quality glyph rendering

## Performance

