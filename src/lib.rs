use num_complex::Complex64;
use png::{self, Encoder, EncodingError};
use rayon::prelude::*;
use std::io::Write;

pub mod tc;

pub fn point_escapes(
    mut z: Complex64,
    c: Complex64,
    escape_threshold: usize,
    radius_squared: f64,
) -> Option<usize> {
    for n in 0..escape_threshold {
        if z.norm_sqr() > radius_squared {
            return Some(n);
        }
        z = z * z + c;
    }
    None
}

pub fn plot_basin_of_infinity<W: Write>(
    output: W,
    width: usize,
    height: usize,
    top_left: Complex64,
    bottom_right: Complex64,
    c: Complex64,
    escape_threshold: usize,
    radius_squared: f64,
) -> Result<(), EncodingError> {
    let mut encoder = Encoder::new(output, width as u32, height as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.add_ztxt_chunk("Comment", "Generation parameters. Add later.")?;

    let mut writer = encoder.write_header()?;
    let mut image_buffer: Vec<u8> = vec![255; width * height * 3];

    image_buffer
        .par_chunks_mut(3)
        .enumerate()
        .for_each(|(index, pixel)| {
            let column = index % width;
            let row = index / width;

            let z_real =
                top_left.re + (column as f64) * (bottom_right.re - top_left.re) / (width as f64);
            let z_imag =
                top_left.im + (row as f64) * (bottom_right.im - top_left.im) / (height as f64);
            let z = Complex64::new(z_real, z_imag);

            match point_escapes(z, c, escape_threshold, radius_squared) {
                None => {
                    // pixel[0] = (255.0 / color_scale(z, 0)) as u8;
                    // pixel[1] = (255.0 / color_scale(z, 0)) as u8;
                    // pixel[2] = (255.0 / color_scale(z, 0)) as u8;
                }
                Some(n) => {
                    // pixel[0] = (255.0 - (255.0 / (1.0 + 0.2 * (n as f64)))) as u8;
                    // pixel[0] = (255.0 - 255.0 / color_scale(z, n)) as u8;
                    pixel[0] = (127.0
                        - 127.0 * 1.0 * approximate_potential_in_basin_of_infinity(z, c, 1e-6))
                        as u8;
                    // pixel[1] = (127.0 - (127.0 / (1.0 + 0.2 * (n as f64)))) as u8;
                    // pixel[1] = (127.0 - 127.0 / color_scale(z, n)) as u8;
                    pixel[1] = (127.0
                        - 127.0 * approximate_potential_in_basin_of_infinity(z, c, 1e-6))
                        as u8;
                }
            }
        });

    writer.write_image_data(&image_buffer)
}

fn approximate_potential_in_basin_of_infinity(
    mut z: Complex64,
    c: Complex64,
    difference_threshold: f64,
) -> f64 {
    let mut den: f64 = 2.0;
    let mut current_value = z.norm_sqr().log10() / den;
    while ((z * z + c).norm_sqr().log10() / (2.0 * den) - current_value).abs()
        > difference_threshold
    {
        z = z * z + c;
        den *= 2.0;
        current_value = z.norm_sqr().log10() / den;
    }
    current_value
}

// fn approximate_potential_in_compact_basin(
//     mut z: Complex64,
//     c: Complex64,
//     difference_threshold: f64
// ) -> f64 {
//     let a = c.
// }
