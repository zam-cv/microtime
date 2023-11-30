use crate::solver::{Message, Solver};
use anyhow::Result;
use esp_idf_svc::hal::gpio::{AnyIOPin, PinDriver};
use serde::{Deserialize, Serialize};
use std::{sync::Arc, thread, time::Duration};

#[derive(Serialize, Deserialize)]
pub struct Report {
    pub status: String,
    pub description: String,
}

pub fn button(pin: AnyIOPin, solver: Arc<Solver>) -> Result<()> {
    let btn = PinDriver::input(pin)?;
    let mut status = false;

    loop {
        if btn.is_high() {
            status = true;
        }

        if status && btn.is_low() {
            status = false;
            log::info!("Button pressed");

            solver.send_to_database(Message::new(Report {
                status: "warning".to_string(),
                description: "Ocurrio algo grave".to_string(),
            }))?;
            solver.send_to_socket(Message::new(Report {
                status: "warning".to_string(),
                description: "Ocurrio algo grave".to_string(),
            }))?;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
