#[macro_use]
extern crate log;
use nostr_sdk::prelude::*;
use simplelog::{ColorChoice, CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode};
use std::str::FromStr;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();

    let nwc_uri = std::env::var("NWC_URI").expect("NWC_URI not set");
    let nwc_uri = NostrWalletConnectURI::from_str(&nwc_uri).unwrap();
    info!("\n{nwc_uri}\n");

    let client = Client::default();
    client.add_relay(nwc_uri.relay_url.clone()).await?;
    client.connect().await;
    println!("Connected to relay {}", nwc_uri.relay_url.clone());

    let req = nip47::Request::get_info().clone();
    let req_event = req.to_event(&nwc_uri).unwrap();

    loop {
        match client.send_event(req_event.clone()).await {
            Ok(event_id) => {
                info!("event_id: {}", event_id);
                sleep(Duration::from_secs(30)).await;
            }
            Err(err) => {
                sleep(Duration::from_secs(300)).await;
                error!("error: {}", err);
            }
        }
    }
}
