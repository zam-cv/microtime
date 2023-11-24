use anyhow::Result;
use embedded_hal::blocking::i2c::{Read, Write};
use esp_idf_svc::hal::delay::Delay;
use std::{error::Error, time::Duration, time::Instant};

static mut DELAY: Delay = Delay::new(1000);

const I2C_ADDRESS: u8 = 0x57;
const MODECONFIG: u8 = 0x09;
const RESET_MASK: u8 = 0xBF;
const RESET: u8 = 0x40;
const FIFOCONFIG: u8 = 0x08;

const SAMPLEAVG_MASK: u8 = 0xE0;
const SAMPLEAVG_1: u8 = 0x00;
const SAMPLEAVG_2: u8 = 0x20;
const SAMPLEAVG_4: u8 = 0x40;
const SAMPLEAVG_8: u8 = 0x60;
const SAMPLEAVG_16: u8 = 0x80;
const SAMPLEAVG_32: u8 = 0xA0;

const ROLLOVER_MASK: u8 = 0xEF;
const ROLLOVER_ENABLE: u8 = 0x10;
const MODE_MASK: u8 = 0xF8;
const MODE_MULTILED: u8 = 0x07;
const MODE_REDIRONLY: u8 = 0x03;
const MODE_REDONLY: u8 = 0x02;
const PARTICLECONFIG: u8 = 0x0A;
const ADCRANGE_MASK: u8 = 0x9F;

const SAMPLERATE_50: u8 = 0x00;
const SAMPLERATE_100: u8 = 0x04;
const SAMPLERATE_200: u8 = 0x08;
const SAMPLERATE_400: u8 = 0x0C;
const SAMPLERATE_800: u8 = 0x10;
const SAMPLERATE_1000: u8 = 0x14;
const SAMPLERATE_1600: u8 = 0x18;
const SAMPLERATE_3200: u8 = 0x1C;

const PULSEWIDTH_MASK: u8 = 0xFC;
const PULSEWIDTH_69: u8 = 0x00;
const PULSEWIDTH_118: u8 = 0x01;
const PULSEWIDTH_215: u8 = 0x02;
const PULSEWIDTH_411: u8 = 0x03;

const LED1_PULSEAMP: u8 = 0x0C;
const LED2_PULSEAMP: u8 = 0x0D;
const LED3_PULSEAMP: u8 = 0x0E;
const LED_PROX_AMP: u8 = 0x10;

const MULTILEDCONFIG1: u8 = 0x11;
const MULTILEDCONFIG2: u8 = 0x12;
const SLOT1_MASK: u8 = 0xF8;
const SLOT2_MASK: u8 = 0x8F;
const SLOT3_MASK: u8 = 0xF8;
const SLOT4_MASK: u8 = 0x8F;

const SLOT_RED_LED: u8 = 0x01;
const SLOT_IR_LED: u8 = 0x02;
const SLOT_GREEN_LED: u8 = 0x03;

const FIFOWRITEPTR: u8 = 0x04;
const FIFOOVERFLOW: u8 = 0x05;
const FIFOREADPTR: u8 = 0x06;

const STORAGE_SIZE: usize = 4;
const FIFODATA: u8 = 0x07;
const I2C_BUFFER_LENGTH: usize = 32;

const SAMPLERATE_MASK: u8 = 0xE3;

type Byte = u8;

pub struct Sense {
    head: Byte,
    red: [u32; STORAGE_SIZE],
    ir: [u32; STORAGE_SIZE],
    green: [u32; STORAGE_SIZE],
}

pub struct MAX3010x<I2C>
where
    I2C: Write + Read,
{
    i2c: I2C,
    sense: Sense,
    active_leds: Byte,
}

pub struct Config {
    pub power_level: Byte,
    pub sample_average: Byte,
    pub led_mode: Byte,
    pub sample_rate: u32,
    pub pulse_width: u32,
    pub adc_range: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            power_level: 0xFF,
            sample_average: 4,
            led_mode: 3,
            sample_rate: 400,
            pulse_width: 411,
            adc_range: 4096,
        }
    }
}

