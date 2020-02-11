#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub servers: Vec<Server>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    #[serde(rename = "ServerName")]
    pub server_name: String,
    #[serde(rename = "ForkName")]
    pub fork_name: String,
    #[serde(rename = "BuildVersion")]
    pub build_version: i64,
    #[serde(rename = "CurrentMap")]
    pub current_map: String,
    #[serde(rename = "GameMode")]
    pub game_mode: String,
    #[serde(rename = "IngameTime")]
    pub ingame_time: String,
    #[serde(rename = "PlayerCount")]
    pub player_count: i64,
    #[serde(rename = "ServerIP")]
    pub server_ip: String,
    #[serde(rename = "ServerPort")]
    pub server_port: i64,
    #[serde(rename = "WinDownload")]
    pub win_download: String,
    #[serde(rename = "OSXDownload")]
    pub osxdownload: String,
    #[serde(rename = "LinuxDownload")]
    pub linux_download: String,
}
