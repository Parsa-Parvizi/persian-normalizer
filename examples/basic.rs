//! Minimal usage: normalize a string with the default configuration.

use persian_normalizer::normalize;

fn main() {
    println!("{}", normalize("كتاب ١٢٣"));
}