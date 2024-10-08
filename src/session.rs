use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Session {
    pub depth: usize,
    pub value: Option<String>,
}
