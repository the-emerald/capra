use crate::common::mtr_bar;

#[derive(thiserror::Error, Debug)]
pub enum GasError {
    #[error("gas does not have total fraction of 1.0")]
    FractionError
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "use-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Gas {
    o2: usize,
    he: usize,
    n2: usize
}

#[macro_export]
macro_rules! gas {
    ($o2:expr, $he:expr) => {
        {
            Gas::new($o2, $he, 100 - $o2 - $he).unwrap()
        }
    };
}

impl Gas {
    pub fn new(o2: usize, he: usize, n2: usize) -> Result<Self, GasError> {
        if o2 + he + n2 != 100 {
        return Err(GasError::FractionError)
        }

        Ok(Self {
            o2,
            he,
            n2
        })
    }

    pub fn fr_n2(&self) -> f64 {
        self.n2 as f64 / 100.0
    }

    pub fn fr_o2(&self) -> f64 {
        self.o2 as f64 / 100.0
    }

    pub fn fr_he(&self) -> f64 {
        self.he as f64 / 100.0
    }

    pub fn o2(&self) -> usize {
        self.o2
    }

    pub fn he(&self) -> usize {
        self.he
    }

    pub fn n2(&self) -> usize {
        self.n2
    }

    pub fn equivalent_narcotic_depth(&self, depth: usize) -> usize {
        (((depth + 10) as f64 * (1.0 - self.fr_he())) - 10.0) as usize
    }

    pub fn pp_o2(&self, depth: usize, metre_per_bar: f64) -> f64 {
        mtr_bar(depth as f64, metre_per_bar) * self.fr_o2()
    }

    pub fn pp_he(&self, depth: usize, metre_per_bar: f64) -> f64 {
        mtr_bar(depth as f64, metre_per_bar) * self.fr_he()
    }

    pub fn pp_n2(&self, depth: usize, metre_per_bar: f64) -> f64 {
        mtr_bar(depth as f64, metre_per_bar) * self.fr_n2()
    }
}

// fn valid_pp(pp: f64) -> bool {
//     pp >= 0.0 && pp <= 1.0
// }