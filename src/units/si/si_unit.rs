use crate::units::dimensions::Dimension;
use crate::units::si::base::BaseUnit;

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
    fn base_units(&self) -> Expression;
    fn symbol(&self) -> String;
}

#[derive(Copy, Clone)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power
}

#[derive(Clone)]
pub enum Expression {
    Unit(BaseUnit),
    Operation(Box<Expression>, Operator, Box<Expression>)
}

impl SIUnit for Expression {
    fn base_units(&self) -> Expression {
        self.clone()
    }

    fn symbol(&self) -> String {
        match self {
            Expression::Unit(u) => {
                SIBaseUnit::symbol(u)
            }
            Expression::Operation(left, op, right) => {
                let ls = left.symbol();
                let rs = right.symbol();
                match op {
                    Operator::Add => {
                        if ls == rs {
                            format!("{:?}", ls)
                        } else {
                            format!("({:?}) + ({:?})", ls, rs)
                        }
                    }
                    Operator::Subtract => {
                        if ls == rs {
                            format!("{:?}", ls)
                        } else {
                            format!("({:?}) - ({:?})", ls, rs)
                        }
                    }
                    Operator::Multiply => {
                        if ls == rs {
                            // Check if the symbols contain a float power
                            if ls.contains("^") {
                                let mut ls_split: Vec<_> = ls.split("^").map(|x| x.to_string()).collect();
                                if let Some(pow) = ls_split.pop() {
                                    if let Ok(f) = pow.parse::<f64>() {
                                        ls_split.push((f * 2.0).to_string());
                                        return ls_split.join("^")
                                    }
                                    // TODO: Handle case where power is not a number
                                    unimplemented!()
                                }
                                // TODO: Handle case where a malformed unit appears
                                unimplemented!()
                            } else {
                                format!("({:?}) ^ 2.0", ls)
                            }
                        } else {
                            format!("({:?}) * ({:?})", ls, rs)
                        }
                    }
                    Operator::Divide => {
                        format!("({:?}) / ({:?})", left.symbol(), right.symbol())
                    }
                    Operator::Power => {
                        format!("({:?}) ^ ({:?})", left.symbol(), right.symbol())
                    }
                }
            }
        }
    }
}

#[macro_export]
macro_rules! expr_unit {
    ($base_unit:expr) => {
        Expression::Unit($base_unit)
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