pub enum Priority {
    Low,
    Normal,
    High,
    None,
}

pub struct Task {
    pub title: String,
    pub priority: Priority,
    pub done: bool,
}

impl Task {
    pub fn from(title: String, priority: Priority) -> Self {
        Task {
            title,
            priority,
            done: false,
        }
    }

    pub fn priority(&self) -> &str {
        match self.priority {
            Priority::Low => "Low",
            Priority::Normal => "Normal",
            Priority::High => "High",
            Priority::None => "None",
        }
    }

    pub fn show(&self) -> String {
        format!("{} - [{}]", self.title, self.priority())
    }
}

impl Priority {
    pub fn from(s: String) -> Self {
        match s.as_str() {
            "h" => Priority::High,
            "n" => Priority::Normal,
            "l" => Priority::Low,
            _ => Priority::None,
        }
    }
}
