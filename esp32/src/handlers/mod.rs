use crate::{
    client::Client,
    network::Network,
    solver::Solver,
    utils::driver::{ArcDriver, PinAsync},
};
use anyhow::Result;
use esp_idf_svc::hal::{
    gpio::{Pins, PinDriver},
    i2c::{config::Config, I2cDriver, I2C0},
    prelude::Hertz,
};
use std::{sync::{Arc, Mutex}, thread, time::Duration};

pub mod ds18b20;
pub mod max3010x;
pub mod mpu6050;
pub mod ssd1306;

pub use ds18b20::ds18b20;
pub use max3010x::max3010x;
pub use mpu6050::mpu6050;
pub use ssd1306::ssd1306;

macro_rules! i2c_threads {
    ($arr:expr, $driver:expr, $solver:expr) => {
        for i in $arr.iter() {
            let d = $driver.clone();
            let s = $solver.clone();
            thread::spawn(move || {
                while let Err(e) = i(d.clone(), Arc::clone(&s)) {
                    println!("Error: {:?}", e);
                    thread::sleep(Duration::from_secs(5));
                }
            });
        }
    };
}

macro_rules! pin_threads {
    ($arr:expr, $driver:expr, $solver:expr) => {
        for (i, p) in $arr {
            let s = Arc::clone(&$solver);
            let pin = p.clone();
            thread::spawn(move || {
                while let Err(e) = i(pin.pin(), Arc::clone(&s)) {
                    println!("Error: {:?}", e);
                    thread::sleep(Duration::from_secs(5));
                }
            });
        }
    };
}

pub fn init(
    pins: Pins,
    i2c0: I2C0,
    network: Arc<Network>,
    client: Arc<Mutex<Option<Client>>>,
) -> Result<()> {
    let config = Config::new().baudrate(Hertz(400_000));
    let ds18b20_pin = PinAsync(pins.gpio3.into());
    let i2c = I2cDriver::new(i2c0, pins.gpio6, pins.gpio7, &config)?;
    let driver = ArcDriver::new(i2c);
    let solver = Arc::new(Solver::new(client, network)?);

    let vibrator_pin = pins.gpio21;
    let mut vibrator = PinDriver::input_output(vibrator_pin)?;
    vibrator.set_low()?;

    i2c_threads!([max3010x, mpu6050], driver.clone(), solver.clone());
    pin_threads!([(ds18b20, ds18b20_pin)], driver.clone(), solver.clone());

    while let Err(e) = ssd1306(driver.clone()) {
        println!("Error: {:?}", e);
        thread::sleep(Duration::from_secs(5));
    }

    Ok(())
}