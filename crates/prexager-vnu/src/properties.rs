//! Strongly-typed property keys and values for VNU constraint math.
//!
//! This module eliminates stringly-typed data from the hot path. Property names
//! are resolved to integer ['PropertyKey']s at load time, and values are stored
//! in the strongly-typed ['PropertyValue'] enum.

use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

/// An integer-based key for property types.
///
/// The inner field is private to enforce that all keys are created via the central
/// Dictionary-Registry, preventing invalid or unmapped keys from entering
/// the Constraint Engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PropertyKey(NonZeroU32);

impl PropertyKey {
    /// Creates a new PropertyKey from a raw non-zero integer.
    /// Typically used only by the Dictionary loader.
    #[inline]
    pub const fn from_raw(raw: NonZeroU32) -> Self {
        Self(raw)
    }

    /// Returns the underlying raw integer for fast switch/match statements.
    #[inline]
    pub const fn get(self) -> u32 {
        self.0.get()
    }

    // Well-known fundamentalproperties are reserved in the lower ID space.
    // The Dictionary maps these to human-readable strings for the UI/LLM.
    pub const DIMENSIONALITY: Self = Self(NonZeroU32::MIN); // 1
    pub const HAS_VOLUME: Self = Self(NonZeroU32::new(2).unwrap());
    pub const HAS_AREA: Self = Self(NonZeroU32::new(3).unwrap());
    pub const HAS_PARTS: Self = Self(NonZeroU32::new(4).unwrap());
    pub const REST_MASS: Self = Self(NonZeroU32::new(5).unwrap());
    pub const COORDINATES: Self = Self(NonZeroU32::new(6).unwrap());
}

/// A strongly-typed property value optimized for constraint math.
///
/// # Note on 'Eq' and 'Hash'
/// This enum intentionally does NOT implement 'Eq' or 'Hash'. The 'Float(f64)'
/// variant contains an IEEE 754 floating-ppoint number, which does not support
/// total ordering or hashing (due to 'NaN' and '-0.0' vs '0.0'). The Constraint
/// Engine must use explicit epsilon comparisons for float bounds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PropertyValue {
    /// Boolean state (e.g., 'has_volume = false').
    Bool(bool),
    /// Exact integer counts (e.g., 'dimensionality = 0').
    Int(i64),
    /// Continuous physical measurements (e.g., 'rest_mass = 9.109e-31').
    Float(f64),
    /// Multi-dimensional observables (e.g., 'coordinates = [x, y, z]').
    Vector(Vec<f64>),
    /// UI artifacts, labels, and descriptions. Kept out of the hot math path.
    Text(String),
    /// Explicit absence of value.
    Null,
}

impl PropertyValue {
    /// Attempts to extract a boolean value.
    #[inline]
    pub const fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Attempts to extract an integer value.
    #[inline]
    pub const fn as_i64(&self) -> Option<i64> {
        match self {
            Self::Int(i) => Some(*i),
            _ => None,
        }
    }

    /// Attempts to extract a float value.
    #[inline]
    pub const fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Float(f) => Some(*f),
            // Allow implicit widening from Int to Float for constraint match
            Self::Int(i) => Some(*i as f64),
            _ => None,
        }
    }

    /// Attampts to extract a slice of floats
    #[inline]
    pub const fn as_vector(&self) -> Option<&[f64]> {
        match self {
            // &Vec<f64> automatically coerces to &[f64] via Deref
            // which is permitted in const fn contexts.
            Self::Vector(v) => Some(v),
            _ => None,
        }
    }

    /// Returns 'true' if the value is explicitly 'Null'.
    #[inline]
    pub const fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn property_key_niche_optimization() {
        // Verify that Option<PropertyKey> is the same size as PropertyKey (4 bytes)
        assert_eq!(
            std::mem::size_of::<PropertyKey>(),
            std::mem::size_of::<Option<PropertyKey>>()
        );
        assert_eq!(std::mem::size_of::<PropertyKey>(), 4);
    }

    #[test]
    fn property_value_extraction() {
        let val = PropertyValue::Float(3.14);
        assert_eq!(val.as_f64(), Some(3.14));
        assert_eq!(val.as_f64(), None);

        let int_val = PropertyValue::Int(42);
        // Int should widen to Float successfully
        assert_eq!(int_val.as_f64(), Some(42.0));
    }

    #[test]
    fn property_vector_coercion() {
        let val = PropertyValue::Vector(vec![1.0, 2.0, 3.0]);
        let slice = val.as_vector().unwrap();
        assert_eq!(slice.len(), 3);
        assert_eq!(slice[1], 2.0);
    }
}
