use crate::{
    client::{self, Client},
    network::Network,
    utils, PASSWORD, SSID,
};
use anyhow::Result;
use esp_idf_svc::{
    eventloop::{EspEventLoop, System},
    hal::modem::Modem,
    nvs::{EspNvsPartition, NvsDefault},
};
use std::{
    sync::{Arc, Mutex},
    thread,
};

pub fn handle(wifi: Arc<Network>, client_opt: Arc<Mutex<Option<Client>>>) -> Result<()> {
    wifi.connect(SSID, PASSWORD)?;
    utils::sntp::init()?;
    client::create_client(client_opt.clone())?;

    // let client_opt_clone = Arc::clone(&client_opt);
    wifi.listen(move || {
        // let _ = client::create_client(client_opt_clone.clone());
    })
}

pub fn init(
    modem: Modem,
    sysloop: EspEventLoop<System>,
    nvs: EspNvsPartition<NvsDefault>,
) -> Result<(Arc<Network>, Arc<Mutex<Option<Client>>>)> {
    let network = Arc::new(Network::new(modem, sysloop, nvs)?);
    let client = Arc::new(Mutex::new(None));

    let network_clone = Arc::clone(&network);
    let client_clone = Arc::clone(&client);
    thread::spawn(move || {
        if let Err(e) = handle(network_clone, client_clone) {
            println!("Error: {:?}", e);
        }
    });

    Ok((network, client))
}
