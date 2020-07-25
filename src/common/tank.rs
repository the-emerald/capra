use crate::common::gas::Gas;

/// A diving cylinder filled with some gas mix with some volume and service pressure.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Tank {
    /// Gas mix currently inside the tank.
    gas: Gas,
    /// Physical volume inside the tank.
    raw_volume: usize,
    /// Manufacturer specified service pressure of the tank.
    service_pressure: usize
}

impl Tank {
    /// Return a new tank with the given parameters
    /// # Arguments
    /// * `gas` - Gas mix currently inside the tank.
    /// * `raw_volume` - Physical volume inside the tank.
    /// * `service_pressure` - Manufacturer specified service pressure of the tank.
    pub fn new(gas: Gas, raw_volume: usize, service_pressure: usize) -> Self {
        Tank {
            gas,
            raw_volume,
            service_pressure
        }
    }
}