//! The Cursor resource is a globally accessible resource for mouse cursor state.

#[derive(Copy, Clone, Debug)]
pub struct Resource {
    pub pos: (f64, f64),
    pub delta: (f64, f64)
}

impl Resource {
    pub fn new() -> Resource {
        Resource {
            pos: (0.0, 0.0),
            delta: (0.0, 0.0)
        }
    }

    pub fn set(&mut self, pos: (f64, f64)) {
        self.delta = (pos.0 - self.pos.0, pos.1 - self.pos.1);
        self.pos = pos;
    }
}