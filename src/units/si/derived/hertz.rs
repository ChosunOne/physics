use crate::expr_op;
use crate::expr_unit;
use crate::units::si::si_unit::{SIUnit, Expression, Operator};
use crate::units::si::base::second::Second;

pub struct Hertz {}

impl SIUnit for Hertz {
    fn base_units() -> Expression {
        expr_op!(
            expr_unit!(1.0),
            Operator::Divide,
            expr_unit!(Second {})
        )
    }

    fn symbol() -> String {
        "Hz".to_string()
    }
}