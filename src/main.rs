use nats;
use ppdioo::get_sdn_data;
use serde_json::json;
use std::env;

async fn send_data_to_nats(url: &str, subject: &str, data: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
    // connect to NATS.IO use nats lib
    let nc = nats::connect(url)?;

    // send netplan to NATS.IO with JSON format
    let data_str = data.to_string();
    nc.publish(subject, &data_str.as_bytes())?;

    println!("Netplan send to NATS.IO: {}", data_str);
    Ok(())
}

#[tokio::main]
async fn main() {
    // SDN Controller RESTCONF connector with use --url parameters
    // Example: --url http://localhost:8181/restconf/operational/network-topology:network-topology
    let args: Vec<String> = env::args().collect();
    let url = if args.len() > 1 { Some(&args[1]) } else { None };
    let username = "admin"; // default username
    let password = "admin"; // default password

    // get data
    let switch_data = get_sdn_data(url.map(|x| x.as_str()), username, password).await;

    // show console
    println!("get Topology: {:?}", switch_data);

    // wrap to JSON
    let data_to_send = json!({
        "backbone": switch_data.backbone,
        "spine": switch_data.spine,
        "access": switch_data.access
    });

    // send to NATS.IO
    let nats_url = "nats://localhost:4222";  // NATS API url
    let subject = "network.topology";  // Topic

    if let Err(e) = send_data_to_nats(nats_url, subject, data_to_send).await {
        eprintln!("Topology could not be sent: {}", e);
    }
}
