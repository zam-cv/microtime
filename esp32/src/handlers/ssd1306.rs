use crate::{
    drivers::ssd1306::Ssd1306,
    images::{BATERRY, WIFI}
};
use anyhow::Result;
use embedded_graphics::{
    mono_font::{ascii::*, MonoTextStyleBuilder},
    image::{Image, ImageRaw},
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

    let raw_battery: ImageRaw<BinaryColor> = ImageRaw::new(&BATERRY, 18);
    let img_battery = Image::new(&raw_battery, Point::new(110, 0));

    let raw_wifi: ImageRaw<BinaryColor> = ImageRaw::new(&WIFI, 13);
    let img_wifi = Image::new(&raw_wifi, Point::new(92, 0));

    let simple = MonoTextStyleBuilder::new()
        .font(&FONT_6X13)
        .text_color(BinaryColor::On)
        .build();

    let big = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(BinaryColor::On)
        .build();

    loop {
        let date = chrono::Local::now();

        ssd1306
            .display
            .clear(BinaryColor::Off)
            .map_err(|e| anyhow::anyhow!("Failed to clear display: {:?}", e))?;

        Text::with_baseline(
            date.format("%d/%m/%Y").to_string().as_str(),
            Point::zero(),
            simple,
            Baseline::Top,
        )
        .draw(&mut ssd1306.display)
        .map_err(|e| anyhow::anyhow!("Failed to draw text: {:?}", e))?;

        Text::with_baseline(
            date.format("%H:%M:%S").to_string().as_str(),
            Point::new(20, 30),
            big,
            Baseline::Top
        )
        .draw(&mut ssd1306.display)
        .map_err(|e| anyhow::anyhow!("Failed to draw text: {:?}", e))?;

        img_wifi
            .draw(&mut ssd1306.display)
            .map_err(|e| anyhow::anyhow!("Failed to draw image: {:?}", e))?;

        img_battery
            .draw(&mut ssd1306.display)
            .map_err(|e| anyhow::anyhow!("Failed to draw image: {:?}", e))?;

        ssd1306
            .display
            .flush()
            .map_err(|e| anyhow::anyhow!("Failed to flush display: {:?}", e))?;

        thread::sleep(Duration::from_millis(500));
    }
}
