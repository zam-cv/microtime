use crate::{
    drivers::ds18b20::Ds18b20 as Sensor,
    solver::{Message, Solver},
};
use anyhow::Result;
use esp_idf_svc::hal::gpio::AnyIOPin;
use serde::{Deserialize, Serialize};
use std::{sync::Arc, thread, time::Duration};

#[derive(Serialize, Deserialize)]
pub struct Ds18b20 {
    pub temperature: f32,
}

pub fn ds18b20(pin: AnyIOPin, solver: Arc<Solver>) -> Result<()> {
    let mut ds18b20 = Sensor::new(pin)?;
    let mut temperature;

    loop {
        thread::sleep(Duration::from_secs(2));
        temperature = ds18b20.get_temp()?;
        log::info!("temperature: {}", temperature);

        solver.send_to_database(Message::new(Ds18b20 { temperature }))?;
    }
}
