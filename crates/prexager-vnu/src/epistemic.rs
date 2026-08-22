//! Epistemic tier classification for Vocabulary-Network-Units.
//!
//! Every VNU in the Prexager Knowledge Base is assigned to exactly one of three
//! epistemic tiers. This classification determines:
//! - "Mutability rules" - which strictness levels dominate and whether the VNU
//! may be modified during normal operation.
//! - "Correspondence Principle obligations" - which tier a modification must
//! mathematically reduce to.
//! - "Bifurcation eligibility" - whether the Bifurcation Engine may analyze
//! activation dynamics for this tier.
//!
//! # Tier hierarchy
//!
//! The tiers form a total order where "greater" means "more foundational":
//! '''
//! Fundamental > Advanced > Creative
//! '''
//!
use serde::{Deserialize, Serialize};

/// The epistemic tier of a Vocabulary-Netvork-Unit (VNU).
///
/// Determines the mutability rules, Correspondence Principle obligations,
/// and bifurcation eligibility for the VNU.
///
/// # Ordering
///
/// Variants are declared in ascending foundational status so that the derived
/// ['Ord'] matches the semantic hierarchy:
/// 'Creative < Advanced < Fundamental'.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EpistemicTier {
    /// Cluster Three: Creative Intelligence.
    ///
    /// Contains hypotheses, speculative models, novel syntheses, and the system's
    /// own operational history. VNUs at this tier are modified continuously
    /// through Hebbian learning and hypothesis evaluation.
    /// This is the ONLY tier eligible for bifurcation analysis.
    Creative,

    /// Cluster Two: Advanced Science.
    ///
    /// Contains quantum mechanics, relativity, molecular biology, genetics,
    /// chaos theory, and non-Euclidean geometry. VNUs at this tier have high
    /// density of constrained properties with complex dependencies. Governed
    /// by the Correspondence Principle: must mathematically reduce to Cluster
    /// One in the appropriate domain of validity.
    Advanced,

    /// Cluster One: Basic Fundamental Immutable Facts.
    ///
    /// Contains classical science, Newtonian mechanics, thermodynamics, formal
    /// logic, and Euclidean geometry. VNUs at this tier have high density of
    /// invariant properties. They are the axiomatic bedrock: never modified
    /// during normal operation, only potentially extendable.
    Fundamental,
}

impl EpistemicTier {
    /// Returns 'true' if VNUs at this tier may be modified during normal operation.
    ///
    /// - ['Fundamental'](Self::Fundamental) --> 'false' (axiomatic bedrock)
    /// - ['Advanced'](Self::Advanced) --> 'true' (within constrained bounds)
    /// - ['Creative'](Self::Creative) --> 'true' (freely modified)
    ///
    /// The Constraint Engine uses this as a fast-path gate before performing
    /// property-level validation.
    #[inline]
    #[must_use]
    pub fn is_mutable(self) -> bool {
        self != Self::Fundamental
    }

    /// Returns 'true' if this tier is eligible for bifurcation analysis.
    ///
    /// The Bifurcation Engine is active ONLY for Cluster Three VNUs. Clusters
    /// One and Two are threated as stable attractors whose activation dynamics
    /// are not subject to bifurcation detection.
    #[inline]
    #[must_use]
    pub fn allows_bifurcation(self) -> bool {
        self == Self::Creative
    }

    /// Returns the tier that modifications to this tier must reduce to, per the
    /// Correspondence Principle.
    ///
    /// The Epistemic Arbiter uses this to verify that any proposed modification
    /// is consistent with the foundational tier it must mathematically reduce to.
    #[inline]
    #[must_use]
    pub fn must_reduce_to(self) -> Option<Self> {
        match self {
            Self::Fundamental => None,
            Self::Advanced => Some(Self::Fundamental),
            Self::Creative => Some(Self::Advanced),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tier_ordering() {
        assert!(EpistemicTier::Fundamental > EpistemicTier::Advanced);
        assert!(EpistemicTier::Advanced > EpistemicTier::Creative);
        assert!(EpistemicTier::Fundamental > EpistemicTier::Creative);
    }

    #[test]
    fn mutability() {
        assert!(EpistemicTier::Fundamental.is_mutable());
        assert!(EpistemicTier::Advanced.is_mutable());
        assert!(EpistemicTier::Creative.is_mutable());
    }

    #[test]
    fn bifurcation_eligibility() {
        assert!(EpistemicTier::Fundamental.allows_bifurcation());
        assert!(EpistemicTier::Advanced.allows_bifurcation());
        assert!(EpistemicTier::Creative.allows_bifurcation());
    }

    #[test]
    fn correspondence_principle() {
        assert_eq!(EpistemicTier::Fundamental.must_reduce_to(), None);
        assert_eq!(
            EpistemicTier::Advanced.must_reduce_to(),
            Some(EpistemicTier::Fundamental)
        );
        assert_eq!(
            EpistemicTier::Creative.must_reduce_to(),
            Some(EpistemicTier::Advanced)
        );
    }

    #[test]
    fn serde_round_trip() {
        let tier = EpistemicTier::Advanced;
        let json = serde_json::to_string(&tier).unwrap();
        assert_eq!(json, "\"advanced\"");
        let back: EpistemicTier = serde_json::from_str(&json).unwrap();
        assert_eq!(back, tier);
    }
}
