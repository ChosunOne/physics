use crate::units::si::si_unit::SIUnit;

pub struct MeasuredQuantity {
    magnitude: f64,
    precision: usize,
    unit: Box<dyn SIUnit>
}

impl MeasuredQuantity {
    pub fn new(magnitude: f64, precision: usize, unit: Box<dyn SIUnit>) -> Self {
        Self {
            magnitude,
            precision,
            unit
        }
    }
}