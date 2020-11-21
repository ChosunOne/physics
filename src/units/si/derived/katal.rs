use crate::expr_op;
use crate::expr_unit;
use crate::units::si::si_unit::{SIUnit, Expression, Operator};
use crate::units::si::base::mole::Mole;
use crate::units::si::base::second::Second;

pub struct Katal {}

impl SIUnit for Katal {
    fn base_units(&self) -> Expression {
        expr_op!(
            expr_unit!(Mole {}),
            Operator::Divide,
            expr_unit!(Second {})
        )
    }

    fn symbol(&self) -> String {
        "kat".to_string()
    }
}