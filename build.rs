extern crate cc;

fn main() {
    cc::Build::new()
        .file("rpi-rgb-led-matrix/lib/led-matrix.cc")
        .include("rpi-rgb-led-matrix/include/")
        .compile("librgbmatrix.a");
}