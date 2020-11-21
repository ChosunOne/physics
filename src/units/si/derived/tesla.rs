use crate::expr_op;
use crate::expr_unit;
use crate::units::si::si_unit::{SIUnit, Expression, Operator};
use crate::units::si::base::meter::Meter;
use crate::units::si::derived::weber::Weber;

pub struct Tesla {}

impl SIUnit for Tesla {
    fn base_units(&self) -> Expression {
        let wb = Weber {}.base_units();
        expr_op!(
            wb,
            Operator::Divide,
            expr_op!(
                expr_unit!(Meter {}),
                Operator::Power,
                expr_unit!(2.0)
            )
        )
    }

    fn symbol(&self) -> String {
        "T".to_string()
    }
}