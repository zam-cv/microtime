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
    // pub accel: Accel,
    // pub rotation: Rotation,
    pub steps: u32
}

fn count_steps(data: &[Accel]) -> i16 {
    let mut step_count = 0;
    let mut last_peak = 0;
    let mut last_valley = 0;
    let mut is_peak = false;
    let peak_threshold = 1; // Umbral para la detección de picos
    let step_threshold = 1; // Umbral para la diferencia entre pico y valle para contar un paso

    for i in 1..data.len() - 1 {
        let prev = data[i - 1].z;
        let current = data[i].z;
        let next = data[i + 1].z;

        if current > prev && current > next {
            is_peak = true;
            last_peak = current;
        } else if current < prev && current < next {
            last_valley = current;
            if is_peak && (last_peak - last_valley > step_threshold) {
                step_count += 1;
                is_peak = false;
            }
        }
    }

    step_count
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
        //let mut accx;

        loop {
            if let Ok(mut mpu6050) = mpu6050.lock() {
                accel = mpu6050.get_accel();
                rotation = mpu6050.get_rotation();

                if let Ok((accel, rotation)) = accel.and_then(|a| rotation.map(|r| (a, r))) {
                    // if accel.x.abs() > 1000 as i16 {
                    //     accx = accel.x.abs();
                    // } else {
                    //     accx = 0;
                    // }

                    // let interval: f32 = 0.08;
                    // let distance_x: f32 = accx as f32 * interval * interval / 2.0;
                    // total_distance += distance_x;
                    // pasos = pasos + total_distance / 0.7;

                    let data = [
                        Accel { x: accel.x, y: accel.y, z: accel.z },
        // ... más datos aquí
                                ];
                    
                    info!("PASOS => {}", pasos);
                    info!("SOCKET => accel: {:?}, rotation: {:?}", accel, rotation);

                    let _ = solver.send_to_socket(Message::new(Mpu6050 { 
                        steps: 0
                     }));
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
                    let _ = solver.send_to_database(Message::new(Mpu6050 { 
                        steps: 0
                     }));
                } else {
                    info!("Error reading sensor");
                }
            }

            thread::sleep(Duration::from_secs(3));
        }
    });

    Ok(())
}
