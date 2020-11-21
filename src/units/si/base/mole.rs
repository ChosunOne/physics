use crate::units::si::si_unit::{SIBaseUnit, SIUnit, Expression};
use crate::units::dimensions::Dimension;
use crate::units::constants::NA;

pub struct Mole {}

impl SIBaseUnit for Mole {
    fn dimension(&self) -> Dimension {
        Dimension::Amount
    }

    fn symbol(&self) -> String {
        "mol".to_string()
    }

    fn constant(&self) -> f64 {
        NA
    }
}

impl SIUnit for Mole {
    fn base_units() -> Expression {
        Expression::Unit(Box::new(Mole {}))
    }

    fn symbol() -> String {
        "mol".to_string()
    }
}