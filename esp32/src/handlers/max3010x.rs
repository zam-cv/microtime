use crate::{
    drivers::max3010x::{Config, Max3010x as Sensor},
    solver::{Message, Solver}
};
use anyhow::{anyhow, Result};
use embedded_hal::blocking::i2c::{Read, Write};
use log::info;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

#[derive(Serialize, Deserialize)]
pub struct Max3010x {
    pub red: u32,
    pub ir: u32,
}

pub fn max3010x<I2C>(i2c: I2C, solver: Arc<Solver>) -> Result<()>
where
    I2C: Write + Read + Send + Sync + Clone + 'static,
    <I2C as Write>::Error: Error + Send + Sync + Sized + 'static,
    <I2C as Read>::Error: Error + Send + Sync + Sized + 'static,
{
    let max3010x = Arc::new(Mutex::new(Sensor::new(
        i2c.clone(),
        &Config {
            ..Default::default()
        },
    )?));

    let m = max3010x.clone();
    let s = solver.clone();
    thread::spawn(move || {
        let solver = Arc::clone(&s);
        let mut red;
        let mut ir;

        loop {
            if let Ok(mut max3010x) = m.lock() {
                red = max3010x.get_red();
                ir = max3010x.get_ir();

                if let Ok((red, ir)) = red.and_then(|red| ir.map(|ir| (red, ir))) {
                    info!("SOCKET => red: {}, ir: {}", red, ir);
                    let _ = solver.send_to_socket(Message::new(Max3010x { red, ir }));
                } else {
                    info!("Error reading sensor");
                }
            }

            thread::sleep(Duration::from_millis(500));
        }
    });

    let m = max3010x.clone();
    let s = solver.clone();
    let handle2 = thread::spawn(move || {
        let solver = Arc::clone(&s);
        let mut red;
        let mut ir;

        loop {
            if let Ok(mut max3010x) = m.lock() {
                red = max3010x.get_red();
                ir = max3010x.get_ir();

                if let Ok((red, ir)) = red.and_then(|red| ir.map(|ir| (red, ir))) {
                    info!("DATABASE => red: {}, ir: {}", red, ir);
                    let _ = solver.send_to_database(Message::new(Max3010x { red, ir }));
                } else {
                    info!("Error reading sensor");
                }
            }

            thread::sleep(Duration::from_secs(2));
        }
    });

    Ok(())
}