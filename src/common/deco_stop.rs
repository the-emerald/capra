#[derive(Debug)]
pub struct DecoStop {
    depth: usize,
    time: usize
}

impl DecoStop {
    pub fn new(depth: usize, time: usize) -> Self {
        Self {
            depth,
            time
        }
    }

    pub fn get_depth(&self) -> usize {
        self.depth
    }

    pub fn get_time(&self) -> usize {
        self.time
    }
}