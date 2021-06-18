#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Altitude(pub u32);

impl Default for Altitude {
    fn default() -> Self {
        Self(0)
    }
}
