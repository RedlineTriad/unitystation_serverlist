use std::thread;
use std::time;
mod data;
mod old_data;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    loop {
        println!("Updating Serverlist");
        let resp = reqwest::get("https://api.unitystation.org/serverlist")
            .await?
            .json::<old_data::Root>()
            .await?
            .servers;

        let servers: Vec<data::Server> = resp
            .into_iter()
            .map(|server| data::Server {
                name: server.server_name,
                fork_name: server.fork_name,
                version: server.build_version,
                map: server.current_map,
                game_mode: server.game_mode,
                game_time: server.ingame_time,
                player_count: server.player_count,
                ip: server.server_ip,
                port: server.server_port,
                win_download: server.win_download,
                osx_download: server.osxdownload,
                lin_download: server.linux_download,
            })
            .collect();
        
        for server in servers {
            client.post("http://api:8080").json(&server).send().await?;
        }

        thread::sleep(time::Duration::from_secs(5));
    }
}
