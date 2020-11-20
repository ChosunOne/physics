use crate::units::si::si_unit::{SIBaseUnit, SIUnit, Expression};
use crate::units::dimensions::Dimension;
use crate::units::constants::{DELTA_V_CS, H, KCD};

pub struct Candela {}

impl SIBaseUnit for Candela {
    fn dimension(&self) -> Dimension {
        Dimension::Luminosity
    }

    fn symbol(&self) -> String {
        "cd".to_string()
    }

    fn constant(&self) -> f64 {
        1.0 / (KCD * H * DELTA_V_CS.powi(2))
    }
}

impl SIUnit for Candela {
    fn base_units(&self) -> Expression {
        Expression::Unit(Box::new(Candela {}))
    }

    fn symbol(&self) -> String {
        "cd".to_string()
    }
}