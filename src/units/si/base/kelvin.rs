use crate::units::si::si_unit::{SIBaseUnit, SIUnit, Expression};
use crate::units::dimensions::Dimension;
use crate::units::constants::{K, H, DELTA_V_CS};

pub struct Kelvin {}

impl SIBaseUnit for Kelvin {
    fn dimension(&self) -> Dimension {
        Dimension::Temperature
    }

    fn symbol(&self) -> String {
        "K".to_string()
    }

    fn constant(&self) -> f64 {
        K / (H * DELTA_V_CS)
    }
}

impl SIUnit for Kelvin {
    fn base_units(&self) -> Expression {
        Expression::Unit(Box::new(Kelvin {}))
    }

    fn symbol(&self) -> String {
        "K".to_string()
    }
}