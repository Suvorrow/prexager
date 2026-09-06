//! Typed relations between Vocabulary-Network-Units (VNUs).
//!
//! This module defines the nine canonical relation types that form
//! the gauge connection structure of the VNU network, and the `VnuEdge` struct
//! that represents a directed, weighted connection between any two VNUs.

use serde::{Deserialize, Serialize};

use crate::entity::VnuId;

//---------------------------------------------------------------------------------------------
// RelationType
//---------------------------------------------------------------------------------------------

/// The eight canonical relation types in the VNU network.
///
/// This set is closed. Each variant defines a distinct gauge connection for
/// the Activation Engine. The variants are ordered by semantic category:
/// taxonomic, meronymic, causal, dependency, opposition, analogy, contrast,
/// temporal, and metric calibration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RelationType {
    /// Taxonomic hierarchy. `A IS_A B` means A is a subtype of B.
    /// Transitive. Not symmetric.
    IsA,

    /// Meronymy. `A HAS-PART B` means B is a component of A.
    /// Not transitive in general. Not symmetric.
    HasPart,

    /// Causal direction. `A CAUSES B` means A is a cause of B.
    /// Must form a DAG (no cycles). Not symmetric.
    Causes,

    /// Dependency. `A REQUIRES B` means A cannot exist or function without B.
    /// Not symmetric.
    Requires,

    /// Logical opposition. `A CONTRADICTS B` means A and B cannot both be true
    /// simultaneously. Symmetric.
    Contradicts,

    /// Structural similarity across domain. `A ANALOGOUS-TO B` means A and B
    /// share a relational structure despite different domains. Symmetric.
    AnalogousTo,

    /// Definitional opposition. `A CONTRASTS-WITH B` means A and B are distinguished
    /// by at least one invariant property. Symmetric.
    /// Weaker than CONTRADICTS: contrasted concepts can coexist.
    ContrastsWith,

    /// Sequential ordering. `A TEMPORAL-AFTER B` means A occurs after B in time.
    /// Transitive. Not symmetric. Antisymmetric.
    TemporalAfter,

    /// Metric calibration. `A HAS-REFERENCE-POINT B` means B is a calibration
    /// anchor (e.g., zero-point, gold standard) for the Metric Frame A.
    /// Used by the RFDE mechanism. Not symmetric. Not transitive. Antisymmetric.
    HasReferencePoint,
}

impl RelationType {
    /// Returns `true` if the relation is symmetric: `A R B` implies `B R A`.
    ///
    /// Symmetric relations: CONTRADICTS, ANALOGOUS-TO, CONTRASTS-WITH.
    #[inline]
    #[must_use]
    pub const fn is_symmetric(self) -> bool {
        matches!(
            self,
            Self::Contradicts | Self::AnalogousTo | Self::ContrastsWith
        )
    }

    /// Returns `true` if the relation is transitive: `A R B` and `B R C` imply `A R C`.
    ///
    /// Transitive relations: IS-A, TEMPORAL-AFTER.
    #[inline]
    #[must_use]
    pub const fn is_transitive(self) -> bool {
        matches!(self, Self::IsA | Self::TemporalAfter)
    }

    /// Returns `true` if the relation is antisymmetric: `A R B` and `B R A`
    /// imply `A = B`.
    ///
    /// Antisymmetric relations: IS-A, CAUSES, REQUIRES, TEMPORAL-AFTER.
    #[inline]
    #[must_use]
    pub const fn is_antisymmetric(self) -> bool {
        matches!(
            self,
            Self::IsA
                | Self::Causes
                | Self::Requires
                | Self::TemporalAfter
                | Self::HasReferencePoint
        )
    }

    /// Returns `true` if this relation type participates in the causal DAG.
    /// The Epistemic Arbiter enforces acyclicity on these relations.
    #[inline]
    #[must_use]
    pub const fn is_causal(self) -> bool {
        matches!(self, Self::Causes | Self::Requires)
    }

    /// Returns `true` if this relation type defines a structural opposition.
    /// The Constraint Patrol checks for simultaneous activation of VNUs connected
    /// by these relations.
    #[inline]
    #[must_use]
    pub const fn is_oppositional(self) -> bool {
        matches!(self, Self::Contradicts | Self::ContrastsWith)
    }

    /// Returns a human-readable label for logging and the Developer API.
    #[inline]
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::IsA => "IS-A",
            Self::HasPart => "HAS-PART",
            Self::Causes => "CAUSES",
            Self::Requires => "REQUIRES",
            Self::Contradicts => "CONTRADICTS",
            Self::AnalogousTo => "ANALOGOUS-TO",
            Self::ContrastsWith => "CONTRASTS-WITH",
            Self::TemporalAfter => "TEMPORAL-AFTER",
            Self::HasReferencePoint => "HAS-REFERENCE-POINT",
        }
    }

    /// Returns all nine relation types as a fixed-size array.
    ///
    /// Useful for iteration when initializing relation-specific projection matrices.
    #[inline]
    #[must_use]
    pub const fn all() -> [Self; 9] {
        [
            Self::IsA,
            Self::HasPart,
            Self::Causes,
            Self::Requires,
            Self::Contradicts,
            Self::AnalogousTo,
            Self::ContrastsWith,
            Self::TemporalAfter,
            Self::HasReferencePoint,
        ]
    }
}

