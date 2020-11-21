use crate::expr_op;
use crate::expr_unit;
use crate::units::si::si_unit::{SIUnit, Expression, Operator};
use crate::units::si::base::ampere::Ampere;
use crate::units::si::derived::weber::Weber;

pub struct Henry {}

impl SIUnit for Henry {
    fn base_units() -> Expression {
        let wb = Weber::base_units();
        expr_op!(
            wb,
            Operator::Divide,
            expr_unit!(Ampere {})
        )
    }

    fn symbol() -> String {
        "H".to_string()
    }
}
