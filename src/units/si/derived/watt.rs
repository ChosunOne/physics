use crate::expr_op;
use crate::expr_unit;
use crate::units::si::si_unit::{SIUnit, Expression, Operator};
use crate::units::si::base::second::Second;
use crate::units::si::derived::joule::Joule;

pub struct Watt {}

impl SIUnit for Watt {
    fn base_units() -> Expression {
        let j = Joule::base_units();
        expr_op!(
            j,
            Operator::Divide,
            expr_unit!(Second {})
        )
    }

    fn symbol() -> String {
        "W".to_string()
    }
}