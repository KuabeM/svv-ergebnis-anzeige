use rpi_led_matrix::*;
use crate::errors::*;
use std::path::Path;


pub struct ScoreCanvas {
    canvas: LedCanvas,
    led_font: LedFont,
    content: String,
}

impl ScoreCanvas {
    pub fn new() -> Result<Self> {
        let options = LedMatrixOptions::new();
        let matrix = LedMatrix::new(Some(options))
            .map_err(|e| format_err!("{:?}", e))?;
        let canvas = matrix.canvas();
        let led_font = LedFont::new(Path::new("/home/korbinian/MiscProjects/rpi-rgb-led-matrix/fonts/8x13.bdf"))
            .map_err(|e| format_err!("{:?}", e))?;
        Ok(ScoreCanvas { canvas, led_font, content: String::new()})
    }

    pub fn write_text(&mut self, text: String, color: &LedColor) {
        
        self.canvas.clear();
        let kerning_offset = 0;
        let vertical = false;
        self.canvas.draw_text(&self.led_font, &text, 30, 30, color, kerning_offset, vertical);
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