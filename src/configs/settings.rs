use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Server {
    pub port: u16,
    pub urls_set: Vec<String>,
}

