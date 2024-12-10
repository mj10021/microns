# microns

**microns** is a simple, dependency-free Rust library that enables working with floating-point values as fixed-precision integers. Its primary purpose is to help avoid the pitfalls of floating-point arithmetic in contexts where a fixed, micrometer-level precision is sufficient.

## Overview

**microns** gets its name from the idea of converting millimeter-formatted floating-point values (`f32`) into integer-based micrometer values (`i32`). Instead of dealing with floating-point numbers directly and risking errors due to floating-point precision, you convert your `f32` values into `Microns` (stored internally as `i32`), and then use standard arithmetic operations on these integers.

By scaling values such that `1_000 microns = 1 millimeter`, `microns` makes it convenient to represent geometric coordinates, distances, and other measurements at a micron-level resolution. Although the primary use case might be for CNC machines, 3D printers, or robotics—where consistent, small increments matter—this library can be useful whenever controlled precision is required and `i32::MIN < float < i32::MAX`.

**Key points:**
- Represent floating-point values as integers at a micron-level scale.
- No dependencies required (optional integration with `serde` for serialization).
- Simple, straightforward API for conversion, arithmetic, and comparisons.
- Safer integer arithmetic helps avoid subtle floating-point rounding errors.

## Features

- **Fixed-precision representation**: Values are stored as `i32` integers, allowing you to consistently reason about increments of 1 micron.
- **Arithmetic operations**: Add, subtract, multiply, and divide by `f32` values, returning `Microns`.  
- **Conversions**:  
  - Convert from `f32` to `Microns` using `Microns::from(f32)`.  
  - Convert back to `f32` using `f32::from(microns)`.  
- **Abs operation**: Compute the absolute value of a `Microns` instance.
- **Range checks**: The `works()` function ensures that the `f32` value can fit in an `i32` after scaling to microns.

## When to Use

Use **microns** when:
- You need stable, predictable fixed-point arithmetic without the overhead of arbitrary precision libraries.
- You are working with machine coordinates (e.g., CNC, 3D printers) where a micron-level resolution is sufficiently precise and where raw floating-point arithmetic might introduce unacceptable rounding errors.
- You need an easy-to-understand scale factor (e.g., 1/1000 for mm-to-microns) to standardize calculations.