pub enum Priority {
    Low,
    Normal,
    High,
    None,
}

pub struct Task {
    pub title: String,
    pub priority: Priority,
}

impl Task {
    pub fn new(title: String, priority: Priority) -> Self {
        Task { title, priority }
    }

    pub fn priority(&self) -> &str {
        match self.priority {
            Priority::Low => "Low",
            Priority::Normal => "Normal",
            Priority::High => "High",
            Priority::None => "-",
        }
    }

    pub fn show(&self) -> String {
        format!("{} - Priority: {}", self.title, self.priority())
    }
}