impl std::fmt::Display for RelationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label())
    }
}

//---------------------------------------------------------------------------------------------
// VnuEdge
//---------------------------------------------------------------------------------------------

/// A typed, weighted directed edge between two VNUs.
///
/// The edge is the fundamental unit of the VNU graph. It carries a relation type
/// (the gauge connection) and a weight (the connection strength, updated by Hebbian learning).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct VnuEdge {
    /// The source VNU. The edge points away from this node.
    pub source: VnuId,

    /// The target VNU. The edge points toward this node.
    pub target: VnuId,

    /// The relation type. Determines the gauge connection used by the
    /// Activation Engine when spreading activation along this edge.
    pub relation: RelationType,

    /// Connection strength. Updated by Hebbian learning.
    /// Range: `[0.0, f32::MAX)`. A weight of zero means the edge is effectively
    /// absent but retained for history.
    pub weight: f32,
}

impl VnuEdge {
    /// Creates a new edge with the given parameters.
    #[inline]
    #[must_use]
    pub const fn new(source: VnuId, target: VnuId, relation: RelationType, weight: f32) -> Self {
        Self {
            source,
            target,
            relation,
            weight,
        }
    }

    /// Returns the edge with source and target swapped.
    ///
    /// For symmetric relations, the swapped edge is semantically identical.
    /// For asymmetric relations, the swapped edge represents the inverse relationship
    /// (e.g., `A IS-A B` becomes `B IS-A A`, which reads as "B has subtype A").
    #[inline]
    #[must_use]
    pub const fn reversed(self) -> Self {
        Self {
            source: self.target,
            target: self.source,
            relation: self.relation,
            weight: self.weight,
        }
    }

    /// Returns `true` if this edge connects the given VNU (as source or target).
    #[inline]
    #[must_use]
    pub fn touches(&self, id: VnuId) -> bool {
        self.source == id || self.target == id
    }

    /// Returns the other endpoint of the edge given one endpoint.
    ///
    /// Returns `None` if `id` is neither the source nor the target.
    #[inline]
    #[must_use]
    pub fn other(&self, id: VnuId) -> Option<VnuId> {
        if self.source == id {
            Some(self.target)
        } else if self.target == id {
            Some(self.source)
        } else {
            None
        }
    }

    /// Returns `true` if this edge is effectively absent (weight is zero
    /// or negative). Retained edges with zero weight are kept for Hebbian history
    /// but excluded from active traversal.
    #[inline]
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.weight > 0.0
    }
}

