use crate::expr_op;
use crate::expr_unit;
use crate::units::si::si_unit::{SIUnit, Expression, Operator};
use crate::units::si::derived::volt::Volt;
use crate::units::si::base::second::Second;

pub struct Weber {}

impl SIUnit for Weber {
    fn base_units(&self) -> Expression {
        let v = Volt {}.base_units();
        expr_op!(
            v,
            Operator::Multiply,
            expr_unit!(Second {})
        )
    }

    fn symbol(&self) -> String {
        "Wb".to_string()
    }
}