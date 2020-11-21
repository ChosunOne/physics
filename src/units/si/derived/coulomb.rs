use crate::expr_op;
use crate::expr_unit;
use crate::units::si::si_unit::{SIUnit, Expression, Operator};
use crate::units::si::base::ampere::Ampere;
use crate::units::si::base::second::Second;

pub struct Coulomb {}

impl SIUnit for Coulomb {
    fn base_units(&self) -> Expression {
        expr_op!(
            expr_unit!(Second {}),
            Operator::Multiply,
            expr_unit!(Ampere {})
        )
    }

    fn symbol(&self) -> String {
        "C".to_string()
    }
}