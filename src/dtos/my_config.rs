use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MyConfig {
    pub id: u16,
    pub name: String,
    pub rank: u16,
}