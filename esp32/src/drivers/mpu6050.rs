use crate::handlers::mpu6050::{Accel, Rotation};
use anyhow::{anyhow, Result};
use embedded_hal::blocking::i2c::{Write, WriteRead};
use esp_idf_svc::hal::delay::*;
use mpu6050_dmp::{address::Address, quaternion::Quaternion, sensor, yaw_pitch_roll::YawPitchRoll};
use std::{error::Error, fmt::Debug};

impl Debug for Accel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Accel")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .finish()
    }
}

impl Debug for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rotation")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .finish()
    }
}

pub struct Mpu6050<I2C>
where
    I2C: Write + WriteRead,
    <I2C as Write>::Error: Error + Send + Sync + 'static,
    <I2C as WriteRead>::Error: Error + Send + Sync + 'static,
{
    sensor: sensor::Mpu6050<I2C>,
    buf: [u8; 64],
}

impl<I2C> Mpu6050<I2C>
where
    I2C: Write + WriteRead,
    <I2C as Write>::Error: Error + Send + Sync + 'static,
    <I2C as WriteRead>::Error: Error + Send + Sync + 'static,
{
    pub fn new(i2c: I2C) -> Result<Self> {
        let mut sensor = match sensor::Mpu6050::new(i2c, Address::default()) {
            Ok(sensor) => Ok(sensor),
            Err(_) => Err(anyhow!("Failed to initialize MPU6050")),
        }?;

        let mut delay = Delay::new(1000);
        let buf = [0; 64];

        if let Err(e) = sensor.initialize_dmp(&mut delay) {
            return Err(anyhow!("Failed to initialize MPU6050: {:?}", e));
        }

        Ok(Self { sensor, buf })
    }

    pub fn get_accel(&mut self) -> Result<Accel> {
        if let Ok(a) = self.sensor.accel() {
            return Ok(Accel {
                x: a.x(),
                y: a.y(),
                z: a.z(),
            });
        }

        return Err(anyhow!("Failed get accel"));
    }

    pub fn get_rotation(&mut self) -> Result<Rotation> {
        let len = self.sensor.get_fifo_count().unwrap_or(0);

        if len >= 28 {
            if let Ok(buf) = self.sensor.read_fifo(&mut self.buf) {
                if let Some(quaternion) = Quaternion::from_bytes(&buf[..16]) {
                    let ypr = YawPitchRoll::from(quaternion);

                    return Ok(Rotation {
                        x: ypr.yaw,
                        y: ypr.pitch,
                        z: ypr.roll,
                    });
                }
            }
        }

        return Err(anyhow!("Failed to read MPU6050"));
    }
}
