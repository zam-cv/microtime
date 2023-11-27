use embedded_hal::blocking::i2c::{Read, Write, WriteRead};
use esp_idf_svc::hal::{gpio::AnyIOPin, i2c::I2cDriver};
use std::sync::{Arc, Mutex};

const TIMEOUT: u32 = 100000;

pub struct Driver<'d> {
    i2c: Mutex<I2cDriver<'d>>,
}

impl<'d> Driver<'d> {
    pub fn new(i2c: I2cDriver<'d>) -> Self {
        Self {
            i2c: Mutex::new(i2c),
        }
    }
}

pub struct ArcDriver<'d>(Arc<Driver<'d>>);

impl<'d> ArcDriver<'d> {
    pub fn new(i2c: I2cDriver<'d>) -> Self {
        Self(Arc::new(Driver::new(i2c)))
    }

    pub fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<'d> Clone for ArcDriver<'d> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<'d> Write for ArcDriver<'d> {
    type Error = std::io::Error;

    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        if let Ok(mut i2c) = self.0.i2c.lock() {
            if let Err(e) = i2c.write(addr, bytes, TIMEOUT) {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to write to I2C: {:?}", e),
                ))
            } else {
                Ok(())
            }
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to write to I2C",
            ))
        }
    }
}

impl<'d> Read for ArcDriver<'d> {
    type Error = std::io::Error;

    fn read(&mut self, addr: u8, bytes: &mut [u8]) -> Result<(), Self::Error> {
        if let Ok(mut i2c) = self.0.i2c.lock() {
            if let Err(e) = i2c.read(addr, bytes, TIMEOUT) {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to read from I2C: {:?}", e),
                ))
            } else {
                Ok(())
            }
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to read from I2C",
            ))
        }
    }
}

impl<'d> WriteRead for ArcDriver<'d> {
    type Error = std::io::Error;

    fn write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Self::Error> {
        if let Ok(mut i2c) = self.0.i2c.lock() {
            if let Err(e) = i2c.write_read(addr, bytes, buffer, TIMEOUT) {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to write/read from I2C: {:?}", e),
                ))
            } else {
                Ok(())
            }
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to write/read from I2C",
            ))
        }
    }
}

pub struct PinAsync(pub AnyIOPin);

unsafe impl Send for PinAsync {}
unsafe impl Sync for PinAsync {}

impl PinAsync {
    pub fn pin(&self) -> AnyIOPin {
        self.clone().0
    }
}

impl Clone for PinAsync {
    fn clone(&self) -> Self {
        unsafe {
            let ptr = &self.0 as *const AnyIOPin;
            let pin = std::ptr::read(ptr);
            PinAsync(pin)
        }
    }
}
