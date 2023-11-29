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

const FIR_COEFFS: [u16; 12] = [172, 321, 579, 927, 1360, 1858, 2390, 2916, 3391, 3768, 4012, 4096];

struct HeartRateMonitor {
    ir_ac_max: i16,
    ir_ac_min: i16,
    ir_ac_signal_current: i16,
    ir_ac_signal_previous: i16,
    ir_ac_signal_min: i16,
    ir_ac_signal_max: i16,
    ir_average_estimated: i16,
    positive_edge: i16,
    negative_edge: i16,
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
            positive_edge: 0,
            negative_edge: 0,
            ir_avg_reg: 0,
            cbuf: [0; 32],
            offset: 0,
        }
    }

    fn check_for_beat(&mut self, sample: i32) -> bool {
        let mut beat_detected = false;
        self.ir_ac_signal_previous = self.ir_ac_signal_current;
        self.ir_average_estimated = self.average_dc_estimator(sample as u16);
        self.ir_ac_signal_current = self.low_pass_fir_filter((sample - self.ir_average_estimated as i32) as i16);

        if (self.ir_ac_signal_previous < 0) & (self.ir_ac_signal_current >= 0) {
            self.ir_ac_max = self.ir_ac_signal_max;
            self.ir_ac_min = self.ir_ac_signal_min;

            self.positive_edge = 1;
            self.negative_edge = 0;
            self.ir_ac_signal_max = 0;

            if (self.ir_ac_max - self.ir_ac_min) > 20 && (self.ir_ac_max - self.ir_ac_min) < 1000 {
                beat_detected = true;
            }
        }

        if (self.ir_ac_signal_previous > 0) & (self.ir_ac_signal_current <= 0) {
            self.positive_edge = 0;
            self.negative_edge = 1;
            self.ir_ac_signal_min = 0;
        }

        if self.positive_edge == 1 && self.ir_ac_signal_current > self.ir_ac_signal_previous {
            self.ir_ac_signal_max = self.ir_ac_signal_current;
        }

        if self.negative_edge == 1 && self.ir_ac_signal_current < self.ir_ac_signal_previous {
            self.ir_ac_signal_min = self.ir_ac_signal_current;
        }

        beat_detected
    }

    fn average_dc_estimator(&mut self, x: u16) -> i16 {
        self.ir_avg_reg += (((x as i32) << 15) - self.ir_avg_reg) >> 4;
        (self.ir_avg_reg >> 15) as i16
    }
    

    fn low_pass_fir_filter(&mut self, din: i16) -> i16 {
        let buf_size = self.cbuf.len();
        self.cbuf[self.offset] = din;
        let mut z = HeartRateMonitor::mul16(FIR_COEFFS[11] as i16, self.cbuf[(self.offset + buf_size - 11) & 0x1F]);

        for i in 0..11 {
            z += HeartRateMonitor::mul16(FIR_COEFFS[i] as i16, self.cbuf[(self.offset - i) & 0x1F] + self.cbuf[(self.offset + buf_size- 22 + i) & 0x1F]);
        }

        self.offset += 1;
        self.offset %= 32;

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
        let mut rates: [i64; 4] = [0; 4];
        let mut last_beat: i32 = 0;
        let mut beats_per_minute: i64 = 0;
        let mut rate_spot: u32 = 0;
        let mut beat_avg: i64 = 0;
        let mut heart_rate_monitor = HeartRateMonitor::new();

        loop {
            if let Ok(mut max3010x) = m.lock() {
                red = max3010x.get_red();
                ir = max3010x.get_ir();

                if let Ok((red, ir)) = red.and_then(|red| ir.map(|ir| (red, ir))) {
                    if heart_rate_monitor.check_for_beat(ir as i32) == true {
                        // let delta: i64 = chrono::Local::now().timestamp() - last_beat as i64;
                        // last_beat = chrono::Local::now().timestamp() as i32;

                        // millis
                        let delta: i64 = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_millis() as i64 - last_beat as i64;
                        // millis
                        last_beat = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_millis() as i32;
                        beats_per_minute = 60 / (delta / 1000);
    
                        if beats_per_minute < 255 && beats_per_minute > 20 {
                            rate_spot = rate_spot + 1;
                            rates[rate_spot as usize] = beats_per_minute;
                            rate_spot %= 4;
    
                            beat_avg = 0;
    
                            for i in 0..4 {
                                beat_avg += rates[i];
                                beat_avg /= 4;
                            }
                        }
                    }

                    info!("BEATS_PER_MINUTE => {}", beats_per_minute);
                    info!("SOCKET => red: {}, ir: {}", red, ir);
                    let _ = solver.send_to_socket(Message::new(Max3010x { red, ir }));
                } else {
                    info!("Error reading sensor");
                }
            }

            thread::sleep(Duration::from_secs(4));
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