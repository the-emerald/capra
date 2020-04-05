use crate::common::mtr_bar;

#[derive(Debug)]
pub enum GasError {
    FractionError
}

impl std::error::Error for GasError {}

impl std::fmt::Display for GasError {
    fn fmt(&self, f: &mut std::fmt::Formatter)
           -> std::fmt::Result {
        match self {
            GasError::FractionError => write!(f, "Gas does not have total gas fraction of 1.0"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Gas {
    fr_n2: f64,
    fr_o2: f64,
    fr_he: f64
}

impl Gas {
    pub fn new(fr_n2: f64, fr_o2: f64, fr_he: f64) -> Result<Self, GasError> {
        if (fr_n2 + fr_o2 + fr_he - 1.0).abs() > 0.005 || !valid_pp(fr_n2) || !valid_pp(fr_o2) ||
            !valid_pp(fr_he){
            return Err(GasError::FractionError)
        }
        Ok(Self {
            fr_n2,
            fr_o2,
            fr_he
        })
    }

    pub fn fr_n2(&self) -> f64 {
        self.fr_n2
    }

    pub fn fr_o2(&self) -> f64 {
        self.fr_o2
    }

    pub fn fr_he(&self) -> f64 {
        self.fr_he
    }
}

pub fn partial_pressure(depth: usize, fr: f64) -> f64 {
    mtr_bar(depth as f64) * fr
}

fn valid_pp(pp: f64) -> bool {
    pp >= 0.0 && pp <= 1.0
}