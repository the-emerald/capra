#[derive(Debug, Copy, Clone)]
pub enum StopType {
    NoDeco, Stop
}

#[derive(Debug)]
pub struct DecoStop {
    deco_type: StopType,
    depth: usize,
    time: usize
}

impl DecoStop {
    pub fn new(deco_type: StopType, depth: usize, time: usize) -> Self {
        Self {
            deco_type,
            depth,
            time
        }
    }

    pub fn get_deco_type(&self) -> StopType {
        self.deco_type
    }

    pub fn get_depth(&self) -> usize {
        self.depth
    }

    pub fn get_time(&self) -> usize {
        self.time
    }
}