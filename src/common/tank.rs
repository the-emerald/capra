use crate::common::gas::Gas;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Tank {
    gas: Gas,
    raw_volume: usize, // How much physical space there is in the tank?
    service_pressure: usize
}

impl Tank {
    pub fn new(gas: Gas, raw_volume: usize, service_pressure: usize) -> Self {
        Tank {
            gas,
            raw_volume,
            service_pressure
        }
    }
}