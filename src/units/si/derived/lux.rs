use crate::expr_op;
use crate::expr_unit;
use crate::units::si::si_unit::{SIUnit, Expression, Operator};
use crate::units::si::base::meter::Meter;
use crate::units::si::derived::lumen::Lumen;

pub struct Lux {}

impl SIUnit for Lux {
    fn base_units(&self) -> Expression {
        let lm = Lumen {}.base_units();
        expr_op!(
            lm,
            Operator::Divide,
            expr_op!(
                expr_unit!(Meter {}),
                Operator::Power,
                expr_unit!(2.0)
            )
        )
    }

    fn symbol(&self) -> String {
        "lx".to_string()
    }
}