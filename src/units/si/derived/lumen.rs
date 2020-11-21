use crate::expr_op;
use crate::expr_unit;
use crate::units::si::si_unit::{SIUnit, Expression, Operator};
use crate::units::si::base::candela::Candela;
use crate::units::si::derived::steradian::Steradian;

pub struct Lumen {}

impl SIUnit for Lumen {
    fn base_units(&self) -> Expression {
        let sr = Steradian {}.base_units();
        expr_op!(
            expr_unit!(Candela {}),
            Operator::Multiply,
            sr
        )
    }

    fn symbol(&self) -> String {
        "lm".to_string()
    }
}