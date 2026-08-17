//! Strictness levels for VNU properties.
//!
//! Every property attached to a Vocabulary-Network-Unit is classified into
//! exactly one of three strictness levels. This classification determines how
//! the Constraint Engine treats proposed modifications to that property.
//!
//! # Strictness hierarchy
//!
//! The levels form a total order where "greater" means "more strict":
//!
//! ```text
//! Invariant > Constrained > Accidental
//! ```
//!
//! This ordering is materialized via [`Ord`]. The Constraint Engine relies on it
//! to enforce subtype inheritance rules: a subtype may narrow
//! a parent constraint (increase strictness) but must never widen it
//! (decrease strictness). Concretely, a proposed modification is rejected when
//! `new_strictness < parent_strictness`.
//!
//! # Epistemic cluster correspondence
//!
//! | Cluster | Dominant strictness | Mutability |
//! |------------------------|------------------------|------------------------|
//! | One — Fundamental Immutable Facts | [`Invariant`](StrictnessLevel::Invariant) | Never modified |
//! | Two — Advanced Science | [`Constrained`](StrictnessLevel::Constrained) | Bounded modification |
//! | Three — Creative Intelligence | [`Accidental`](StrictnessLevel::Accidental) | Freely modified |

use serde::{Deserialize, Serialize};

/// The mutability classification of a single VNU property.
///
/// This is a pure marker type with no payload. The actual property data
/// (values, bounds, confidence) lives in [`properties`](crate::properties).
///
/// # Ordering
///
/// Variants are declared in ascending strictness so that the derived [`Ord`]
/// matches the semantic hierarchy:
/// `Accidental < Constrained < Invariant`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StrictnessLevel {
    /// Exploratory property (Cluster Three).
    ///
    /// Modified continuously through Hebbian learning and hypothesis
    /// evaluation. The Constraint Engine records but does not reject
    /// modifications to accidental properties.
    Accidental,

    /// Bounded property (Cluster Two).
    ///
    /// May change, but only within explicitly declared minimum and maximum
    /// bounds. A modification that exceeds bounds yields
    /// `CONSTRAINT_BOUND_EXCEEDED`.
    Constrained,

    /// Axiomatic bedrock (Cluster One).
    ///
    /// Never modified during normal operation. Represents conservation laws,
    /// logical axioms, and geometric truths. Any proposed modification is
    /// rejected immediately with `CONSTRAINT_INVARIANT_VIOLATION`.
    Invariant,
}

impl StrictnessLevel {
    /// Returns `true` if a property at this level may have its value modified
    /// under any circumstances.
    ///
    /// - [`Invariant`](Self::Invariant) → `false`
    /// - [`Constrained`](Self::Constrained) → `true` (within bounds)
    /// - [`Accidental`](Self::Accidental) → `true` (without bounds)
    ///
    /// The Constraint Engine uses this as a fast-path check before performing
    /// the more expensive bound validation.
    #[inline]
    #[must_use]
    pub fn is_mutable(self) -> bool {
        self != Self::Invariant
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strictness_ordering() {
        assert!(StrictnessLevel::Invariant > StrictnessLevel::Constrained);
        assert!(StrictnessLevel::Constrained > StrictnessLevel::Accidental);
        assert!(StrictnessLevel::Invariant > StrictnessLevel::Accidental);
    }

    #[test]
    fn narrowing_is_permitted() {
        let parent = StrictnessLevel::Constrained;
        let child = StrictnessLevel::Invariant;
        // Narrowing: child >= parent → permitted
        assert!(child >= parent);
    }

    #[test]
    fn widening_is_rejected() {
        let parent = StrictnessLevel::Constrained;
        let child = StrictnessLevel::Accidental;
        // Widening: child < parent → rejected
        assert!(child < parent);
    }

    #[test]
    fn mutability() {
        assert!(!StrictnessLevel::Invariant.is_mutable());
        assert!(StrictnessLevel::Constrained.is_mutable());
        assert!(StrictnessLevel::Accidental.is_mutable());
    }

    #[test]
    fn serde_round_trip() {
        let level = StrictnessLevel::Constrained;
        let json = serde_json::to_string(&level).unwrap();
        assert_eq!(json, "\"constrained\"");
        let back: StrictnessLevel = serde_json::from_str(&json).unwrap();
        assert_eq!(back, level);
    }
}
