use crate::units::si::si_unit::{SIBaseUnit, SIUnit, Expression};
use crate::units::dimensions::Dimension;
use crate::units::constants::{DELTA_V_CS, E};

pub struct Ampere {}

impl SIBaseUnit for Ampere {
    fn dimension(&self) -> Dimension {
        Dimension::Current
    }

    fn symbol(&self) -> String {
        "A".to_string()
    }

    fn constant(&self) -> f64 {
        1.0 / (E * DELTA_V_CS)
    }
}

impl SIUnit for Ampere {
    fn base_units(&self) -> Expression {
        Expression::Unit(Box::new(Ampere {}))
    }

    fn symbol(&self) -> String {
        "A".to_string()
    }
}