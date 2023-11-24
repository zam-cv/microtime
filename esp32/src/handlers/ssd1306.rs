use crate::drivers::ssd1306::Ssd1306;
use anyhow::Result;
use embedded_graphics::{
    mono_font::{ascii::*, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use embedded_hal::blocking::i2c::Write;
use std::{thread, time::Duration};

pub fn ssd1306<I2C>(i2c: I2C) -> Result<()>
where
    I2C: Write
{
    let mut ssd1306 = Ssd1306::new(i2c)?;

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    let mut count = 0;

    loop {
        ssd1306
            .display
            .clear(BinaryColor::Off)
            .map_err(|e| anyhow::anyhow!("Failed to clear display: {:?}", e))?;

        Text::with_baseline(
            count.to_string().as_str(),
            Point::zero(),
            text_style,
            Baseline::Top,
        )
        .draw(&mut ssd1306.display)
        .map_err(|e| anyhow::anyhow!("Failed to draw text: {:?}", e))?;

        ssd1306
            .display
            .flush()
            .map_err(|e| anyhow::anyhow!("Failed to flush display: {:?}", e))?;

        count += 1;
        thread::sleep(Duration::from_secs(1));
    }
}
