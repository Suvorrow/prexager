//! Observable signatures for VNU spatial grounding.
//!
//! Every physical object or concept VNU may possess an Observable Signature:
//! a multi-spectral, multi-modal vector of physical primitives. This is the
//! spatial grounding mechanism consumed by the Imagination Engine.
//!
//! The four signature components correspond to the four physical channels
//! used by the Imagination Engine during the grounding step:
//! - `GeometricSignature` --> Renders the geometric skeleton from the topological signature.
//! - `MechanicalSignature` --> Fills the internal structure from the mechanical signature.
//! - `ElectromagneticSignature` --> Applies surface properties from the electromagnetic signature.
//! - `AcousticSignature` --> Overlays resonance patterns from the acoustic signature.
//!
//! # Dual Grounding Principle
//!
//! Observable Signatures provide spatial grounding (objects). Concepts that are
//! processes or dependencies are grounded via Process Signatures and Process-Component
//! Signatures (to be defined in a future `process` module).
//!
//! The Imagination Engine must complete before any reasoning begins. No thought cycle
//! may skip the imagination step. If a concept has no Observable Signature,
//! no Process Signature, no Process-Component Signature, and no metaphorical primitive,
//! it is flagged as ungroundable and assigned low confidence.
//!
//! `ObservableSignature::is_grounded` --> checks only the spatial channel
//! (whether this specific signature has any content). The overall groundability
//! of a VNU requires checking all four grounding mechanisms at the entity level.

use serde::{Deserialize, Serialize};

//---------------------------------------------------------------------------------------------
// Top-level Observable Signature
//---------------------------------------------------------------------------------------------

/// The complete Observable Signature of a VNU.
///
/// Bundles the four physical channels for spatial grounding. Each channel
/// is independently optional: a math concept may only have geometry.
/// An empty signature means the concept lacks spatial grounding, but it may still
/// be dynamically grounded via a Process Signature.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ObservableSignature {
    /// Geometric and topological properties. Used to render the skeleton.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometric: Option<GeometricSignature>,

    /// Mass, density, and material properties. Used to fill internal structure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mechanical: Option<MechanicalSignature>,

    /// Reflectance and emission properties. Used to apply surface properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub electromagnetic: Option<ElectromagneticSignature>,

    /// Resonance and damping properties. Used to overlay resonance patterns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acoustic: Option<AcousticSignature>,
}

impl ObservableSignature {
    /// Returns `true` if at least one signature component is present.
    ///
    /// Checks for spatial grounding only. If `false`, the concept lacks an
    /// Observable Signature but may still be dynamically grounded via a
    /// Process Signature or a metaphorical primitive.
    #[inline]
    #[must_use]
    pub fn is_grounded(&self) -> bool {
        self.geometric.is_some()
            || self.mechanical.is_some()
            || self.electromagnetic.is_some()
            || self.acoustic.is_some()
    }
}

//---------------------------------------------------------------------------------------------
// Geometric / Topological Signature
//---------------------------------------------------------------------------------------------

/// Geometric and topological properties of a VNU.
///
/// The Imagination Engine renders the geometric skeleton from this data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct GeometricSignature {
    /// Number of spatial dimensions: 0 (point), 1 (line), 2 (surface), 3 (volume).
    /// Higher values for abstract math objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensionality: Option<u8>,

    /// Symmetry class of the geometric shape.
    pub symmetry: Option<SymmetryGroup>,

    /// Volume of the bounding region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_volume: Option<f64>,
}

/// Symmetry classification for geometric rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymmetryGroup {
    /// Invariant under all rotations (point, sphere).
    Spherical,
    /// Invariant around one axis (line, cylinder).
    Cylindrical,
    /// Invariant within a plane (surface, sheet).
    Planar,
    /// Invariant under cube-group rotations (crystal lattice).
    Cubic,
    /// No recognizible symmetry (irregular shape).
    Irregular,
}

//---------------------------------------------------------------------------------------------
// Mechanical Signature
//---------------------------------------------------------------------------------------------

/// Mechanical properties of a VNU.
///
/// The Imagination Engine fills the internal structure from this data.
/// For a `GEOMETRIC_POINT`, the signature is `rest_mass = 0.0`, `density = None`
/// (undefined / Derac delta).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MechanicalSignature {
    /// Rest mass in kg.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest_mass: Option<f64>,

    /// Density in kg/m³.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub density: Option<f64>,

    /// Young's modulus in Pa (resistance to elastic deformation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticity: Option<f64>,
}

//---------------------------------------------------------------------------------------------
// Electromagnetic Signature
//---------------------------------------------------------------------------------------------

