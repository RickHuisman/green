#[derive(Debug, Clone)]
pub struct Local {
    name: String, // TODO &str??
    depth: i32,
}

impl Local {
    pub fn new(name: String, depth: i32) -> Self {
        Local { name, depth }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn depth(&self) -> &i32 {
        &self.depth
    }

    pub fn depth_mut(&mut self) -> &mut i32 {
        &mut self.depth
    }
}