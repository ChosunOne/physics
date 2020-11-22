use crate::expr_unit;

use crate::units::si::si_unit::{SIBaseUnit, SIUnit, Expression};
use crate::units::dimensions::Dimension;
use crate::units::constants::{DELTA_V_CS, C, H, K, E, NA, KCD};

#[derive(Copy, Clone)]
pub enum BaseUnit {
    Second,
    Meter,
    Kilogram,
    Kelvin,
    Ampere,
    Mole,
    Candela,
    Float(f64)
}

impl SIBaseUnit for BaseUnit {
    fn dimension(&self) -> Dimension {
        match &self {
            BaseUnit::Second => {
                Dimension::Time
            }
            BaseUnit::Meter => {
                Dimension::Length
            }
            BaseUnit::Kilogram => {
                Dimension::Mass
            }
            BaseUnit::Kelvin => {
                Dimension::Temperature
            }
            BaseUnit::Ampere => {
                Dimension::Current
            }
            BaseUnit::Mole => {
                Dimension::Amount
            }
            BaseUnit::Candela => {
                Dimension::Luminosity
            }
            BaseUnit::Float(_) => {
                Dimension::None
            }
        }
    }

    fn symbol(&self) -> String {
        match &self {
            BaseUnit::Second => {
                "s".to_string()
            }
            BaseUnit::Meter => {
                "m".to_string()
            }
            BaseUnit::Kilogram => {
                "kg".to_string()
            }
            BaseUnit::Kelvin => {
                "K".to_string()
            }
            BaseUnit::Ampere => {
                "A".to_string()
            }
            BaseUnit::Mole => {
                "mol".to_string()
            }
            BaseUnit::Candela => {
                "cd".to_string()
            }
            BaseUnit::Float(_) => {
                "".to_string()
            }
        }
    }

    fn constant(&self) -> f64 {
        match &self {
            BaseUnit::Second => {
                DELTA_V_CS
            }
            BaseUnit::Meter => {
                DELTA_V_CS / C
            }
            BaseUnit::Kilogram => {
                C.powi(2) / (H * DELTA_V_CS)
            }
            BaseUnit::Kelvin => {
                K / (H * DELTA_V_CS)
            }
            BaseUnit::Ampere => {
                1.0 / (E * DELTA_V_CS)
            }
            BaseUnit::Mole => {
                NA
            }
            BaseUnit::Candela => {
                1.0 / (KCD * H * DELTA_V_CS.powi(2))
            }
            BaseUnit::Float(_) => {
                1.0
            }
        }
    }
}

impl SIUnit for BaseUnit {
    fn base_units(&self) -> Expression {
        expr_unit!(*self)
    }

    fn symbol(&self) -> String {
        SIBaseUnit::symbol(self)
    }
}