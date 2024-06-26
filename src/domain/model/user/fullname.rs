#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FullName {
    pub first_name: String,
    pub last_name: String,
}

impl FullName {
    pub fn new(first_name: &str, last_name: &str) -> Self {
        Self {
            first_name: first_name.trim().to_string(),
            last_name: last_name.trim().to_string(),
        }
    }
    pub fn first_name(&self) -> String {
        self.first_name.clone()
    }
    pub fn last_name(&self) -> String {
        self.last_name.clone()
    }
}
