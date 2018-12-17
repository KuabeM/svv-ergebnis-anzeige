use rpi_led_matrix::{LedMatrix, LedMatrixOptions};
use super::super::errors::*;

pub fn init_score_panel() -> Result<LedMatrix> {
    
    let options = LedMatrixOptions::new();
    let matrix = LedMatrix::new(Some(options)).map_err(|e| PanelError::InitError.context(e))?;
    Ok(matrix)

}