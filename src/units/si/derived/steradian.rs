use crate::units::si::si_unit::{SIUnit, Expression, Operator};
use crate::units::si::base::meter::Meter;

pub struct Steradian {}

impl SIUnit for Steradian {
    fn base_units(&self) -> Expression {
        Expression::Operation(
            Box::new(Expression::Operation(
                Box::new(Expression::Unit(Box::new(Meter {}))),
                Operator::Power,
                Box::new(Expression::Unit(Box::new(2.0)))
            )),
            Operator::Divide,
            Box::new(Expression::Operation(
                Box::new(Expression::Unit(Box::new(Meter {}))),
                Operator::Power,
                Box::new(Expression::Unit(Box::new(2.0)))
            ))
        )
    }

    fn symbol(&self) -> String {
        "sr".to_string()
    }
}