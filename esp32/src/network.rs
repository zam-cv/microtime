use anyhow::{bail, Result};
use esp_idf_svc::{
    hal::{modem::Modem, peripheral::Peripheral},
    nvs::EspDefaultNvsPartition,
    wifi::{AuthMethod, ClientConfiguration, Configuration},
    {eventloop::EspSystemEventLoop, wifi::EspWifi},
};
use log::info;
use std::{sync::Mutex, thread, time::Duration};

static mut WIFI: Option<Mutex<EspWifi<'static>>> = None;
const MAX_RETRIES: u8 = 7;

pub struct Network {
    pub sysloop: EspSystemEventLoop,
}

impl Network {
    pub fn new(
        modem: impl Peripheral<P = Modem> + 'static,
        sysloop: EspSystemEventLoop,
        nvs: EspDefaultNvsPartition,
    ) -> Result<Self> {
        let esp_wifi = EspWifi::new(modem, sysloop.clone(), Some(nvs))?;

        unsafe {
            if WIFI.is_none() {
                WIFI = Some(Mutex::new(esp_wifi));
            }
        }

        Ok(Self { sysloop })
    }

    pub fn connect(&self, ssid: &str, pass: &str) -> Result<()> {
        let mut auth_method = AuthMethod::WPA2Personal;

        if ssid.is_empty() {
            bail!("Missing WiFi name")
        }

        if pass.is_empty() {
            auth_method = AuthMethod::None;
            info!("Wifi password is empty");
        }

        unsafe {
            if let Some(wifi) = WIFI.take() {
                let mut channel = None;

                {
                    if let Ok(mut wifi) = wifi.lock() {
                        wifi.set_configuration(&Configuration::Client(
                            ClientConfiguration::default(),
                        ))?;
                    }
                }

                {
                    if let Ok(mut wifi) = wifi.lock() {
                        info!("Starting wifi...");
                        wifi.start()?;
                        info!("Scanning...");
                    }
                }

                {
                    if let Ok(mut wifi) = wifi.lock() {
                        let ap_infos = wifi.scan()?;
                        let ours = ap_infos.into_iter().find(|a| a.ssid == ssid);

                        channel = if let Some(ours) = ours {
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
                    }
                }

                {
                    if let Ok(mut wifi) = wifi.lock() {
                        wifi.set_configuration(&Configuration::Client(ClientConfiguration {
                            ssid: ssid.into(),
                            password: pass.into(),
                            channel,
                            auth_method,
                            ..Default::default()
                        }))?;
                    }
                }

                info!("Connecting wifi...");
                let mut count = 0;

                loop {
                    info!("Waiting for wifi connection...");

                    {
                        if let Ok(mut wifi) = wifi.lock() {
                            if wifi.connect().is_err() {
                                std::thread::sleep(std::time::Duration::from_secs(10));
                            }
                        }
                    }

                    if count >= MAX_RETRIES - 1 {
                        esp_idf_sys::esp_restart();
                        // esp_idf_sys::nvs_flash_erase();
                        // esp_idf_sys::nvs_flash_deinit();
                        // count = 0;
                    }

                    count += 1;

                    if let Ok(wifi) = wifi.lock() {
                        if wifi.is_connected()? {
                            break;
                        }
                    }
                }

                {
                    info!("Waiting for DHCP lease...");
                    if let Ok(wifi) = wifi.lock() {
                        let ip_info = wifi.sta_netif().get_ip_info()?;
                        info!("Wifi DHCP info: {:?}", ip_info);
                    }
                }

                WIFI = Some(wifi);
            }
        }

        Ok(())
    }

    pub fn listen<F>(&self, services: F) -> Result<()>
    where
        F: Fn() + Send + 'static,
    {
        thread::spawn(move || loop {
            if let Some(wifi) = unsafe { WIFI.take() } {
                if let Ok(mut wifi) = wifi.lock() {
                    if !wifi.is_connected().unwrap_or(false) {
                        info!("Reconnecting...");
                        let _ = wifi.connect();
                        services();
                    }
                }

                unsafe {
                    WIFI = Some(wifi);
                }
            }

            thread::sleep(Duration::from_secs(5));
        });

        Ok(())
    }

    pub fn is_connected(&self) -> Result<bool> {
        unsafe {
            if let Some(wifi) = WIFI.take() {
                let result;

                if let Ok(wifi) = wifi.lock() {
                    result = Ok(wifi.is_connected()?);
                } else {
                    result = Ok(false);
                }

                WIFI = Some(wifi);
                result
            } else {
                Ok(false)
            }
        }
    }
}
