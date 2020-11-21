use crate::expr_op;
use crate::expr_unit;
use crate::units::si::si_unit::{SIUnit, Expression, Operator};
use crate::units::si::base::{kilogram::Kilogram, meter::Meter, second::Second};

pub struct Newton {}

impl SIUnit for Newton {
    fn base_units() -> Expression {
        expr_op!(
            expr_op!(
                expr_unit!(Kilogram {}),
                Operator::Multiply,
                expr_unit!(Meter {})
            ),
            Operator::Divide,
            expr_op!(
                expr_unit!(Second {}),
                Operator::Power,
                expr_unit!(2.0)
            )
        )
    }

    fn symbol() -> String {
        "N".to_string()
    }
}