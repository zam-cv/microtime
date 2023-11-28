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
    time::Duration, array,
};

#[derive(Serialize, Deserialize)]
pub struct Max3010x {
    pub red: u32,
    pub ir: u32,
}


struct HeartRateMonitor {
    ir_ac_max: i16,
    ir_ac_min: i16,
    ir_ac_signal_current: i16,
    ir_ac_signal_previous: i16,
    ir_ac_signal_min: i16,
    ir_ac_signal_max: i16,
    ir_average_estimated: i16,
    positive_edge: bool,
    negative_edge: bool,
    ir_avg_reg: i32,
    cbuf: [i16; 32],
    offset: usize,
}

impl HeartRateMonitor {
    fn new() -> HeartRateMonitor {
        HeartRateMonitor {
            ir_ac_max: 20,
            ir_ac_min: -20,
            ir_ac_signal_current: 0,
            ir_ac_signal_previous: 0,
            ir_ac_signal_min: 0,
            ir_ac_signal_max: 0,
            ir_average_estimated: 0,
            positive_edge: false,
            negative_edge: false,
            ir_avg_reg: 0,
            cbuf: [0; 32],
            offset: 0,
        }
    }

    fn check_for_beat(&mut self, sample: i32) -> bool {
        let mut beat_detected = false;
        self.ir_ac_signal_previous = self.ir_ac_signal_current;
        self.ir_average_estimated = self.average_dc_estimator(sample as i16);
        self.ir_ac_signal_current = self.low_pass_fir_filter(sample - self.ir_average_estimated as i32);

        if self.ir_ac_signal_previous < 0 && self.ir_ac_signal_current >= 0 {
            self.ir_ac_max = self.ir_ac_signal_max;
            self.ir_ac_min = self.ir_ac_signal_min;

            self.positive_edge = true;
            self.negative_edge = false;
            self.ir_ac_signal_max = 0;

            if (self.ir_ac_max - self.ir_ac_min) > 20 && (self.ir_ac_max - self.ir_ac_min) < 1000 {
                beat_detected = true;
            }
        }

        if self.ir_ac_signal_previous > 0 && self.ir_ac_signal_current <= 0 {
            self.positive_edge = false;
            self.negative_edge = true;
            self.ir_ac_signal_min = 0;
        }

        if self.positive_edge && self.ir_ac_signal_current > self.ir_ac_signal_previous {
            self.ir_ac_signal_max = self.ir_ac_signal_current;
        }

        if self.negative_edge && self.ir_ac_signal_current < self.ir_ac_signal_previous {
            self.ir_ac_signal_min = self.ir_ac_signal_current;
        }

        beat_detected
    }

    fn average_dc_estimator(&mut self, x: i16) -> i16 {
        self.ir_avg_reg += (((x as i32) << 15) - self.ir_avg_reg) >> 4;
        (self.ir_avg_reg >> 15) as i16
    }

    fn low_pass_fir_filter(&mut self, din: i32) -> i16 {
        self.cbuf[self.offset] = din as i16;

        let mut z = HeartRateMonitor::mul16(FIR_COEFFS[11] as i16, self.cbuf[(self.offset + 11) % 32]);
        for i in 0..11 {
            z += HeartRateMonitor::mul16(FIR_COEFFS[i] as i16, self.cbuf[(self.offset + i) % 32] + self.cbuf[(self.offset + 22 - i) % 32]);
        }

        self.offset = (self.offset + 1) % 32;
        (z >> 15) as i16
    }

    fn mul16(x: i16, y: i16) -> i32 {
        x as i32 * y as i32
    }
}

pub struct Headers {
    pub timestamp: i64,
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
        let timestamp = chrono::Local::now().timestamp();
        let rates: [i32; 4];
        let mut last_beat: i32 = 0;
        let mut beats_per_minute: f32 = 0.0;
        let mut rate_spot: i32 = 0;
        let mut beat_avg: i32 = 0;

        loop {
            if let Ok(mut max3010x) = m.lock() {
                red = max3010x.get_red();
                ir = max3010x.get_ir();

                if check_for_beat(ir) == true{
                    let delta: i32 = chrono::Local::now().timestamp() - last_beat;
                    last_beat = chrono::Local::now().timestamp();
                    beats_per_minute = 60 / (delta as f32 / 1000.0);

                    if(beats_per_minute < 255 && beats_per_minute > 20){
                        rates[rate_spot+=1] = beats_per_minute as i32;
                        rate_spot %= 4;

                        beat_avg = 0;

                        for i in 0..4{
                            beat_avg += rates[i];
                            beat_avg /= 4;
                        }
                    }
                }

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