//! DPI settings and validation domain model.

pub use crate::protocol::constants::{DPI_MIN, DPI_MAX, DPI_STEP};

/// Validate and clamp DPI to step boundary
pub fn clamp_dpi(dpi: u32) -> u32 {
    let clamped = dpi.clamp(DPI_MIN, DPI_MAX);
    let rem = clamped % DPI_STEP;
    if rem >= DPI_STEP / 2 {
        clamped + (DPI_STEP - rem)
    } else {
        clamped - rem
    }
}
