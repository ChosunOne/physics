use crate::expr_op;
use crate::expr_unit;
use crate::units::si::si_unit::{SIUnit, Expression, Operator};
use crate::units::si::base::meter::Meter;

pub struct Radian {}

impl SIUnit for Radian {
    fn base_units() -> Expression {
        expr_op!(
            expr_unit!(Meter {}),
            Operator::Divide,
            expr_unit!(Meter {})
        )
    }

    fn symbol() -> String {
        "rad".to_string()
    }
}