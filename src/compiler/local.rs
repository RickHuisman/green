#[derive(Debug, Clone)]
pub struct Local {
    name: String,
    depth: isize,
}

impl Local {
    pub fn new(name: String, depth: isize) -> Self {
        Local { name, depth }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn depth(&self) -> &isize {
        &self.depth
    }

    pub fn depth_mut(&mut self) -> &mut isize {
        &mut self.depth
    }
}
