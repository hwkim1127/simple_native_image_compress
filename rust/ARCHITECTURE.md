# ARCHITECTURE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a high-performance Flutter plugin for native image compression, built with Rust using Flutter Rust Bridge. The plugin provides optimized image compression capabilities with support for multiple formats (JPEG, WebP, AVIF), compression strategies, and parallel processing.

## Core Architecture

### Rust Backend (`rust/src/`)
- **`lib.rs`**: Main entry point exposing the `api` module
- **`api/mod.rs`**: Module declarations for the API surface
- **`api/compress.rs`**: Core compression logic with `ImageCompress` struct containing all public methods
- **`api/batch.rs`**: Parallel batch processing for multiple images using Rayon
- **`api/encode.rs`**: Format-specific encoding functions (JPEG, WebP, AVIF) with memory optimization
- **`api/constants.rs`**: Enums for compression formats and filter types with Copy/Clone traits
- **`api/orientation.rs`**: EXIF orientation handling and image rotation

### Key Dependencies
- `image`: Core image processing library (optimized features only)
- `kamadak-exif`: EXIF metadata reading
- `flutter_rust_bridge`: Bridge between Rust and Flutter
- `rayon`: Parallel processing library for batch operations
- `anyhow`: Error handling

## API Design

### Single Image Processing (`ImageCompress`)
The main `ImageCompress` struct provides three compression strategies:

1. **Fit Width**: Scale image to fit within maximum width
2. **Fit Height**: Scale image to fit within maximum height  
3. **Contain**: Scale image to fit within both width and height constraints

Each strategy supports:
- Multiple input sources (file path, bytes)
- Format selection (JPEG, WebP, AVIF)
- Quality control (0-100)
- Sampling filters (Nearest, Triangle, CatmullRom, Gaussian, Lanczos3)
- Speed optimization for AVIF

### Batch Processing (`BatchImageCompress`)
For high-throughput scenarios:
- **`file_paths()`**: Process multiple files in parallel
- **`bytes()`**: Process multiple byte arrays in parallel
- Uses Rayon for CPU-bound parallel processing
- Returns `Vec<Result<Vec<u8>, String>>` for individual error handling

## Performance Optimizations

### Build Configuration
- **Optimization Level**: `opt-level = 3` for maximum performance
- **Link Time Optimization**: `lto = "fat"` for release, `"thin"` for dev
- **Code Generation**: Single codegen unit for better optimization
- **Image Crate**: Only essential features enabled (jpeg, webp, avif, png)

### Runtime Optimizations
- **Memory Pre-allocation**: Buffers pre-sized based on image dimensions
- **Function Inlining**: Critical path functions marked with `#[inline]`
- **Copy Semantics**: Enums implement `Copy` to avoid moves in closures
- **Reduced Allocations**: Eliminated unnecessary clones and temporary variables

### Parallel Processing
- **Rayon Integration**: CPU-bound work distributed across available cores
- **Batch Operations**: Efficient processing of multiple images
- **Error Isolation**: Individual image failures don't affect batch processing

## Build Process

This project uses Cargokit for building the Rust library and integrating with Flutter platforms.

## Key Features

- **EXIF Orientation**: Automatically rotates images based on EXIF orientation data
- **Multiple Formats**: Supports JPEG, WebP, and AVIF compression
- **High Performance**: Optimized build profiles and runtime efficiency
- **Parallel Processing**: Batch operations for improved throughput
- **Memory Efficient**: Pre-allocated buffers and minimal allocations
- **Cross-platform**: Supports all Flutter platforms through Cargokit integration

## Development Notes

- The project includes deprecated methods marked for removal in v4.0.0
- EXIF orientation from bytes is commented out but available for future use
- All compression operations return `Vec<u8>` for maximum flexibility
- Error handling uses `anyhow::Result` for comprehensive error reporting
- Batch processing uses parallel iterators for optimal CPU utilization
- Critical functions are inlined for performance
- Memory allocations are minimized through capacity pre-allocation