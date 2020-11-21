use crate::units::si::si_unit::{SIBaseUnit, SIUnit, Expression};
use crate::units::dimensions::Dimension;
use crate::units::constants::DELTA_V_CS;

pub struct Second {}

impl SIBaseUnit for Second {
    fn dimension(&self) -> Dimension {
        Dimension::Time
    }

    fn symbol(&self) -> String {
        "s".to_string()
    }

    fn constant(&self) -> f64 {
        DELTA_V_CS
    }
}

impl SIUnit for Second {
    fn base_units() -> Expression {
        Expression::Unit(Box::new(Second {}))
    }

    fn symbol() -> String {
        "s".to_string()
    }
}