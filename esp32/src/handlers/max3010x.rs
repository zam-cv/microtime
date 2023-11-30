use crate::{
    drivers::max3010x::{Config, Max3010x as Sensor},
    solver::{Message, Solver},
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
    time::Instant,
};

const THRESHOLD: u32 = 200;

struct HeartRateMonitor {
    last_beat_time: Option<Instant>,
    bpm: f32,
    peak_threshold: u32,
    last_ir_value: u32,
}

impl HeartRateMonitor {
    fn new(peak_threshold: u32) -> HeartRateMonitor {
        HeartRateMonitor {
            last_beat_time: None,
            bpm: 0.0,
            peak_threshold,
            last_ir_value: 0,
        }
    }

    fn update(&mut self, ir: u32) {
        // Aquí iría la lógica para filtrar la señal 'ir', que es esencial
        let filtered_ir = self.filter_ir_signal(ir);

        // Detección de picos (un ejemplo simple)
        if filtered_ir > self.peak_threshold && self.last_ir_value <= self.peak_threshold {
            if let Some(last_time) = self.last_beat_time {
                let now = Instant::now();
                let duration = now.duration_since(last_time);
                self.last_beat_time = Some(now);

                // Convertir la duración a BPM (60 segundos divididos por el intervalo en segundos)
                let interval = duration.as_secs_f32();
                if interval > 0.0 {
                    self.bpm = 60.0 / interval;
                }
            } else {
                // Primera vez que se detecta un latido
                self.last_beat_time = Some(Instant::now());
            }
        }

        self.last_ir_value = filtered_ir;
    }

    fn get_bpm(&self) -> f32 {
        self.bpm
    }

    fn filter_ir_signal(&self, ir: u32) -> u32 {
        // Aquí deberías implementar un filtro apropiado, como un filtro de paso bajo
        // Por el momento, esto es solo un placeholder
        ir
    }
}

#[derive(Serialize, Deserialize)]
pub struct Max3010x {
    // pub red: u32,
    // pub ir: u32,
    pub heart_rate: u32
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
        let mut monitor = HeartRateMonitor::new(50000);

        loop {
            if let Ok(mut max3010x) = m.lock() {
                red = max3010x.get_red();
                ir = max3010x.get_ir();

                if let Ok((red, ir)) = red.and_then(|red| ir.map(|ir| (red, ir))) {
                    monitor.update(ir);

                    // Obtener y usar el valor BPM actual
                    let bpm = monitor.get_bpm();
                    println!("BPM: {}", bpm);

                    info!("SOCKET => red: {}, ir: {}", red, ir);
                    let _ = solver.send_to_socket(Message::new(Max3010x { 
                        heart_rate: 0
                     }));
                } else {
                    info!("Error reading sensor");
                }
            }

            thread::sleep(Duration::from_millis(100));
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
                    let _ = solver.send_to_database(Message::new(Max3010x { 
                        heart_rate: 0
                     }));
                } else {
                    info!("Error reading sensor");
                }
            }

            thread::sleep(Duration::from_secs(2));
        }
    });

    Ok(())
}
