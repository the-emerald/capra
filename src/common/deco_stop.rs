pub struct DecoStop {
    depth: i32,
    time: usize
}

impl DecoStop {
    pub fn new(depth: i32, time: usize) -> Self {
        Self {
            depth,
            time
        }
    }
}