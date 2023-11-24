use anyhow::{anyhow, Result};
use embedded_hal::blocking::i2c::Write;
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface};

pub struct Ssd1306<I2C>
where
    I2C: Write,
{
    pub display: ssd1306::Ssd1306<
        I2CInterface<I2C>,
        DisplaySize128x64,
        BufferedGraphicsMode<DisplaySize128x64>,
    >,
}

impl<I2C> Ssd1306<I2C>
where
    I2C: Write,
{
    pub fn new(i2c: I2C) -> Result<Self> {
        let interface = I2CDisplayInterface::new(i2c);

        let mut display =
            ssd1306::Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
                .into_buffered_graphics_mode();

        display
            .init()
            .map_err(|_| anyhow!("Failed to initialize display"))?;
        Ok(Self { display })
    }
}
