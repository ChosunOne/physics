use crate::units::si::si_unit::{SIBaseUnit, SIUnit, Expression};
use crate::units::dimensions::Dimension;
use crate::units::constants::{DELTA_V_CS, C, H};

pub struct Kilogram {}

impl SIBaseUnit for Kilogram {
    fn dimension(&self) -> Dimension {
        Dimension::Mass
    }

    fn symbol(&self) -> String {
        "kg".to_string()
    }

    fn constant(&self) -> f64 {
        C.powi(2) / (H * DELTA_V_CS)
    }
}

impl SIUnit for Kilogram {
    fn base_units() -> Expression {
        Expression::Unit(Box::new(Kilogram {}))
    }

    fn symbol() -> String {
        "kg".to_string()
    }
}