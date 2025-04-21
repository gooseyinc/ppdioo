use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::{self, Write};
use base64::{encode, decode};

// SwitchData Structure
// ODL has this structure on it
#[derive(Debug, Serialize, Deserialize)]
pub struct SwitchData {
    pub backbone: Option<Backbone>,
    pub spine: Option<Spine>,
    pub access: Option<AccessLayer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Backbone {
    pub backbone_switch: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Spine {
    pub spine_switch: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessLayer {
    pub tor: Option<String>,
    pub aor: Option<String>,
}

// fetch data to SDN Controller
pub async fn fetch_sdn_data(url: &str, username: &str, password: &str) -> Result<SwitchData, Box<dyn Error>> {
    let client = Client::new();
    let res = client
        .get(url)
        .basic_auth(username, Some(password))
        .send()
        .await?;

    if res.status().is_success() {
        let data: SwitchData = res.json().await?;
        Ok(data)
    } else {
        Err("Failed to get Netplan".into())
    }
}

// Base64 encoded
pub fn encode_base64(data: &[u8]) -> String {
    encode(data)
}

// Base64 decoded
pub fn decode_base64(encoded: &str) -> Result<Vec<u8>, base64::DecodeError> {
    decode(encoded)
}

// No data input manual
pub fn manual_input() -> SwitchData {
    let mut backbone_switch = String::new();
    let mut spine_switch = String::new();
    let mut tor_switch = String::new();
    let mut aor_switch = String::new();

    print!("Core (Backbone) switch name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut backbone_switch).unwrap();

    print!("Spine switch name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut spine_switch).unwrap();

    print!("TOR switch name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut tor_switch).unwrap();

    print!("AOR switch name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut aor_switch).unwrap();

    SwitchData {
        backbone: Some(Backbone {
            backbone_switch: Some(backbone_switch.trim().to_string()),
        }),
        spine: Some(Spine {
            spine_switch: Some(spine_switch.trim().to_string()),
        }),
        access: Some(AccessLayer {
            tor: Some(tor_switch.trim().to_string()),
            aor: Some(aor_switch.trim().to_string()),
        }),
    }
}

// This function add manual data not CLI
pub async fn get_sdn_data(url: Option<&str>, username: &str, password: &str) -> SwitchData {
    match url {
        Some(u) => {
            println!("get data for ODL API...");
            match fetch_sdn_data(u, username, password).await {
                Ok(data) => {
                    println!("All network topology downloaded.");
                    data
                }
                Err(e) => {
                    println!("Network topology could not be received: {}", e);
                    manual_input()
                }
            }
        }
        None => {
            println!("No SDN Controller URL, please add manual...");
            manual_input()
        }
    }
}
