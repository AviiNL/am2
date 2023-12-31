use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub score: i32,
    pub duration: u64,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Info {
    pub name: String,
    pub map: String,
    pub mission: String,
    pub max_players: u8,
    pub players: Vec<Player>,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub enum ArmaStatus {
    Online(Info),
    #[default]
    Offline,
}
