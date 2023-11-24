use crate::{
    drivers::max3010x::{Config, MAX3010x as Sensor},
    solver::{Driver, Headers, Message, Solver, Update},
};
use anyhow::Result;
use embedded_hal::blocking::i2c::{Read, Write};
use log::info;
use serde::{Deserialize, Serialize};
use std::{error::Error, sync::Arc, thread, time::Duration};

#[derive(Serialize, Deserialize)]
pub struct Max3010x {
    pub red: u32,
    pub ir: u32,
}

pub fn max3010x<I2C>(i2c: I2C, solver: Arc<Solver>) -> Result<()>
where
    I2C: Write + Read,
    <I2C as Write>::Error: Error + Send + Sync + Sized + 'static,
    <I2C as Read>::Error: Error + Send + Sync + Sized + 'static,
{
    let mut max3010x = Sensor::new(
        i2c,
        &Config {
            ..Default::default()
        },
    )?;

    let mut red;
    let mut ir;

    loop {
        red = max3010x.get_red();
        ir = max3010x.get_ir();

        if let Ok((red, ir)) = red.and_then(|red| ir.map(|ir| (red, ir))) {
            info!("red: {}, ir: {}", red, ir);

            solver.send(
                Driver::Max3010x,
                Update::Socket(Message {
                    headers: Headers {
                        timestamp: chrono::Local::now().timestamp() as u64,
                    },
                    payload: Max3010x { red, ir },
                }),
            )?;
        } else {
            info!("Error reading sensor");
        }

        thread::sleep(Duration::from_secs(1));
    }
}
