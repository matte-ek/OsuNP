use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    fs::{self},
    process::Command,
    thread::{self, sleep},
    time::Duration,
};
use tungstenite::Message;

fn main() {
    let token = read_token();
    
    //Command::new("./osu_memory/gosumemory.exe").spawn().unwrap();

    sleep(Duration::from_secs(10));
    println!("[REPORTER] Connecting to websocket...");
    let mut client;
    loop {
        match tungstenite::client::connect("ws://localhost:24050/ws") {
            Ok((c, _)) => {
                client = c;
                break;
            }
            Err(_) => sleep(Duration::from_secs(10)),
        }
    }
    println!("[REPORTER] Websocket connected!");
    thread::spawn(move || {
        let mut last_updated = std::time::Instant::now();

        loop {
            let msg = client.read_message().unwrap();
            if !msg.is_text() || last_updated.elapsed().as_secs() < 5 {
                continue;
            };
            match handle_update_message(msg, &token) {
                Ok(_) => last_updated = std::time::Instant::now(),
                Err(e) => println!("Error: {:?}", e),
            }
        }
    })
    .join()
    .unwrap();
}

pub mod status_message;
use status_message::StatusMessage;

fn handle_update_message(message: Message, token: &str) -> Result<(), HandleError> {
    let message: StatusMessage = serde_json::from_str(message.to_text().unwrap()).unwrap();

    let currently_playing = NowPlaying {
        token: token.to_string(),
        title: message.menu.bm.metadata.title,
        artist: message.menu.bm.metadata.artist,
        mode: message.menu.game_mode,
        current_time: message.menu.bm.time.current,
        full_time: message.menu.bm.time.full,
        difficulty_id: message.menu.bm.id.to_string(),
        beatmap_id: message.menu.bm.set.to_string(),
        difficulty: message.menu.bm.metadata.difficulty,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis(),
    };

    send_update_reqeust(currently_playing)
}

fn send_update_reqeust(np: NowPlaying) -> Result<(), HandleError> {
    reqwest::blocking::Client::new()
        .post("https://osu.bitknox.me/playing/update")
        .json(&PostBody {
            token: np.token.to_string(),
            data: np,
        })
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .map_or(Err(HandleError::new("reqwest")), |_| Ok(()))
}

fn get_i64(value: &Value, key: &str) -> Result<i64, HandleError> {
    value
        .get(key)
        .and_then(|v| v.as_i64())
        .ok_or(HandleError::new(key))
}

fn get_string(val: &Value, key: &str) -> Result<String, HandleError> {
    Ok(val
        .get(key)
        .ok_or(HandleError::new(key))?
        .as_str()
        .ok_or(HandleError::new(key))?
        .to_string())
}

fn read_token() -> String {
    fs::read_to_string("token.txt").unwrap().to_string()
}

#[derive(Debug, Clone)]
enum HandleError {
    Token(String),
}
impl HandleError {
    fn new(token: &str) -> HandleError {
        HandleError::Token(token.to_string())
    }
}

#[derive(Debug, Serialize)]
struct PostBody {
    token: String,
    data: NowPlaying,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NowPlaying {
    pub token: String,
    pub title: String,
    pub mode: i64,
    #[serde(rename(serialize = "currentTime"))]
    pub current_time: i64,
    #[serde(rename(serialize = "fullTime"))]
    pub full_time: i64,
    #[serde(rename(serialize = "beatmapId"))]
    pub beatmap_id: String,
    pub artist: String,
    pub difficulty: String,
    pub difficulty_id: String,
    pub timestamp: u128,
}
