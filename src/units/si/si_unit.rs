use crate::units::dimensions::Dimension;

pub trait SIBaseUnit {
    fn dimension(&self) -> Dimension;
    fn symbol(&self) -> String;
    fn constant(&self) -> f64;
}

impl SIBaseUnit for f64 {
    fn dimension(&self) -> Dimension {
        Dimension::None
    }

    fn symbol(&self) -> String {
        "".to_string()
    }

    fn constant(&self) -> f64 {
        1.0
    }
}

pub trait SIUnit {
    fn base_units() -> Expression;
    fn symbol() -> String;
}

pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power
}

pub enum Expression {
    Unit(Box<dyn SIBaseUnit>),
    Operation(Box<Expression>, Operator, Box<Expression>)
}

#[macro_export]
macro_rules! expr_unit {
    ($base_unit:expr) => {
        Expression::Unit(Box::new($base_unit))
    }
}

#[macro_export]
macro_rules! expr_op {
    ($left:expr, $op:expr, $right:expr) => {
        Expression::Operation(
            Box::new($left),
            $op,
            Box::new($right)
        )
    }
}