/// Represents the variants of ZHL16 defined in the library.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Variant {
    /// ZHL-16B
    B,
    /// ZHL-16C
    C
}