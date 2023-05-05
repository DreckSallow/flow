#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub temp_id: u32,
    pub description: String,
    pub status: TaskStatus,
}

impl Task {
    pub fn new(id: u32, desc: &str, temp_id: u32, status: TaskStatus) -> Self {
        Self {
            id,
            description: desc.to_string(),
            temp_id,
            status,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TaskStatus {
    Start,
    Stop,
    Done,
    NoStarted,
}

impl TaskStatus {
    pub fn to_str<'a>(&self) -> &'a str {
        match self {
            TaskStatus::Start => "In progress",
            TaskStatus::Stop => "Stop",
            TaskStatus::Done => "Complete",
            TaskStatus::NoStarted => "Not started",
        }
    }
}

impl ToString for TaskStatus {
    fn to_string(&self) -> String {
        self.to_str().to_string()
    }
}

impl Into<&str> for TaskStatus {
    fn into(self) -> &'static str {
        self.to_str()
    }
}

impl<T: AsRef<str>> From<T> for TaskStatus {
    fn from(s: T) -> Self {
        match s.as_ref() {
            "In progress" => Self::Start,
            "Stop" => Self::Stop,
            "Complete" => Self::Done,
            "Not started" => TaskStatus::NoStarted,
            _ => Self::NoStarted,
        }
    }
}
