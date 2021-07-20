use julia::*;
use num_complex::Complex64;
use std::env;
use std::fs::File;
use std::io::BufWriter;

fn main() {
    let path = env::args()
        .nth(1)
        .expect("Expected a filename to output to.");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let top_left = Complex64::new(-2.0, 2.0);
    let bottom_right = Complex64::new(2.0, -2.0);
    let width = 6000;
    let height = 6000;
    let c = Complex64::new(-0.1, 0.75);
    let escape_threshold = 1000;
    let radius_squared = 1e8;
    plot_basin_of_infinity(
        w,
        width,
        height,
        top_left,
        bottom_right,
        c,
        escape_threshold,
        radius_squared,
    )
    .unwrap();
}
