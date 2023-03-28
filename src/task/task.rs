#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub temp_id: u32,
    pub description: String,
}

impl Task {
    pub fn new(id: u32, desc: &str, temp_id: u32) -> Self {
        Self {
            id,
            description: desc.to_string(),
            temp_id,
        }
    }
}