//---------------------------------------------------------------------------------------------
// Tests
//---------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to create a valid `VnuId` from a `u64`.
    /// Panics if `n` is zero, which is exactly what we want in tests.
    fn id(n: u64) -> VnuId {
        VnuId::new(n).expect("test id must be non-zero")
    }

    #[test]
    fn relation_type_count() {
        // Updated to 9 to include HAS-REFERENCE-POINT
        assert_eq!(RelationType::all().len(), 9);
    }

    #[test]
    fn all_relation_types_are_unique() {
        let all = RelationType::all();
        for (i, r1) in all.iter().enumerate() {
            for (j, r2) in all.iter().enumerate() {
                if i != j {
                    assert_ne!(r1, r2, "Duplicate relation type found in all()");
                }
            }
        }
    }

    #[test]
    fn symmetric_relations() {
        // Positive cases
        assert!(RelationType::Contradicts.is_symmetric());
        assert!(RelationType::AnalogousTo.is_symmetric());
        assert!(RelationType::ContrastsWith.is_symmetric());

        // Negative cases
        assert!(!RelationType::IsA.is_symmetric());
        assert!(!RelationType::HasPart.is_symmetric());
        assert!(!RelationType::Causes.is_symmetric());
        assert!(!RelationType::Requires.is_symmetric());
        assert!(!RelationType::TemporalAfter.is_symmetric());
        assert!(!RelationType::HasReferencePoint.is_symmetric());
    }

    #[test]
    fn transitive_relations() {
        // Positive cases
        assert!(RelationType::IsA.is_transitive());
        assert!(RelationType::TemporalAfter.is_transitive());

        // Negative cases
        assert!(!RelationType::HasPart.is_transitive());
        assert!(!RelationType::Causes.is_transitive());
        assert!(!RelationType::Requires.is_transitive());
        assert!(!RelationType::Contradicts.is_transitive());
        assert!(!RelationType::AnalogousTo.is_transitive());
        assert!(!RelationType::ContrastsWith.is_transitive());
        assert!(!RelationType::HasReferencePoint.is_transitive());
    }

    #[test]
    fn antisymmetric_relations() {
        // Positive cases
        assert!(RelationType::IsA.is_antisymmetric());
        assert!(RelationType::Causes.is_antisymmetric());
        assert!(RelationType::Requires.is_antisymmetric());
        assert!(RelationType::TemporalAfter.is_antisymmetric());
        assert!(RelationType::HasReferencePoint.is_antisymmetric());

        // Negative cases
        assert!(!RelationType::HasPart.is_antisymmetric());
        assert!(!RelationType::Contradicts.is_antisymmetric());
        assert!(!RelationType::AnalogousTo.is_antisymmetric());
        assert!(!RelationType::ContrastsWith.is_antisymmetric());
    }

    #[test]
    fn causal_relations() {
        // Positive cases
        assert!(RelationType::Causes.is_causal());
        assert!(RelationType::Requires.is_causal());

        // Negative cases
        assert!(!RelationType::IsA.is_causal());
        assert!(!RelationType::HasPart.is_causal());
        assert!(!RelationType::Contradicts.is_causal());
        assert!(!RelationType::AnalogousTo.is_causal());
        assert!(!RelationType::ContrastsWith.is_causal());
        assert!(!RelationType::TemporalAfter.is_causal());
        assert!(!RelationType::HasReferencePoint.is_causal());
    }

    #[test]
    fn oppositional_relations() {
        // Positive cases
        assert!(RelationType::Contradicts.is_oppositional());
        assert!(RelationType::ContrastsWith.is_oppositional());

        // Negative cases
        assert!(!RelationType::IsA.is_oppositional());
        assert!(!RelationType::HasPart.is_oppositional());
        assert!(!RelationType::Causes.is_oppositional());
        assert!(!RelationType::Requires.is_oppositional());
        assert!(!RelationType::AnalogousTo.is_oppositional());
        assert!(!RelationType::TemporalAfter.is_oppositional());
        assert!(!RelationType::HasReferencePoint.is_oppositional());
    }

    #[test]
    fn edge_reversal() {
        let edge = VnuEdge::new(id(1), id(2), RelationType::IsA, 1.0);
        let rev = edge.reversed();

        assert_eq!(rev.source, id(2));
        assert_eq!(rev.target, id(1));
        assert_eq!(rev.relation, RelationType::IsA);
        assert_eq!(rev.weight, 1.0);
    }

    #[test]
    fn edge_touches_and_other() {
        let edge = VnuEdge::new(id(1), id(2), RelationType::Causes, 0.5);

        assert!(edge.touches(id(1)));
        assert!(edge.touches(id(2)));
        assert!(!edge.touches(id(3)));

        assert_eq!(edge.other(id(1)), Some(id(2)));
        assert_eq!(edge.other(id(2)), Some(id(1)));
        assert_eq!(edge.other(id(3)), None);
    }

    #[test]
    fn edge_activity() {
        let active = VnuEdge::new(id(1), id(2), RelationType::IsA, 0.5);
        let dead_zero = VnuEdge::new(id(1), id(2), RelationType::IsA, 0.0);
        let dead_negative = VnuEdge::new(id(1), id(2), RelationType::IsA, -0.1);

        assert!(active.is_active());
        assert!(!dead_zero.is_active());
        assert!(!dead_negative.is_active());
    }

    #[test]
    fn edge_activity_nan() {
        // IEEE 754 NaN is not greater than 0.0, so it must be considered inactive.
        let nan_edge = VnuEdge::new(id(1), id(2), RelationType::IsA, f32::NAN);
        assert!(!nan_edge.is_active());
    }

    #[test]
    fn serde_round_trip_relation_type() {
        let rt = RelationType::AnalogousTo;
        let json = serde_json::to_string(&rt).unwrap();
        // Kebab-case matches the architectural document's hyphenated notation
        assert_eq!(json, "\"analogous-to\"");
        let back: RelationType = serde_json::from_str(&json).unwrap();
        assert_eq!(back, rt);

        // Verify the new 9th relation type serializes correctly
        let rt_ref = RelationType::HasReferencePoint;
        let json_ref = serde_json::to_string(&rt_ref).unwrap();
        assert_eq!(json_ref, "\"has-reference-point\"");
        let back_ref: RelationType = serde_json::from_str(&json_ref).unwrap();
        assert_eq!(back_ref, rt_ref);
    }

    #[test]
    fn serde_round_trip_edge() {
        let edge = VnuEdge::new(id(10), id(20), RelationType::HasReferencePoint, 0.75);
        let json = serde_json::to_string(&edge).unwrap();
        let back: VnuEdge = serde_json::from_str(&json).unwrap();

        assert_eq!(back.source, id(10));
        assert_eq!(back.target, id(20));
        assert_eq!(back.relation, RelationType::HasReferencePoint);
        assert!((back.weight - 0.75).abs() < f32::EPSILON);
    }

    #[test]
    fn display_impls() {
        // Relies on VnuId implementing Display in entity.rs
        assert_eq!(format!("{}", id(42)), "vnu:42");
        assert_eq!(format!("{}", RelationType::Causes), "CAUSES");
        assert_eq!(format!("{}", RelationType::TemporalAfter), "TEMPORAL-AFTER");
        assert_eq!(
            format!("{}", RelationType::HasReferencePoint),
            "HAS-REFERENCE-POINT"
        );
    }
}
