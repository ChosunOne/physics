use crate::expr_op;
use crate::expr_unit;
use crate::units::si::si_unit::{SIUnit, Expression, Operator};
use crate::units::si::base::meter::Meter;

pub struct Steradian {}

impl SIUnit for Steradian {
    fn base_units() -> Expression {
        expr_op!(
            expr_op!(
                expr_unit!(Meter {}),
                Operator::Power,
                expr_unit!(2.0)
            ),
            Operator::Divide,
            expr_op!(
                expr_unit!(Meter {}),
                Operator::Power,
                expr_unit!(2.0)
            )
        )
    }

    fn symbol() -> String {
        "sr".to_string()
    }
}