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

#[derive(Debug)]
pub struct Gas {
    fr_n2: f32,
    fr_o2: f32,
    fr_he: f32
}

impl Gas {
    pub fn new(fr_n2: f32, fr_o2: f32, fr_he: f32) -> Result<Self, GasError> {
        if fr_n2 + fr_o2 + fr_he != 1.0 {
            return Err(GasError::FractionError)
        }
        Ok(Self {
            fr_n2,
            fr_o2,
            fr_he
        })
    }

    pub fn fr_n2(&self) -> f32 {
        self.fr_n2
    }

    pub fn fr_o2(&self) -> f32 {
        self.fr_o2
    }

    pub fn fr_he(&self) -> f32 {
        self.fr_he
    }
}

pub fn partial_pressure(depth: usize, fr: f32) -> f32 {
    mtr_bar(depth as f32) * fr
}