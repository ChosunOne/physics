use crate::expr_op;
use crate::expr_unit;
use crate::units::si::si_unit::{SIUnit, Expression, Operator};
use crate::units::si::base::ampere::Ampere;
use crate::units::si::derived::watt::Watt;

pub struct Volt {}

impl SIUnit for Volt {
    fn base_units(&self) -> Expression {
        let w = Watt {}.base_units();
        expr_op!(
            w,
            Operator::Divide,
            expr_unit!(Ampere {})
        )
    }

    fn symbol(&self) -> String {
        "V".to_string()
    }
}