use crate::expr_op;
use crate::expr_unit;
use crate::units::si::si_unit::{SIUnit, Expression, Operator};
use crate::units::si::base::meter::Meter;
use crate::units::si::derived::newton::Newton;

pub struct Joule {}

impl SIUnit for Joule {
    fn base_units() -> Expression {
        let n = Newton::base_units();
        expr_op!(
            n,
            Operator::Multiply,
            expr_unit!(Meter {})
        )
    }

    fn symbol() -> String {
        "J".to_string()
    }
}