// usage.rs

use ppdioo::{fetch_sdn_data, SwitchData};
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    // Temporary URL added
    let default_url = "http://localhost:8181/restconf/operational/network-topology:network-topology".to_string();
    let url = args.get(2).unwrap_or(&default_url);
    
    let binding = "admin".to_string();  // Temporary credentials
    let username = args.get(3).unwrap_or(&binding);
    let password = args.get(4).unwrap_or(&binding);

    println!("get data to ODL...");

    let switch_data = fetch_sdn_data(url, username, password).await.unwrap_or_else(|err| {
        eprintln!("Hata: {}", err);
        SwitchData {
            backbone: None,
            spine: None,
            access: None,
        }
    });

    if let Some(backbone) = switch_data.backbone {
        println!("Backbone switch: {}", backbone.backbone_switch.unwrap());
    }
    if let Some(spine) = switch_data.spine {
        println!("Spine switch: {}", spine.spine_switch.unwrap());
    }
    if let Some(access) = switch_data.access {
        println!("TOR: {}", access.tor.unwrap());
        println!("AOR: {}", access.aor.unwrap());
    }
}
