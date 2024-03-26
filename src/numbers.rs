use serde::{Deserialize, Serialize};
use std::ops::Deref;

use super::{DecimalType, NumberType};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DecimalDatatype(f64);

impl Deref for DecimalDatatype {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<f64> for DecimalDatatype {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl DecimalType for DecimalDatatype {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct IntegerDatatype(i64);

impl Deref for IntegerDatatype {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<i64> for IntegerDatatype {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl NumberType for IntegerDatatype {}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct NonNegativeIntegerDatatype(u64);

impl Deref for NonNegativeIntegerDatatype {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u64> for NonNegativeIntegerDatatype {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl NumberType for NonNegativeIntegerDatatype {
    fn minimum() -> Option<i64> {
        Some(0)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PositiveIntegerDatatype(u64);

impl Deref for PositiveIntegerDatatype {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u64> for PositiveIntegerDatatype {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl NumberType for PositiveIntegerDatatype {
    fn minimum() -> Option<i64> {
        Some(1)
    }
}
