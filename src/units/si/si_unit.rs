use crate::units::dimensions::Dimension;

pub trait SIBaseUnit {
    fn dimension(&self) -> Dimension;
    fn symbol(&self) -> String;
    fn constant(&self) -> f64;
}

impl SIBaseUnit for f64 {
    fn dimension(&self) -> Dimension {
        unimplemented!()
    }

    fn symbol(&self) -> String {
        unimplemented!()
    }

    fn constant(&self) -> f64 {
        unimplemented!()
    }
}

pub trait SIUnit {
    fn base_units(&self) -> Expression;
    fn symbol(&self) -> String;
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