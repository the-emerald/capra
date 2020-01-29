#[derive(Debug)]
pub struct Gas {
    fr_n2: f32,
    fr_o2: f32,
    fr_he: f32
}

impl Gas {
    pub fn new(fr_n2: f32, fr_o2: f32, fr_he: f32) -> Self {
        Self {
            fr_n2,
            fr_o2,
            fr_he
        }
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