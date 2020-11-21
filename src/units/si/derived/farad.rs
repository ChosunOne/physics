use crate::expr_op;
use crate::units::si::si_unit::{SIUnit, Expression, Operator};
use crate::units::si::derived::coulomb::Coulomb;
use crate::units::si::derived::volt::Volt;

pub struct Farad {}

impl SIUnit for Farad {
    fn base_units(&self) -> Expression {
        let c = Coulomb {}.base_units();
        let v = Volt {}.base_units();
        expr_op!(
            c,
            Operator::Divide,
            v
        )
    }

    fn symbol(&self) -> String {
        "F".to_string()
    }
}