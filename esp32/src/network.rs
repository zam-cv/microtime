use anyhow::{bail, Result};
use esp_idf_svc::{
    hal::{modem::Modem, peripheral::Peripheral},
    nvs::EspDefaultNvsPartition,
    wifi::{AuthMethod, ClientConfiguration, Configuration},
    {eventloop::EspSystemEventLoop, wifi::BlockingWifi, wifi::EspWifi},
};
use log::info;

const MAX_RETRIES: u8 = 7;

pub fn connect(
    ssid: &str,
    pass: &str,
    modem: impl Peripheral<P = Modem> + 'static,
    sysloop: EspSystemEventLoop,
    nvs: EspDefaultNvsPartition,
) -> Result<Box<EspWifi<'static>>> {
    let mut auth_method = AuthMethod::WPA2Personal;

    if ssid.is_empty() {
        bail!("Missing WiFi name")
    }

    if pass.is_empty() {
        auth_method = AuthMethod::None;
        info!("Wifi password is empty");
    }

    let mut esp_wifi = EspWifi::new(modem, sysloop.clone(), Some(nvs))?;
    let mut wifi = BlockingWifi::wrap(&mut esp_wifi, sysloop)?;

    wifi.set_configuration(&Configuration::Client(ClientConfiguration::default()))?;

    info!("Starting wifi...");
    wifi.start()?;
    info!("Scanning...");

    let ap_infos = wifi.scan()?;
    let ours = ap_infos.into_iter().find(|a| a.ssid == ssid);

    let channel = if let Some(ours) = ours {
        info!(
            "Found configured access point {} on channel {}",
            ssid, ours.channel
        );

        Some(ours.channel)
    } else {
        info!(
            "Configured access point {} not found during scanning, will go with unknown channel",
            ssid
        );

        None
    };

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: ssid.into(),
        password: pass.into(),
        channel,
        auth_method,
        ..Default::default()
    }))?;

    info!("Connecting wifi...");

    let mut count = 0;

    loop {
        info!("Waiting for wifi connection...");

        if wifi.connect().is_err() {
            std::thread::sleep(std::time::Duration::from_secs(10));
        }

        if count > MAX_RETRIES - 1 {
            unsafe {
                esp_idf_sys::esp_restart();
            }
        }

        count += 1;

        if wifi.is_connected()? {
            break;
        }
    }

    info!("Waiting for DHCP lease...");
    wifi.wait_netif_up()?;

    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;
    info!("Wifi DHCP info: {:?}", ip_info);

    Ok(Box::new(esp_wifi))
}
