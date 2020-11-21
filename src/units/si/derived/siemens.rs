use crate::expr_op;
use crate::expr_unit;
use crate::units::si::si_unit::{SIUnit, Expression, Operator};
use crate::units::si::derived::ohm::Ohm;

pub struct Siemens {}

impl SIUnit for Siemens {
    fn base_units(&self) -> Expression {
        let o = Ohm {}.base_units();
        expr_op!(
            expr_unit!(1.0),
            Operator::Divide,
            o
        )
    }

    fn symbol(&self) -> String {
        "S".to_string()
    }
}