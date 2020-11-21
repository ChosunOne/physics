use crate::expr_op;
use crate::expr_unit;
use crate::units::si::si_unit::{SIUnit, Expression, Operator};
use crate::units::si::base::ampere::Ampere;
use crate::units::si::derived::volt::Volt;

pub struct Ohm {}

impl SIUnit for Ohm {
    fn base_units(&self) -> Expression {
        let v = Volt {}.base_units();
        expr_op!(
            v,
            Operator::Divide,
            expr_unit!(Ampere {})
        )
    }

    fn symbol(&self) -> String {
        "Ω".to_string()
    }
}