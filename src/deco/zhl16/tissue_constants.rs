use crate::deco::zhl16::variant::Variant;
use crate::deco::TISSUE_COUNT;

/// N2 half-lives for the ZHL-16B deco algorithm.
pub const ZHL16B_N2_HALFLIFE: [f64; 16] = [
    5.0, 8.0, 12.5, 18.5, 27.0, 38.3, 54.3, 77.0, 109.0, 146.0, 187.0, 239.0, 305.0, 390.0, 498.0,
    635.0,
];

/// N2 A-values for the ZHL-16B deco algorithm.
pub const ZHL16B_N2_A: [f64; 16] = [
    1.1696, 1.0000, 0.8618, 0.7562, 0.6667, 0.5600, 0.4947, 0.4500, 0.4187, 0.3798, 0.3497, 0.3223,
    0.2850, 0.2737, 0.2523, 0.2327,
];

/// N2 B-values for the ZHL-16B deco algorithm.
pub const ZHL16B_N2_B: [f64; 16] = [
    0.5578, 0.6514, 0.7222, 0.7825, 0.8126, 0.8434, 0.8693, 0.8910, 0.9092, 0.9222, 0.9319, 0.9403,
    0.9477, 0.9544, 0.9602, 0.9653,
];

/// Helium half-lives for the ZHL-16B deco algorithm.
pub const ZHL16B_HE_HALFLIFE: [f64; 16] = [
    1.88, 3.02, 4.72, 6.99, 10.21, 14.48, 20.53, 29.11, 41.20, 55.19, 70.69, 90.34, 115.29, 147.42,
    188.24, 240.03,
];

/// Helium A-values for the ZHL-16B deco algorithm.
pub const ZHL16B_HE_A: [f64; 16] = [
    1.6189, 1.3830, 1.1919, 1.0458, 0.9220, 0.8205, 0.7305, 0.6502, 0.5950, 0.5545, 0.5333, 0.5189,
    0.5181, 0.5176, 0.5172, 0.5119,
];

/// Helium B-values for the ZHL-16B deco algorithm.
pub const ZHL16B_HE_B: [f64; 16] = [
    0.4770, 0.5747, 0.6527, 0.7223, 0.7582, 0.7957, 0.8279, 0.8553, 0.8757, 0.8903, 0.8997, 0.9073,
    0.9122, 0.9171, 0.9217, 0.9267,
];

/// N2 half-lives for the ZHL-16C deco algorithm.
pub const ZHL16C_N2_HALFLIFE: [f64; 16] = [
    4.0, 8.0, 12.5, 18.5, 27.0, 38.3, 54.3, 77.0, 109.0, 146.0, 187.0, 239.0, 305.0, 390.0, 498.0,
    635.0,
];

/// N2 A-values for the ZHL-16C deco algorithm.
pub const ZHL16C_N2_A: [f64; 16] = [
    1.2599, 1.0000, 0.8618, 0.7562, 0.6200, 0.5043, 0.4410, 0.4000, 0.3750, 0.3500, 0.3295, 0.3065,
    0.2835, 0.2610, 0.2480, 0.2327,
];

/// N2 B-values for the ZHL-16C deco algorithm.
pub const ZHL16C_N2_B: [f64; 16] = [
    0.5050, 0.6514, 0.7222, 0.7825, 0.8126, 0.8434, 0.8693, 0.8910, 0.9092, 0.9222, 0.9319, 0.9403,
    0.9477, 0.9544, 0.9602, 0.9653,
];

/// Helium half-lives for the ZHL-16C deco algorithm.
pub const ZHL16C_HE_HALFLIFE: [f64; 16] = [
    1.51, 3.02, 4.72, 6.99, 10.21, 14.48, 20.53, 29.11, 41.20, 55.19, 70.69, 90.34, 115.29, 147.42,
    188.24, 240.03,
];

/// Helium A-values for the ZHL-16C deco algorithm.
pub const ZHL16C_HE_A: [f64; 16] = [
    1.7424, 1.3830, 1.1919, 1.0458, 0.9220, 0.8205, 0.7305, 0.6502, 0.5950, 0.5545, 0.5333, 0.5189,
    0.5181, 0.5176, 0.5172, 0.5119,
];

/// Helium B-values for the ZHL-16C deco algorithm.
pub const ZHL16C_HE_B: [f64; 16] = [
    0.4245, 0.5747, 0.6527, 0.7223, 0.7582, 0.7957, 0.8279, 0.8553, 0.8757, 0.8903, 0.8997, 0.9073,
    0.9122, 0.9171, 0.9217, 0.9267,
];

#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TissueConstants {
    n2_a: [f64; TISSUE_COUNT],
    n2_b: [f64; TISSUE_COUNT],
    n2_hl: [f64; TISSUE_COUNT],
    he_a: [f64; TISSUE_COUNT],
    he_b: [f64; TISSUE_COUNT],
    he_hl: [f64; TISSUE_COUNT],
}

impl TissueConstants {
    pub fn new(
        n2_a: [f64; TISSUE_COUNT],
        n2_b: [f64; TISSUE_COUNT],
        n2_hl: [f64; TISSUE_COUNT],
        he_a: [f64; TISSUE_COUNT],
        he_b: [f64; TISSUE_COUNT],
        he_hl: [f64; TISSUE_COUNT],
    ) -> Self {
        Self {
            n2_a,
            n2_b,
            n2_hl,
            he_a,
            he_b,
            he_hl,
        }
    }

    pub fn n2_a(&self) -> [f64; 16] {
        self.n2_a
    }

    pub fn n2_b(&self) -> [f64; 16] {
        self.n2_b
    }

    pub fn n2_hl(&self) -> [f64; 16] {
        self.n2_hl
    }

    pub fn he_a(&self) -> [f64; 16] {
        self.he_a
    }

    pub fn he_b(&self) -> [f64; 16] {
        self.he_b
    }

    pub fn he_hl(&self) -> [f64; 16] {
        self.he_hl
    }

    pub fn new_by_variant(variant: Variant) -> Self {
        match variant {
            Variant::B => Self::new(
                ZHL16B_N2_A,
                ZHL16B_N2_B,
                ZHL16B_N2_HALFLIFE,
                ZHL16B_HE_A,
                ZHL16B_HE_B,
                ZHL16B_HE_HALFLIFE,
            ),
            Variant::C => Self::new(
                ZHL16C_N2_A,
                ZHL16C_N2_B,
                ZHL16C_N2_HALFLIFE,
                ZHL16C_HE_A,
                ZHL16C_HE_B,
                ZHL16C_HE_HALFLIFE,
            ),
        }
    }
}
