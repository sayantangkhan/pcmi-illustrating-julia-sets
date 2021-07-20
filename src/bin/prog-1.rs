use clap::{crate_version, AppSettings, Clap};
use julia::*;
use num_complex::Complex64;
use std::fs::File;
use std::io::BufWriter;

/// A command line program to plot the filled Julia set of z^2 + c
#[derive(Clap)]
#[clap(version = crate_version!(), author = "Sayantan Khan <saykhan@umich.edu>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Width of the generated image in pixels
    #[clap(long, default_value = "4000")]
    width: usize,
    /// Height of the generated image in pixels
    #[clap(long, default_value = "4000")]
    height: usize,
    /// Complex coordinate of the top left corner
    #[clap(long, default_value = "-2.0 + 2.0i")]
    top_left: Complex64,
    /// Complex coordinate of the bottom right corner
    #[clap(long, default_value = "2.0 - 2.0i")]
    bottom_right: Complex64,
    /// The value of c in the iterated polynomial z^2+c
    #[clap(short)]
    c: Complex64,
    /// Maximum number of iterates for a point
    #[clap(short, long, default_value = "100")]
    escape_threshold: usize,
    /// Squared radius of ball around origin
    #[clap(short, long, default_value = "1e4")]
    radius_squared: f64,
    /// Output file
    #[clap(short, long)]
    output: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    let file = File::create(opts.output).expect("Unable to create output file");
    let ref mut w = BufWriter::new(file);

    plot_basin_of_infinity(
        w,
        opts.width,
        opts.height,
        opts.top_left,
        opts.bottom_right,
        opts.c,
        opts.escape_threshold,
        opts.radius_squared,
    )
    .unwrap();
}
