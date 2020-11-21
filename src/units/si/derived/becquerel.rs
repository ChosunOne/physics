use crate::expr_op;
use crate::expr_unit;
use crate::units::si::si_unit::{SIUnit, Expression, Operator};
use crate::units::si::base::second::Second;

pub struct Becquerel {}

impl SIUnit for Becquerel {
    fn base_units() -> Expression {
        expr_op!(
            expr_unit!(1.0),
            Operator::Divide,
            expr_unit!(Second {})
        )
    }

    fn symbol() -> String {
        "Bq".to_string()
    }
}