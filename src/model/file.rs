use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct File {
    pub path: String,
    pub parent_id: i64,
    pub file_id: i64,
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.file_id == other.file_id
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
