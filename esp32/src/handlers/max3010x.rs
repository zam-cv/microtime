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
    collections::VecDeque
};

const THRESHOLD: u32 = 200;

struct HeartRateMonitor {
    last_beat_time: Option<Instant>,
    bpm: f32,
    readings: VecDeque<u32>,
    total_ir: u32,
    last_ir_value: u32,
    finger_detected: bool,
}

impl HeartRateMonitor {
    fn new() -> HeartRateMonitor {
        HeartRateMonitor {
            last_beat_time: None,
            bpm: 0.0,
            readings: VecDeque::with_capacity(WINDOW_SIZE),
            total_ir: 0,
            last_ir_value: 0,
            finger_detected: false,
        }
    }

    fn update(&mut self, ir: u32, red: u32) {
        if self.readings.len() == WINDOW_SIZE {
            self.total_ir -= self.readings.pop_front().unwrap();
        }
        self.readings.push_back(ir);
        self.total_ir += ir;

        let average_ir = self.total_ir as f32 / self.readings.len() as f32;

        // Actualización de la detección de dedo
        self.finger_detected = self.is_finger_detected(ir, red, average_ir);

        if !self.finger_detected {
            self.bpm = 0.0;
            return;
        }

        let filtered_ir = self.filter_ir_signal(ir);

        if self.is_peak(filtered_ir, self.last_ir_value) && self.last_beat_time.map_or(true, |last_time| Instant::now().duration_since(last_time).as_secs_f32() >= MIN_INTERVAL) {
            let now = Instant::now();
            self.bpm = self.last_beat_time.map_or(0.0, |last_time| 60.0 / now.duration_since(last_time).as_secs_f32());
            self.last_beat_time = Some(now);
        }

        self.last_ir_value = filtered_ir;
    }

    fn get_bpm(&self) -> f32 {
        self.bpm
    }

    fn filter_ir_signal(&self, ir: u32) -> u32 {
        // Implementar un filtro de señal aquí, como un filtro de paso bajo o de media móvil
        ir // Placeholder
    }

    fn is_peak(&self, current: u32, previous: u32) -> bool {
        current > previous && current > PEAK_THRESHOLD
    }

    fn is_finger_detected(&self, ir: u32, red: u32, average_ir: f32) -> bool {
        // Implementar lógica para detectar si un dedo está presente
        // Esto puede incluir comprobar si 'ir' y 'red' están dentro de un rango esperado y si hay suficiente variabilidad
        ir > MIN_IR_VALUE && red > MIN_RED_VALUE && (ir as f32 - average_ir).abs() > VARIABILITY_THRESHOLD
    }
}

const WINDOW_SIZE: usize = 10;
const PEAK_THRESHOLD: u32 = 50000; // Ajustar según los datos
const MIN_IR_VALUE: u32 = 20000; // Ajustar según los datos
const MIN_RED_VALUE: u32 = 20000; // Ajustar según los datos
const VARIABILITY_THRESHOLD: f32 = 1000.0; // Ajustar según los datos
const MIN_INTERVAL: f32 = 0.5; // 120 BPM

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
        let mut monitor = HeartRateMonitor::new();

        loop {
            if let Ok(mut max3010x) = m.lock() {
                red = max3010x.get_red();
                ir = max3010x.get_ir();

                if let Ok((red, ir)) = red.and_then(|red| ir.map(|ir| (red, ir))) {
                    monitor.update(ir, red);

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
