# svv-ergebnis-anzeige -- Nitschkes magische Ergebnisanzeige

__'Backend' of the display written in `rust`__

Provide API to control content of RGB-LED matrix displays over GPIOs on a raspberry pi. The frontend lives at [svv-ergebnis-frontend](https://github.com/KuabeM/svv-ergebnis-frontend)

## Build an run it

The display part depends on the [rpi-rgb-led] library and the corresponding [rust-bindings](https://crates.io/crates/rpi-led-matrix). Therefore, the rpi-rgb-led library has to be built and linked:

1. clone [rpi-rgb-led] and build the library with `make`
2. adjust the path to the library in `build.rs` to point to the `lib/` folder
3. `cargo run`, server listens on 192.168.0.41:3000 (TODO: make ip configurable)




[rpi-rgb-led]: https://github.com/hzeller/rpi-rgb-led-matrix