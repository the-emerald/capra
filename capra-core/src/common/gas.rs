use crate::common::mtr_bar;

/// Represents errors that occur while working with Gases.
#[cfg_attr(feature = "std", derive(thiserror::Error))]
#[derive(Debug)]
pub enum GasError {
    #[cfg_attr(feature = "std", error("gas does not have total fraction of 1.0"))]
    /// The sum of all percentage gas fractions does not add up to 100.
    FractionError,
}

/// A gas mix used in a dive.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Gas {
    /// Percentage fraction of oxygen in the mix.
    o2: usize,
    /// Percentage fraction of helium in the mix.
    he: usize,
    /// Percentage fraction of nitrogen in the mix.
    n2: usize,
}

/// Shorthand for creating a Gas, in a style similar to mix notation (O2/He)
/// # Panics
/// This macro will panic if the two supplied values exceed 100.
#[macro_export]
macro_rules! gas {
    ($o2:expr, $he:expr) => {{
        Gas::new($o2, $he, 100 - $o2 - $he).unwrap()
    }};
}

impl Gas {
    /// Returns a new Gas with the given parameters.
    /// # Arguments
    /// * `o2` - Percentage fraction of oxygen in the mix.
    /// * `he` - Percentage fraction of helium in the mix.
    /// * `n2` - Percentage fraction of nitrogen in the mix.
    /// # Errors
    /// This function will return a [`GasError`] if the percentage fractions do not add up to 100.
    pub fn new(o2: usize, he: usize, n2: usize) -> Result<Self, GasError> {
        if o2 + he + n2 != 100 {
            return Err(GasError::FractionError);
        }

        Ok(Self { o2, he, n2 })
    }

    /// Returns the **fraction** of nitrogen in the mix.
    pub fn fr_n2(&self) -> f64 {
        self.n2 as f64 / 100.0
    }

    /// Returns the **fraction** of oxygen in the mix.
    pub fn fr_o2(&self) -> f64 {
        self.o2 as f64 / 100.0
    }

    /// Returns the **fraction** of helium in the mix.
    pub fn fr_he(&self) -> f64 {
        self.he as f64 / 100.0
    }
    /// Returns the percentage fraction of oxygen in the mix.
    pub fn o2(&self) -> usize {
        self.o2
    }

    /// Returns the percentage fraction of helium in the mix.
    pub fn he(&self) -> usize {
        self.he
    }

    /// Returns the percentage fraction of nitrogen in the mix.
    pub fn n2(&self) -> usize {
        self.n2
    }

    /// Returns the Equivalent Narcotic Depth (END) of the mix at a given depth.
    /// # Arguments
    /// * `depth` - The depth the mix is being breathed at.
    pub fn equivalent_narcotic_depth(&self, depth: usize) -> usize {
        (((depth + 10) as f64 * (1.0 - self.fr_he())) - 10.0) as usize
    }

    /// Helper function to check whether the mix is in an acceptable ppO2 range at a given depth.
    /// # Arguments
    /// * `depth` -Depth the mix is being breathed at.
    /// * `min` - Minimum tolerable ppO2.
    /// * `max` - Maximum tolerable ppO2.
    pub fn in_ppo2_range(&self, depth: usize, min: f64, max: f64) -> bool {
        let ppo2 = self.pp_o2(depth, 10.0);
        ppo2 >= min && ppo2 <= max
    }

    /// Returns the ppO2 of the mix at a given depth.
    /// # Arguments
    /// * `depth` - Depth the mix is being breathed at.
    /// * `metres_per_bar` - Depth of water required to induce 1 bar of pressure.
    pub fn pp_o2(&self, depth: usize, metres_per_bar: f64) -> f64 {
        mtr_bar(depth as f64, metres_per_bar) * self.fr_o2()
    }

    /// Returns the ppHe of the mix at a given depth.
    /// # Arguments
    /// * `depth` - Depth the mix is being breathed at.
    /// * `metres_per_bar` - Depth of water required to induce 1 bar of pressure.
    pub fn pp_he(&self, depth: usize, metres_per_bar: f64) -> f64 {
        mtr_bar(depth as f64, metres_per_bar) * self.fr_he()
    }

    /// Returns the ppN2 of the mix at a given depth.
    /// # Arguments
    /// * `depth` - Depth the mix is being breathed at.
    /// * `metres_per_bar` - Depth of water required to induce 1 bar of pressure.
    pub fn pp_n2(&self, depth: usize, metre_per_bar: f64) -> f64 {
        mtr_bar(depth as f64, metre_per_bar) * self.fr_n2()
    }
}
