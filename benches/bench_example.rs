#![feature(test)]
extern crate test;

use hyber;
use hyber::display::Display;
use hyber::renderer::Renderer;
use hyber_raqote;

/// To run benchmarks you need rust nightly.
/// First install rust nightly with
/// `rustup install nightly`
/// And then use the command
/// `rustup run nightly cargo bench`
#[cfg(test)]
mod benches {
    use test::{black_box, Bencher};

    /// This example benchmarks 100 runs of the power function.
    /// The bencher will execute this 50 times by default and remove any outliers. Check documentation for more details.
    /// You can choose between a bench file for each widget or simply one function for each widget in a single file.
    /// You can even choose to have a bench file for widgets only, and another bench file for events, and another for multi-widget apps!
    /// The black_box handler ensures that the functions inside it are not optimized by the compiler. The compiler treats it like a black-box!
    #[bench]
    fn bench_example(b: &mut Bencher) {
        // Optionally include some setup
        let x: f64 = 211.0 * 11.0;
        let y: f64 = 301.0 * 103.0;

        b.iter(|| {
            // The benchmark measures the time passed inside this closure, not outside!
            for i in 1..100 {
                black_box(x.powf(y).powf(x));
            }
        });
    }
}
