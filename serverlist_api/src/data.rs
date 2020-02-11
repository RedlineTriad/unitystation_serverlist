use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    pub name: String,
    pub fork_name: String,
    pub version: i64,
    pub map: String,
    pub game_mode: String,
    pub game_time: String,
    pub player_count: i64,
    pub ip: String,
    pub port: i64,
    pub win_download: String,
    pub osx_download: String,
    pub lin_download: String,
}