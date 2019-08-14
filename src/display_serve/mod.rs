use crate::errors::*;
use rpi_led_matrix::*;
use std::env::var;
use std::path::Path;

pub struct ScoreCanvas {
    canvas: LedCanvas,
    led_font: LedFont,
    led_color: LedColor,
    content: String,
}

impl ScoreCanvas {
    pub fn new() -> Result<Self> {
        let mut options = LedMatrixOptions::new();
        options.set_hardware_mapping("adafruit-hat");
        options.set_rows(16);

        let matrix = LedMatrix::new(Some(options)).map_err(|e| format_err!("{:?}", e))?;
        let canvas = matrix.canvas();
        let font_var =
            var("RGBLED_LIB_PATH").unwrap_or("/home/korbinian/rpi-rgb-led-matrix".to_string());
        let font_path = Path::new(&font_var).join("fonts/8x13.bdf");
        let led_font = LedFont::new(&font_path).map_err(|e| format_err!("{:?}", e))?;
        let led_color = LedColor{red: 255, green: 255, blue: 255};
        Ok(ScoreCanvas {
            canvas,
            led_font,
            led_color,
            content: String::new(),
        })
    }

    pub fn write_text(&mut self, text: String) {
        self.canvas.clear();
        let kerning_offset = 0;
        let vertical = false;
        self.canvas.draw_text(
            &self.led_font,
            &text,
            30,
            30,
            &self.led_color,
            kerning_offset,
            vertical,
        );
    }

    pub fn led_color(&mut self, red: u8, green: u8, blue: u8) -> () {
        self.led_color = LedColor{ red, green, blue};
    }
}

pub struct NameCanvas {
    matrix: LedMatrix,
    options: LedMatrixOptions,
    content: String,
}

pub struct TimeCanvas {
    matrix: LedMatrix,
    options: LedMatrixOptions,
    content: u64,
}
