//! Core identity types for Vocabulary-Network-Units.
//!
//! This module defines the fundamental identity and hot-path representation
//! of VNUs - the atomic units of knowledge in the Prexager system.
//!
//! # The Two-State Architecture
//!
//! A VNU exists in two distinct physical states depending on what the system is doing:
//! - State A (Cold Path / DuckDB): A distributed set of relational rows across
//! `vnu_constants`, `vnu_constraints`, `vnu_attributes`, `vnu_observables`,
//! and `vnu_relations` tables. Optimized for storage efficiency, querying,
//! and strict validation.
//! - State B (Hot-Path / In-Memory RAM): A lightweight, contiguous struct acting
//! as a node in the graph during Markov exploration and activation spreading.
//! Optimized for memory bandwidth, cache locality, and microsecond traversal.
//!
//! The [`Vnu`] struct in this module represents State B only. It carries the minimum
//! metadata needed to route activation and identify the node. Heavy property data is
//! fetched from DuckDB on demand by the Constraint Engine and Imagination Engine.
//!
//! # Memory Budget
//!
//! The [`Vnu`] struct occupies 24 bytes on a 64-bit target. For a graph with
//! 1 million VNUs, the total node array occupies approximately 24 MB -
//! well within L3 cache.

use serde::{Deserialize, Serialize};
use std::num::{NonZeroU32, NonZeroU64};

use crate::epistemic::EpistemicTier;

//---------------------------------------------------------------------------------------------
// VnuId
//---------------------------------------------------------------------------------------------

/// Compact, globally unique identifier for a Vocabulary-Network-Unit.
///
/// Wraps [`NonZeroU64`] to guarantee that `Option<VnuId>` occupies exactly 8 bytes
/// (niche optimization). The zero value is reserved as the niche for `None`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct VnuId(NonZeroU64);

impl VnuId {
    /// Creates a `VnuId` from a raw non-zero integer.
    ///
    /// Used by the storage layer when loading VNUs from DuckDB.
    #[inline]
    #[must_use]
    pub const fn from_non_zero(raw: NonZeroU64) -> Self {
        Self(raw)
    }

    /// Attempts to create a `VnuId` from a `u64`.
    ///
    /// Returns `None` if `raw` is zero. Zero is not a valid VNU identifier.
    #[inline]
    #[must_use]
    pub const fn new(raw: u64) -> Option<Self> {
        match NonZeroU64::new(raw) {
            Some(nz) => Some(Self(nz)),
            None => None,
        }
    }

    /// Returns the underlying `u64` value.
    #[inline]
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

impl std::fmt::Display for VnuId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "vnu:{}", self.0.get())
    }
}

//---------------------------------------------------------------------------------------------
// WordId
//---------------------------------------------------------------------------------------------

/// Compact identifier for a word in the Central Dictionary.
///
/// Wraps [`NonZeroU32`] to guarantee that `Option<WordId>` occupies exactly 4 bytes,
/// (niche optimization). The zero value is reserved as the niche for `None`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WordId(NonZeroU32);

impl WordId {
    /// Creates a `WordId` from a raw non-zero integer.
    ///
    /// Used by the storage layer when loading dictionary entries.
    #[inline]
    #[must_use]
    pub const fn from_non_zero(raw: NonZeroU32) -> Self {
        Self(raw)
    }

    /// Attempts to create a `WordId` from a `u32`.
    ///
    /// Returns `None` if `raw` is zero. Zero is not a valid word identifier.
    #[inline]
    #[must_use]
    pub const fn new(raw: u32) -> Option<Self> {
        match NonZeroU32::new(raw) {
            Some(nz) => Some(Self(nz)),
            None => None,
        }
    }

    /// Returns the underlying `u32` value.
    #[inline]
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0.get()
    }
}

impl std::fmt::Display for WordId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "word:{}", self.0.get())
    }
}

//---------------------------------------------------------------------------------------------
// Vnu (Hot-Path Graph Node)
//---------------------------------------------------------------------------------------------

/// State B: The hot-path graph node.
///
/// This struct is the in-memory representation of a VNU during graph traversal.
/// It is kept as small as possible (24 bytes) to maximize CPU cache locality during
/// Markov exploration and activation spreading.
///
/// # Design constraints
///
/// - `f32` activation, not `f64`. Activation levels do not require double precision.
/// Using `f32` halves the memory footprint of the activation array, allowing
/// millions of nodes  to fit in CPU cache.
///
/// - No embedded properties, signatures, or relations. These live in the cold path
/// (DuckDB) and are fetched on demand. The graph node carries only routing metadata.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vnu {
    /// Unique identifier for this VNU in the global graph.
    pub id: VnuId,

    /// Reference to the Central Dictionary entry that provides this VNU's lexical
    /// label. The VNU does not store the word string directly;
    /// it references the dictionary via this hard link.
    pub label_id: WordId,

    /// Epistemic tier classification. Determines mutability rules,
    /// Correspondence Principle obligations, and bifurcation eligibility.
    ///
    /// - [`EpistemicTier::Fundamental`]: Cluster One. Never modified.
    /// - [`EpistemicTier::Advanced`]: Cluster Two. Bounded modified.
    /// - [`EpistemicTier::Creative`]: Cluster Three. Freely modified.
    pub tier: EpistemicTier,

    /// Current activation level in the spreading-activation network.
    ///
    /// Range: `[0.0, 1.0]`. Set to `1.0` by the Activation Engine when a content
    /// word maps to this VNU. Decays via lateral inhibition and diffusion
    /// smoothing during the thought cycle.
    ///
    /// This field is mutated during the thought cycle by the Activation Engine
    /// and read by the Exploration Engine.
    pub activation: f32,
}

