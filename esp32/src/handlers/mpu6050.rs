use crate::{
    drivers::mpu6050::Mpu6050 as Sensor,
    solver::{Message, Solver},
    utils::check,
};
use anyhow::Result;
use embedded_hal::blocking::i2c::{Write, WriteRead};
use log::info;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

#[derive(Serialize, Deserialize)]
pub struct Accel {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

#[derive(Serialize, Deserialize)]
pub struct Rotation {
    pub yaw: f32,
    pub patch: f32,
    pub roll: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Mpu6050 {
    pub accel: Accel,
    pub rotation: Rotation,
}

pub fn mpu6050<I2C>(i2c: I2C, solver: Arc<Solver>) -> Result<()>
where
    I2C: WriteRead + Write + Send + Sync + Clone + 'static,
    <I2C as WriteRead>::Error: Error + Send + Sync + Sized + 'static,
    <I2C as Write>::Error: Error + Send + Sync + Sized + 'static,
{
    let mpu6050 = Arc::new(Mutex::new(Sensor::new(i2c.clone())?));

    let m = mpu6050.clone();
    let s = solver.clone();
    thread::spawn(move || {
        let check = check::Check::new(20);
        let mpu6050 = Arc::clone(&m);
        let solver = Arc::clone(&s);
        let mut accel;
        let mut rotation;
        let mut total_distance: f32 = 0.0;
        let mut pasos: f32 = 0.0;

        loop {
            if let Ok(mut mpu6050) = mpu6050.lock() {
                accel = mpu6050.get_accel();
                rotation = mpu6050.get_rotation();
                let mut accx = accel.x;

                if let Ok((accel, rotation)) = accel.and_then(|a| rotation.map(|r| (a, r))) {
                    if accel.x > 100 | accel.x < -100{
                        accx = accel.x;
                    } else {
                        accx = 0;
                    }
                    let interval: f32 = 0.08;
                    let distance_x: f32 = accx as f32 * interval * interval / 2.0;
                    total_distance += distance_x;
                    pasos = pasos + total_distance / 0.7;
                    





                    info!("pasos => {}", pasos);
                    info!("SOCKET => accel: {:?}, rotation: {:?}", accel, rotation);
                    let _ = solver.send_to_socket(Message::new(Mpu6050 { accel, rotation }));
                } else {
                    info!("Error reading sensor");
                    check.error();

                    if check.is_limit() {
                        thread::sleep(Duration::from_secs(5));
                        if let Ok(sensor) = Sensor::new(i2c.clone()) {
                            *mpu6050 = sensor;
                        } else {
                            info!("Error creating sensor");
                        }
                    }
                }
            }

            thread::sleep(Duration::from_secs(1));
        }
    });

    let m = mpu6050.clone();
    let s = solver.clone();
    thread::spawn(move || {
        let mpu6050 = Arc::clone(&m);
        let solver = Arc::clone(&s);
        let mut accel;
        let mut rotation;

        loop {
            if let Ok(mut mpu6050) = mpu6050.lock() {
                accel = mpu6050.get_accel();
                rotation = mpu6050.get_rotation();

                if let Ok((accel, rotation)) = accel.and_then(|a| rotation.map(|r| (a, r))) {
                    info!("DATABASE => accel: {:?}, rotation: {:?}", accel, rotation);
                    let _ = solver.send_to_database(Message::new(Mpu6050 { accel, rotation }));
                } else {
                    info!("Error reading sensor");
                }
            }

            thread::sleep(Duration::from_secs(3));
        }
    });

    Ok(())
}
