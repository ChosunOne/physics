use crate::units::si::si_unit::{SIUnit, Expression, Operator};
use crate::units::si::base::meter::Meter;

pub struct Radian {}

impl SIUnit for Radian {
    fn base_units(&self) -> Expression {
        Expression::Operation(
            Box::new(Expression::Unit(Box::new(Meter {}))),
            Operator::Divide,
            Box::new(Expression::Unit(Box::new(Meter {})))
        )
    }

    fn symbol(&self) -> String {
        "rad".to_string()
    }
}