impl Vnu {
    /// Creates a new inactive VNU node for the graph.
    ///
    /// The activation level is initialized to `0.0`. The Activation Engine
    /// will set it to `1.0` when a content word maps to this VNU.
    #[inline]
    #[must_use]
    pub const fn new(id: VnuId, label_id: WordId, tier: EpistemicTier) -> Self {
        Self {
            id,
            label_id,
            tier,
            activation: 0.0,
        }
    }

    /// Returns `true` if this VNU is currently active (activation > 0).
    ///
    /// Used by the Activation Engine and Exploration Engine to filter the active
    /// subgraph during Markov traversal.
    #[inline]
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.activation > 0.0
    }

    /// Sets the activation level, clamping to `[0.0, 1.0]`.
    ///
    /// Used by the Activation Engine during spreading activation.
    /// Values outside `[0.0, 1.0]` are clamped to prevent numerical instability
    /// in the Markov transition matrix.
    #[inline]
    pub fn set_activation(&mut self, value: f32) {
        self.activation = value.clamp(0.0, 1.0);
    }

    /// Resets the activation level to zero.
    ///
    /// Called at the start of each thought cycle to clear the previous cycle's
    /// activation pattern.
    #[inline]
    pub fn deactivate(&mut self) {
        self.activation = 0.0;
    }
}

//---------------------------------------------------------------------------------------------
// Tests
//---------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vnu_id_niche_optimization() {
        assert_eq!(
            std::mem::size_of::<VnuId>(),
            std::mem::size_of::<Option<VnuId>>(),
        );
        assert_eq!(std::mem::size_of::<VnuId>(), 8);
    }

    #[test]
    fn word_id_niche_optimization() {
        assert_eq!(
            std::mem::size_of::<WordId>(),
            std::mem::size_of::<Option<WordId>>(),
        );
        assert_eq!(std::mem::size_of::<WordId>(), 4);
    }

    #[test]
    fn vnu_id_zero_is_invalid() {
        assert!(VnuId::new(0).is_none());
        assert!(VnuId::new(1).is_some());
    }

    #[test]
    fn word_id_zero_is_invalid() {
        assert!(WordId::new(0).is_none());
        assert!(WordId::new(1).is_some());
    }

    #[test]
    fn vnu_memory_layout() {
        // Vnu struct should be 24 bytes on a 64-bit target
        assert_eq!(std::mem::size_of::<Vnu>(), 24);
    }

    #[test]
    fn vnu_construction() {
        let id = VnuId::new(1).unwrap();
        let word = WordId::new(1001).unwrap();
        let vnu = Vnu::new(id, word, EpistemicTier::Fundamental);

        assert_eq!(vnu.id, id);
        assert_eq!(vnu.label_id, word);
        assert_eq!(vnu.tier, EpistemicTier::Fundamental);
        assert_eq!(vnu.activation, 0.0);
        assert!(!vnu.is_active());
    }

    #[test]
    fn vnu_activation_lifecycle() {
        let id = VnuId::new(1).unwrap();
        let word = WordId::new(1001).unwrap();
        let mut vnu = Vnu::new(id, word, EpistemicTier::Creative);

        // Initially inactive
        assert!(!vnu.is_active());

        // Activate
        vnu.set_activation(1.0);
        assert!(vnu.is_active());
        assert_eq!(vnu.activation, 1.0);

        // Partial activation
        vnu.set_activation(0.5);
        assert!(vnu.is_active());
        assert_eq!(vnu.activation, 0.5);

        // Clamping: values above 1.0 are clamped
        vnu.set_activation(1.5);
        assert_eq!(vnu.activation, 1.0);

        // Clamping: negative values are clamped to 0.0
        vnu.set_activation(-0.5);
        assert_eq!(vnu.activation, 0.0);
        assert!(!vnu.is_active());

        // Deactivate
        vnu.set_activation(0.8);
        vnu.deactivate();
        assert_eq!(vnu.activation, 0.0);
        assert!(!vnu.is_active());
    }

    #[test]
    fn display_impls() {
        let id = VnuId::new(42).unwrap();
        assert_eq!(format!("{}", id), "vnu:42");

        let word = WordId::new(1001).unwrap();
        assert_eq!(format!("{}", word), "word:1001");
    }

    #[test]
    fn serde_round_trip_vnu_id() {
        let id = VnuId::new(12345).unwrap();
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "12345"); // transparent serialization
        let back: VnuId = serde_json::from_str(&json).unwrap();
        assert_eq!(back, id);
    }

    #[test]
    fn serde_round_trip_word_id() {
        let word = WordId::new(999).unwrap();
        let json = serde_json::to_string(&word).unwrap();
        assert_eq!(json, "999"); // transparent serialization
        let back: WordId = serde_json::from_str(&json).unwrap();
        assert_eq!(back, word);
    }

    #[test]
    fn serde_round_trip_vnu() {
        let id = VnuId::new(1).unwrap();
        let word = WordId::new(1001).unwrap();
        let vnu = Vnu::new(id, word, EpistemicTier::Fundamental);

        let json = serde_json::to_string(&vnu).unwrap();
        let back: Vnu = serde_json::from_str(&json).unwrap();

        assert_eq!(back.id, vnu.id);
        assert_eq!(back.label_id, vnu.label_id);
        assert_eq!(back.tier, vnu.tier);
        assert!((back.activation - vnu.activation).abs() < f32::EPSILON);
    }
}
