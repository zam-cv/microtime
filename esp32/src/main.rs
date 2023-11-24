#![allow(unused_variables, unused_imports, dead_code)]
#![feature(c_variadic, ascii_char)]
use anyhow::Result;
use esp_idf_svc::{eventloop::EspSystemEventLoop, hal::peripherals::Peripherals, nvs};
use esp_idf_sys as _;

mod client;
mod drivers;
mod handlers;
mod network;
mod solver;
mod utils;

pub const CLIENT_ID: &str = "EQUIPO-2";
pub const HOST: &str = "192.168.167.102";
pub const PORT: &str = "1883";

pub const HOSTPOT_SSID: &str = "MicroTime";
pub const HOSTPOT_PASSWORD: &str = "qwertyui";

pub const SSID: &str = "Red";
pub const PASSWORD: &str = "12345678";

fn app() -> Result<()> {
    let peripherals = Peripherals::take()?;
    let sysloop = EspSystemEventLoop::take()?;
    let nvs = nvs::EspDefaultNvsPartition::take()?;

    #[allow(unused_mut, unused)]
    let mut wifi = network::connect(SSID, PASSWORD, peripherals.modem, sysloop, nvs)?;
    utils::sntp::init()?;

    handlers::init(peripherals.pins, peripherals.i2c0)
}

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    if let Err(e) = app() {
        println!("Error: {:?}", e);
    }
}
