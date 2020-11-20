use crate::units::si::si_unit::{SIUnit, Expression, Operator};
use crate::units::si::base::{kilogram::Kilogram, meter::Meter, second::Second};

pub struct Newton {}

impl SIUnit for Newton {
    fn base_units(&self) -> Expression {
        Expression::Operation(
            Box::new(Expression::Operation(
                Box::new(Expression::Unit(Box::new(Kilogram {}))),
                Operator::Multiply,
                Box::new(Expression::Unit(Box::new(Meter {})))
            )),
            Operator::Divide,
            Box::new(Expression::Operation(
                Box::new(Expression::Unit(Box::new(Second {}))),
                Operator::Power,
                Box::new(Expression::Unit(Box::new(2.0)))
            ))
        )
    }

    fn symbol(&self) -> String {
        "N".to_string()
    }
}