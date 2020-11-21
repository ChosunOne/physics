use crate::units::si::si_unit::{SIBaseUnit, SIUnit, Expression};
use crate::units::dimensions::Dimension;
use crate::units::constants::{C, DELTA_V_CS};

pub struct Meter {}

impl SIBaseUnit for Meter {
    fn dimension(&self) -> Dimension {
        Dimension::Length
    }

    fn symbol(&self) -> String {
        "m".to_string()
    }

    fn constant(&self) -> f64 {
        DELTA_V_CS / C
    }
}

impl SIUnit for Meter {
    fn base_units() -> Expression {
        Expression::Unit(Box::new(Meter {}))
    }

    fn symbol() -> String {
        "m".to_string()
    }
}