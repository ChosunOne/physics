use crate::units::si::si_unit::{SIUnit, Expression, Operator};
use crate::units::si::base::second::Second;

pub struct Hertz {}

impl SIUnit for Hertz {
    fn base_units(&self) -> Expression {
        Expression::Operation(
            Box::new(Expression::Unit(Box::new(1.0))),
            Operator::Divide,
            Box::new(Expression::Unit(Box::new(Second {})))
        )
    }

    fn symbol(&self) -> String {
        "Hz".to_string()
    }
}