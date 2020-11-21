use crate::expr_op;
use crate::expr_unit;
use crate::units::si::si_unit::{SIUnit, Expression, Operator};
use crate::units::si::base::kilogram::Kilogram;
use crate::units::si::derived::joule::Joule;

pub struct Sievert {}

impl SIUnit for Sievert {
    fn base_units() -> Expression {
        let j = Joule::base_units();
        expr_op!(
            j,
            Operator::Divide,
            expr_unit!(Kilogram {})
        )
    }

    fn symbol() -> String {
        "Sv".to_string()
    }
}