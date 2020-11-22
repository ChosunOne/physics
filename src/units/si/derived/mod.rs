use crate::expr_op;
use crate::expr_unit;
use crate::units::si::si_unit::{SIUnit, Expression, Operator};
use crate::units::si::base::BaseUnit::{Float, Second, Meter, Kilogram, Kelvin, Ampere, Mole, Candela};

pub enum DerivedUnit {
    Becquerel,
    Celsius,
    Coulomb,
    Farad,
    Gray,
    Henry,
    Hertz,
    Joule,
    Katal,
    Lumen,
    Lux,
    Newton,
    Ohm,
    Pascal,
    Radian,
    Siemens,
    Sievert,
    Steradian,
    Tesla,
    Volt,
    Watt,
    Weber
}

impl SIUnit for DerivedUnit {
    fn base_units(&self) -> Expression {
        match self {
            DerivedUnit::Becquerel => {
                expr_op!(
                    expr_unit!(Float(1.0)),
                    Operator::Divide,
                    expr_unit!(Second)
                )
            }
            DerivedUnit::Celsius => {
                expr_unit!(Kelvin)
            }
            DerivedUnit::Coulomb => {
                expr_op!(
                    expr_unit!(Second),
                    Operator::Multiply,
                    expr_unit!(Ampere)
                )
            }
            DerivedUnit::Farad => {
                let c = DerivedUnit::Coulomb.base_units();
                let v = DerivedUnit::Volt.base_units();
                expr_op!(
                    c,
                    Operator::Divide,
                    v
                )
            }
            DerivedUnit::Gray => {
                let j = DerivedUnit::Joule.base_units();
                expr_op!(
                    j,
                    Operator::Divide,
                    expr_unit!(Kilogram)
                )
            }
            DerivedUnit::Henry => {
                let wb = DerivedUnit::Weber.base_units();
                expr_op!(
                    wb,
                    Operator::Divide,
                    expr_unit!(Ampere)
                )
            }
            DerivedUnit::Hertz => {
                expr_op!(
                    expr_unit!(Float(1.0)),
                    Operator::Divide,
                    expr_unit!(Second)
                )
            }
            DerivedUnit::Joule => {
                let n = DerivedUnit::Newton.base_units();
                expr_op!(
                    n,
                    Operator::Multiply,
                    expr_unit!(Meter)
                )
            }
            DerivedUnit::Katal => {
                expr_op!(
                    expr_unit!(Mole),
                    Operator::Divide,
                    expr_unit!(Second)
                )
            }
            DerivedUnit::Lumen => {
                let sr = DerivedUnit::Steradian.base_units();
                expr_op!(
                    expr_unit!(Candela),
                    Operator::Multiply,
                    sr
                )
            }
            DerivedUnit::Lux => {
                let lm = DerivedUnit::Lumen.base_units();
                expr_op!(
                    lm,
                    Operator::Divide,
                    expr_op!(
                        expr_unit!(Meter),
                        Operator::Power,
                        expr_unit!(Float(2.0))
                    )
                )
            }
            DerivedUnit::Newton => {
                expr_op!(
                    expr_op!(
                        expr_unit!(Kilogram),
                        Operator::Multiply,
                        expr_unit!(Meter)
                    ),
                    Operator::Divide,
                    expr_op!(
                        expr_unit!(Second),
                        Operator::Power,
                        expr_unit!(Float(2.0))
                    )
                )
            }
            DerivedUnit::Ohm => {
                let v = DerivedUnit::Volt.base_units();
                expr_op!(
                    v,
                    Operator::Divide,
                    expr_unit!(Ampere)
                )
            }
            DerivedUnit::Pascal => {
                let n = DerivedUnit::Newton.base_units();
                expr_op!(
                    n,
                    Operator::Divide,
                    expr_op!(
                        expr_unit!(Meter),
                        Operator::Power,
                        expr_unit!(Float(2.0))
                    )
                )
            }
            DerivedUnit::Radian => {
                expr_op!(
                    expr_unit!(Meter),
                    Operator::Divide,
                    expr_unit!(Meter)
                )
            }
            DerivedUnit::Siemens => {
                let o = DerivedUnit::Ohm.base_units();
                expr_op!(
                    expr_unit!(Float(1.0)),
                    Operator::Divide,
                    o
                )
            }
            DerivedUnit::Sievert => {
                let j = DerivedUnit::Joule.base_units();
                expr_op!(
                    j,
                    Operator::Divide,
                    expr_unit!(Kilogram)
                )
            }
            DerivedUnit::Steradian => {
                expr_op!(
                    expr_op!(
                        expr_unit!(Meter),
                        Operator::Power,
                        expr_unit!(Float(2.0))
                    ),
                    Operator::Divide,
                    expr_op!(
                        expr_unit!(Meter),
                        Operator::Power,
                        expr_unit!(Float(2.0))
                    )
                )
            }
            DerivedUnit::Tesla => {
                let wb = DerivedUnit::Weber.base_units();
                expr_op!(
                    wb,
                    Operator::Divide,
                    expr_op!(
                        expr_unit!(Meter),
                        Operator::Power,
                        expr_unit!(Float(2.0))
                    )
                )
            }
            DerivedUnit::Volt => {
                let w = DerivedUnit::Watt.base_units();
                expr_op!(
                    w,
                    Operator::Divide,
                    expr_unit!(Ampere)
                )
            }
            DerivedUnit::Watt => {
                let j = DerivedUnit::Joule.base_units();
                expr_op!(
                    j,
                    Operator::Divide,
                    expr_unit!(Second)
                )
            }
            DerivedUnit::Weber => {
                let v = DerivedUnit::Volt.base_units();
                expr_op!(
                    v,
                    Operator::Multiply,
                    expr_unit!(Second)
                )
            }
        }
    }

    fn symbol(&self) -> String {
        match self {
            DerivedUnit::Becquerel => {
                "Bq".to_string()
            }
            DerivedUnit::Celsius => {
                "°C".to_string()
            }
            DerivedUnit::Coulomb => {
                "C".to_string()
            }
            DerivedUnit::Farad => {
                "F".to_string()
            }
            DerivedUnit::Gray => {
                "Gy".to_string()
            }
            DerivedUnit::Henry => {
                "H".to_string()
            }
            DerivedUnit::Hertz => {
                "Hz".to_string()
            }
            DerivedUnit::Joule => {
                "J".to_string()
            }
            DerivedUnit::Katal => {
                "kat".to_string()
            }
            DerivedUnit::Lumen => {
                "lm".to_string()
            }
            DerivedUnit::Lux => {
                "lx".to_string()
            }
            DerivedUnit::Newton => {
                "N".to_string()
            }
            DerivedUnit::Ohm => {
                "Ω".to_string()
            }
            DerivedUnit::Pascal => {
                "Pa".to_string()
            }
            DerivedUnit::Radian => {
                "rad".to_string()
            }
            DerivedUnit::Siemens => {
                "S".to_string()
            }
            DerivedUnit::Sievert => {
                "Sv".to_string()
            }
            DerivedUnit::Steradian => {
                "sr".to_string()
            }
            DerivedUnit::Tesla => {
                "T".to_string()
            }
            DerivedUnit::Volt => {
                "V".to_string()
            }
            DerivedUnit::Watt => {
                "W".to_string()
            }
            DerivedUnit::Weber => {
                "Wb".to_string()
            }
        }
    }
}