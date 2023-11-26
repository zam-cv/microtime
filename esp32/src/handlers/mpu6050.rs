use crate::{
    drivers::mpu6050::Mpu6050 as Sensor,
    solver::{Message, Solver},
};
use anyhow::Result;
use embedded_hal::blocking::i2c::{Write, WriteRead};
use log::info;
use serde::{Deserialize, Serialize};
use std::{error::Error, sync::Arc, thread, time::Duration};

#[derive(Serialize, Deserialize)]
pub struct Accel {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

#[derive(Serialize, Deserialize)]
pub struct Rotation {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Mpu6050 {
    pub accel: Accel,
    pub rotation: Rotation,
}

#[derive(Serialize, Deserialize)]
pub struct Test {
    pub test: String,
}

pub fn mpu6050<I2C>(i2c: I2C, solver: Arc<Solver>) -> Result<()>
where
    I2C: WriteRead + Write,
    <I2C as WriteRead>::Error: Error + Send + Sync + Sized + 'static,
    <I2C as Write>::Error: Error + Send + Sync + Sized + 'static,
{
    let mut mpu6050 = Sensor::new(i2c)?;
    let mut accel;
    let mut rotation;

    loop {
        accel = mpu6050.get_accel();
        rotation = mpu6050.get_rotation();

        if let Ok((accel, rotation)) =
            accel.and_then(|accel| rotation.map(|rotation| (accel, rotation)))
        {
            info!("accel: {:?}, rotation: {:?}", accel, rotation);
            solver.send_to_database(Message::new(Mpu6050 { accel, rotation }))?;
        } else {
            info!("Error reading sensor");
        }

        thread::sleep(Duration::from_secs(1));
    }
}
