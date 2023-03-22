#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub description: String,
}

impl Task {
    pub fn new(id: u32, desc: &str) -> Self {
        Self {
            id,
            description: desc.to_string(),
        }
    }
}
