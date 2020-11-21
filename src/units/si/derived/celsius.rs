use crate::expr_unit;
use crate::units::si::si_unit::{SIUnit, Expression};
use crate::units::si::base::kelvin::Kelvin;

pub struct Celsius {}

impl SIUnit for Celsius {
    fn base_units() -> Expression {
        expr_unit!(Kelvin {})
    }

    fn symbol() -> String {
        "°C".to_string()
    }
}