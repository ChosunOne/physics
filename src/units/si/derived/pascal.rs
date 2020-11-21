use crate::expr_unit;
use crate::expr_op;
use crate::units::si::si_unit::{SIUnit, Expression, Operator};
use crate::units::si::base::meter::Meter;
use crate::units::si::derived::newton::Newton;
pub struct Pascal {}

impl SIUnit for Pascal {
    fn base_units(&self) -> Expression {
        let n = Newton {}.base_units();
        expr_op!(
            n,
            Operator::Divide,
            expr_op!(
                expr_unit!(Meter {}),
                Operator::Power,
                expr_unit!(2.0)
            ))
    }

    fn symbol(&self) -> String {
        "Pa".to_string()
    }
}