/// Electromagnetic properties of a VNU.
///
/// The Imagination Engine applies surface properties from this data.
/// For a `GEOMETRIC_POINT`, the signature is `reflectance = 0.0`
/// (a mathematical point does not interact with light).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ElectromagneticSignature {
    /// Reflectance coefficient in [0.0, 1.0].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reflectance: Option<f64>,

    /// Peak emission wavelengths in nm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emission_spectrum: Option<Vec<f64>>,
}

//---------------------------------------------------------------------------------------------
// Acoustic Signature
//---------------------------------------------------------------------------------------------

/// Acoustic properties of a VNU.
///
/// The Imagination Engine overlays resonance patterns from this data.
/// For a `GEOMETRIC_POINT`, the signature is `resonant_frequency = None`
/// (a point has no acoustic resonance).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct AcousticSignature {
    /// Fundamental resonant frequency in Hz.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resonant_frequency: Option<f64>,

    /// Damping ratio (dimensionless, typically in [0.0, 1.0]).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damping: Option<f64>,
}

//---------------------------------------------------------------------------------------------
// Tests
//---------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// The GEOMETRIC_POINT bootstrap.
    fn geometric_point_signature() -> ObservableSignature {
        ObservableSignature {
            geometric: Some(GeometricSignature {
                dimensionality: Some(0),
                symmetry: Some(SymmetryGroup::Spherical),
                bounding_volume: Some(0.0),
            }),
            mechanical: Some(MechanicalSignature {
                rest_mass: Some(0.0),
                density: None,
                elasticity: None,
            }),
            electromagnetic: Some(ElectromagneticSignature {
                reflectance: Some(0.0),
                emission_spectrum: None,
            }),
            acoustic: Some(AcousticSignature {
                resonant_frequency: None,
                damping: None,
            }),
        }
    }

    #[test]
    fn empty_signature_is_not_grounded() {
        let sig = ObservableSignature::default();
        assert!(!sig.is_grounded());
    }

    #[test]
    fn partial_signature_is_grounded() {
        let sig = ObservableSignature {
            geometric: Some(GeometricSignature {
                dimensionality: Some(0),
                symmetry: None,
                bounding_volume: None,
            }),
            mechanical: None,
            electromagnetic: None,
            acoustic: None,
        };
        assert!(sig.is_grounded());
    }

    #[test]
    fn geometric_point_is_grounded() {
        let sig = geometric_point_signature();
        assert!(sig.is_grounded());
        assert_eq!(sig.geometric.as_ref().unwrap().dimensionality, Some(0));
        assert_eq!(
            sig.geometric.as_ref().unwrap().symmetry,
            Some(SymmetryGroup::Spherical)
        );
        assert_eq!(sig.mechanical.as_ref().unwrap().rest_mass, Some(0.0));
        assert!(sig.mechanical.as_ref().unwrap().density.is_none());
        assert!(sig.acoustic.as_ref().unwrap().resonant_frequency.is_none());
    }

    #[test]
    fn serde_round_trip() {
        let sig = geometric_point_signature;
        let json = serde_json::to_string(&sig).unwrap();
        let back: ObservableSignature = serde_json::from_str(&json).unwrap();
        assert_eq!(back.geometric, sig.geometric);
        assert_eq!(back.mechanical, sig.mechanical);
        assert_eq!(back.electromagnetic, sig.electromagnetic);
        assert_eq!(back.acoustic, sig.acoustic);
    }

    #[test]
    fn serde_forward_compatibility() {
        // A JSON object withmissing fields should deserialize to None.
        let json = r#"{"geometric": {"dimensionality": 3}}"#;
        let sig: ObservableSignature = serde_json::from_str(json).unwrap();
        assert!(sig.geometric.is_some());
        assert_eq!(sig.geometric.as_ref().unwrap().dimensionality, Some(3));
        assert!(sig.geometric.as_ref().unwrap().symmetry.is_none());
        assert!(sig.mechanical.is_none());
        assert!(sig.electromagnetic.is_none());
        assert!(sig.acoustic.is_none());
    }

    #[test]
    fn serde_skips_none_fields() {
        let sig = ObservableSignature {
            geometric: Some(GeometricSignature {
                dimensionality: Some(0),
                symmetry: None,
                bounding_volume: None,
            }),
            mechanical: None,
            electromagnetic: None,
            acoustic: None,
        };
        let json = serde_json::to_string(&sig).unwrap();
        assert!(json.contains("dimensionality"));
        assert!(!json.contains("symmetry"));
        assert!(!json.contains("bounding_volume"));
        assert!(!json.contains("mechanical"));
        assert!(!json.contains("acoustic"));
    }

    #[test]
    fn symmetry_serde_round_trip() {
        let sym = SymmetryGroup::Cylindrical;
        let json = serde_json::to_string(&sym).unwrap();
        assert_eq!(json, "\"cylindrical\"");
        let back: SymmetryGroup = serde_json::from_str(&json).unwrap();
        assert_eq!(back, sym);
    }
}