impl<I2C> MAX3010x<I2C>
where
    I2C: Write + Read,
    <I2C as Write>::Error: Error + Send + Sync + Sized + 'static,
    <I2C as Read>::Error: Error + Send + Sync + Sized + 'static,
{
    pub fn new(i2c: I2C, config: &Config) -> Result<Self> {
        let mut max3010x = Self {
            i2c,
            sense: Sense {
                head: 0,
                red: [0; STORAGE_SIZE],
                ir: [0; STORAGE_SIZE],
                green: [0; STORAGE_SIZE],
            },
            active_leds: 0,
        };

        max3010x.setup(&config)?;
        Ok(max3010x)
    }

    pub fn read_register(&mut self, address: u8, reg: u8) -> Result<u8> {
        self.i2c.write(address, &[reg])?;
        let mut data = [0u8; 1];
        self.i2c.read(address, &mut data)?;
        Ok(data[0])
    }

    pub fn write_register(&mut self, address: u8, reg: u8, value: u8) -> Result<()> {
        self.i2c.write(address, &[reg, value])?;
        Ok(())
    }

    pub fn bit_mask(&mut self, reg: u8, mask: u8, thing: u8) -> Result<()> {
        let original_contents = self.read_register(I2C_ADDRESS, reg)?;
        let new_contents = (original_contents & mask) | thing;
        self.write_register(I2C_ADDRESS, reg, new_contents)?;
        Ok(())
    }

    pub fn set_fifo_average(&mut self, samples: u8) -> Result<()> {
        self.bit_mask(FIFOCONFIG, SAMPLEAVG_MASK, samples << 5)
    }

    pub fn enable_fifo_rollover(&mut self) -> Result<()> {
        self.bit_mask(FIFOCONFIG, ROLLOVER_MASK, ROLLOVER_ENABLE)
    }

    pub fn soft_reset(&mut self) -> Result<()> {
        self.bit_mask(MODECONFIG, RESET_MASK, RESET)?;

        let start = Instant::now();
        while Instant::now().duration_since(start) < Duration::from_millis(100) {
            let response = self.read_register(I2C_ADDRESS, RESET)?;
            if response & RESET == 0 {
                break;
            }
            return Ok(());
        }

        Ok(())
    }

    pub fn set_led_mode(&mut self, mode: u8) -> Result<()> {
        self.bit_mask(MODECONFIG, MODE_MASK, mode)
    }

    pub fn set_adc_range(&mut self, adc_range: u8) -> Result<()> {
        self.bit_mask(PARTICLECONFIG, ADCRANGE_MASK, adc_range)
    }

    pub fn set_pulse_width(&mut self, pulse_width: u8) -> Result<()> {
        self.bit_mask(PARTICLECONFIG, PULSEWIDTH_MASK, pulse_width)
    }

    pub fn set_sample_rate(&mut self, sample_rate: u8) -> Result<()> {
        self.bit_mask(PARTICLECONFIG, SAMPLERATE_MASK, sample_rate)
    }

    pub fn set_pulse_amplitude_red(&mut self, amplitude: u8) -> Result<()> {
        self.write_register(I2C_ADDRESS, LED1_PULSEAMP, amplitude)
    }

    pub fn set_pulse_amplitude_ir(&mut self, amplitude: u8) -> Result<()> {
        self.write_register(I2C_ADDRESS, LED2_PULSEAMP, amplitude)
    }

    pub fn set_pulse_amplitude_green(&mut self, amplitude: u8) -> Result<()> {
        self.write_register(I2C_ADDRESS, LED3_PULSEAMP, amplitude)
    }

    pub fn set_pulse_amplitude_proximity(&mut self, amplitude: u8) -> Result<()> {
        self.write_register(I2C_ADDRESS, LED_PROX_AMP, amplitude)
    }

    pub fn enable_slot(&mut self, slot_number: u8, device: u8) -> Result<()> {
        match slot_number {
            1 => self.bit_mask(MULTILEDCONFIG1, SLOT1_MASK, device),
            2 => self.bit_mask(MULTILEDCONFIG1, SLOT2_MASK, device << 4),
            3 => self.bit_mask(MULTILEDCONFIG2, SLOT3_MASK, device),
            4 => self.bit_mask(MULTILEDCONFIG2, SLOT4_MASK, device << 4),
            _ => Ok(()),
        }
    }

    pub fn clear_fifo(&mut self) -> Result<()> {
        self.write_register(I2C_ADDRESS, FIFOWRITEPTR, 0)?;
        self.write_register(I2C_ADDRESS, FIFOOVERFLOW, 0)?;
        self.write_register(I2C_ADDRESS, FIFOREADPTR, 0)?;

        Ok(())
    }

    pub fn get_read_pointer(&mut self) -> Result<u8> {
        self.read_register(I2C_ADDRESS, FIFOREADPTR)
    }

    pub fn get_write_pointer(&mut self) -> Result<u8> {
        self.read_register(I2C_ADDRESS, FIFOWRITEPTR)
    }

    pub fn check(&mut self) -> Result<u16> {
        let read_pointer = self.get_read_pointer()?;
        let write_pointer = self.get_write_pointer()?;

        let mut number_of_samples: i16 = 0;

        if read_pointer != write_pointer {
            number_of_samples = write_pointer as i16 - read_pointer as i16;

            if number_of_samples < 0 {
                number_of_samples += 32;
            }

            let mut bytes_left_to_read = number_of_samples * self.active_leds as i16 * 3;
            self.i2c.write(I2C_ADDRESS, &[FIFODATA])?;

            while bytes_left_to_read > 0 {
                let mut to_get: i16 = bytes_left_to_read;

                if to_get as usize > I2C_BUFFER_LENGTH {
                    to_get = I2C_BUFFER_LENGTH as i16
                        - (I2C_BUFFER_LENGTH as i16 % (self.active_leds as i16 * 3));
                }

                bytes_left_to_read -= to_get;

                let mut buffer = vec![0u8; to_get as usize];

                self.i2c.read(I2C_ADDRESS, &mut buffer)?;

                while to_get > 0 {
                    self.sense.head += 1;
                    self.sense.head %= STORAGE_SIZE as u8;

                    let mut temp: [u8; 4] = [0; 4];
                    self.i2c.read(I2C_ADDRESS, &mut temp)?;

                    let mut temp_long = u32::from_be_bytes(temp) & 0x3FFFF;
                    self.sense.red[self.sense.head as usize] = temp_long;

                    if self.active_leds > 1 {
                        self.i2c.read(I2C_ADDRESS, &mut temp)?;
                        temp_long = u32::from_be_bytes(temp) & 0x3FFFF;

                        self.sense.ir[self.sense.head as usize] = temp_long;
                    }

                    if self.active_leds > 2 {
                        self.i2c.read(I2C_ADDRESS, &mut temp)?;
                        temp_long = u32::from_be_bytes(temp) & 0x3FFFF;

                        temp_long &= 0x3FFFF;
                        self.sense.green[self.sense.head as usize] = temp_long;
                    }

                    to_get -= self.active_leds as i16 * 3;
                }
            }
        }

        Ok(number_of_samples as u16)
    }

    pub fn safe_check(&mut self, max_time_to_clock: u8) -> Result<bool> {
        let mark_time = Instant::now();

        loop {
            if Instant::now().duration_since(mark_time)
                > Duration::from_millis(max_time_to_clock as u64)
            {
                return Ok(false);
            }

            if self.check()? > 0 {
                return Ok(true);
            }

            unsafe {
                DELAY.delay_ms(1);
            }
        }
    }

    pub fn get_red(&mut self) -> Result<u32> {
        if self.safe_check(250)? {
            return Ok(self.sense.red[self.sense.head as usize]);
        } else {
            return Ok(0);
        }
    }

    pub fn get_ir(&mut self) -> Result<u32> {
        if self.safe_check(250)? {
            return Ok(self.sense.ir[self.sense.head as usize]);
        } else {
            return Ok(0);
        }
    }

    pub fn setup(&mut self, config: &Config) -> Result<()> {
        self.soft_reset()?;

        match config.sample_average {
            1 => self.set_fifo_average(SAMPLEAVG_1)?,
            2 => self.set_fifo_average(SAMPLEAVG_2)?,
            4 => self.set_fifo_average(SAMPLEAVG_4)?,
            8 => self.set_fifo_average(SAMPLEAVG_8)?,
            16 => self.set_fifo_average(SAMPLEAVG_16)?,
            32 => self.set_fifo_average(SAMPLEAVG_32)?,
            _ => self.set_fifo_average(SAMPLEAVG_4)?,
        }

        self.enable_fifo_rollover()?;

        match config.led_mode {
            3 => self.set_led_mode(MODE_MULTILED)?,
            2 => self.set_led_mode(MODE_REDIRONLY)?,
            _ => self.set_led_mode(MODE_REDONLY)?,
        }

        self.active_leds = config.led_mode;

        match config.adc_range {
            0..=4095 => self.set_adc_range(0x00)?,
            4096..=8191 => self.set_adc_range(0x20)?,
            8192..=16383 => self.set_adc_range(0x40)?,
            16384 => self.set_adc_range(0x60)?,
            _ => self.set_adc_range(0x00)?,
        }

        match config.sample_rate {
            0..=99 => self.set_sample_rate(SAMPLERATE_50)?,
            100..=199 => self.set_sample_rate(SAMPLERATE_100)?,
            200..=399 => self.set_sample_rate(SAMPLERATE_200)?,
            400..=799 => self.set_sample_rate(SAMPLERATE_400)?,
            800..=999 => self.set_sample_rate(SAMPLERATE_800)?,
            1000..=1599 => self.set_sample_rate(SAMPLERATE_1000)?,
            1600..=3199 => self.set_sample_rate(SAMPLERATE_1600)?,
            3200 => self.set_sample_rate(SAMPLERATE_3200)?,
            _ => self.set_sample_rate(SAMPLERATE_50)?,
        }

        match config.pulse_width {
            0..=117 => self.set_pulse_width(PULSEWIDTH_69)?,
            118..=214 => self.set_pulse_width(PULSEWIDTH_118)?,
            215..=410 => self.set_pulse_width(PULSEWIDTH_215)?,
            411 => self.set_pulse_width(PULSEWIDTH_411)?,
            _ => self.set_pulse_width(PULSEWIDTH_69)?,
        }

        self.set_pulse_amplitude_red(config.power_level)?;
        self.set_pulse_amplitude_ir(config.power_level)?;
        self.set_pulse_amplitude_green(config.power_level)?;
        self.set_pulse_amplitude_proximity(config.power_level)?;

        self.enable_slot(1, SLOT_RED_LED)?;

        if config.led_mode > 1 {
            self.enable_slot(2, SLOT_IR_LED)?;
        }

        if config.led_mode > 2 {
            self.enable_slot(3, SLOT_GREEN_LED)?;
        }

        self.clear_fifo()?;
        Ok(())
    }
}
