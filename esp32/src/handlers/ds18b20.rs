use crate::{
    drivers::ds18b20::Ds18b20 as Sensor,
    solver::{Message, Solver},
};
use anyhow::Result;
use esp_idf_svc::hal::gpio::AnyIOPin;
use serde::{Deserialize, Serialize};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

#[derive(Serialize, Deserialize)]
pub struct Ds18b20 {
    pub temperature: f32,
}

pub fn ds18b20(pin: AnyIOPin, solver: Arc<Solver>) -> Result<()> {
    let ds18b20 = Arc::new(Mutex::new(Sensor::new(pin)?));

    let d = ds18b20.clone();
    let s = solver.clone();
    thread::spawn(move || {
        let ds18b20 = Arc::clone(&d);
        let solver = Arc::clone(&s);

        loop {
            if let Ok(mut ds18b20) = ds18b20.lock() {
                if let Ok(temperature) = ds18b20.get_temp() {
                    log::info!("SOCKET => temperature: {}", temperature);
                    let _ = solver.send_to_socket(Message::new(Ds18b20 { temperature }));
                } else {
                    log::info!("Error reading sensor");
                }
            }

            thread::sleep(Duration::from_secs(2));
        }
    });

    let d = ds18b20.clone();
    let s = solver.clone();
    thread::spawn(move || {
        let ds18b20 = Arc::clone(&d);
        let solver = Arc::clone(&s);

        loop {
            if let Ok(mut ds18b20) = ds18b20.lock() {
                if let Ok(temperature) = ds18b20.get_temp() {
                    log::info!("DATABASE => temperature: {}", temperature);
                    let _ = solver.send_to_database(Message::new(Ds18b20 { temperature }));
                } else {
                    log::info!("Error reading sensor");
                }
            }

            thread::sleep(Duration::from_secs(5));
        }
    });

    Ok(())
}
