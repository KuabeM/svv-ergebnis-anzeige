use super::super::errors::*;
use rpi_led_matrix::{LedCanvas, LedColor, LedFont, LedMatrix, LedMatrixOptions};

// #[derive(Copy)]
pub struct ScorePanel {
    matrix: LedMatrix,
    name: String,
    offset: (i32, i32),
    font: LedFont,
    color: LedColor,
}

impl ScorePanel {
    pub fn new(name: String, offset: (i32,i32), font: LedFont, color: LedColor) -> Result<Self> {
        let options = LedMatrixOptions::new();
        let matrix = LedMatrix::new(Some(options)).map_err(|e| PanelError::InitError.context(e))?;
        Ok(ScorePanel {
            matrix,
            name,
            offset,
            font,
            color,
        })
    }

    pub fn draw_number(&mut self, value: u8) -> Result<()> {
        
        let (x, y) = self.offset;

        let mut canvas = self.matrix.canvas();
        let advanced = canvas.draw_text(&self.font, &value.to_string(), x, y, &self.color, 0, false);
        info!(
            "On Panel {}: advanced {} when writing {}",
            &self.name, &advanced, value
        );
        Ok(())
    }
}
