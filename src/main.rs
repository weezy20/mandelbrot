#![allow(unused)]
use std::ptr::NonNull;

use exitfailure::ExitFailure;
use failure::ResultExt;
use num::{
    complex::{self, Complex64},
    Complex,
};

/// Defines the complex plane which is to be mapped onto the image size
struct ComplexPlane {
    /// Upper Left
    ul: Complex64,
    /// Bottom Right
    br: Complex64,
    /// Image size
    size: (u32, u32),
}
fn main() -> Result<(), ExitFailure> {
    // carefully chosen to preserve aspect ratio
    let size = (559_u32, 400_u32);
    let ul = Complex64::new(-2.0, 1.4);
    let br = Complex64::new(0.0, 0.0);
    let cplane = ComplexPlane {ul , br , size};
    Ok(())
}

// Check if for any complex point `c` does it diverge or converge
fn square(c: Complex64) -> Option<Complex64> {
    let mut z = Complex::new(0.0, 0.0);
    for _ in 0..255 {
        z = z.powi(2) + c;
    }
    if z.norm() > 2.0 {
        Some(z)
    } else {
        None
    }
}

/// Takes a pixel on our image and maps it on the Complex Plane for mandelbrot computation
fn pixel_to_point(pixel: (u32, u32), cplane: &ComplexPlane) -> Complex64 {
    let width = cplane.size.0 as f64;
    let height = cplane.size.1 as f64;
    let ul = cplane.ul;
    let br = cplane.br;

    let complex_width = (-ul.re + br.re).abs();
    let complex_height = (ul.im - br.im).abs();

    let map_width = complex_width / width;
    let map_height = complex_height / height;

    let re = ul.re + pixel.0 as f64 * map_width;
    let im = ul.im - pixel.1 as f64 * map_height;

    Complex { re, im }
}

#[test]
fn test_pixel_to_point() {
    let ul = Complex { re: -1.0, im:  1.0 };
    let br = Complex { re:  1.0, im: -1.0 };
    let result = Complex { re: -0.5, im: -0.75 };
    let size = (100, 200);
    let pixel = (25, 175);
    let ref cplane = ComplexPlane {size , ul , br};
    assert_eq!(result , pixel_to_point(pixel, cplane));
}