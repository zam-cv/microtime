use anyhow::{anyhow, Result};
use esp_idf_svc::{
    hal::{delay::Delay, gpio::*},
    sys::bind,
};
use log::info;

static mut DELAY: Delay = Delay::new(1000);
static mut PIN: Option<PinDriver<'static, AnyIOPin, InputOutput>> = None;
static mut OBJECT: bind::ds18b20_handle_t = bind::ds18b20_handle_t {
    bus_init: Some(bus_init),
    bus_deinit: Some(bus_deinit),
    bus_read: Some(bus_read),
    bus_write: Some(bus_write),
    delay_ms: Some(delay_ms),
    delay_us: Some(delay_us),
    enable_irq: Some(enable_interrupt),
    disable_irq: Some(disable_interrupt),
    debug_print: Some(debug_print),
    inited: 0,
    mode: 0,
    rom: [0; 8],
};

unsafe extern "C" fn bus_init() -> u8 {
    0
}

unsafe extern "C" fn bus_deinit() -> u8 {
    0
}

unsafe extern "C" fn bus_read(value: *mut u8) -> u8 {
    unsafe {
        if let Some(pin) = PIN.as_mut() {
            *value = if pin.is_high() { 1 } else { 0 };

            0
        } else {
            info!("Sensor data not available");
            1
        }
    }
}

unsafe extern "C" fn bus_write(value: u8) -> u8 {
    if let Some(pin) = PIN.as_mut() {
        let err = if value == 0 {
            pin.set_low()
        } else {
            pin.set_high()
        };

        if let Err(e) = err {
            info!("Failed to write to pin: {:?}", e);
        }

        0
    } else {
        info!("Sensor data not available");
        1
    }
}

unsafe extern "C" fn delay_ms(ms: u32) {
    DELAY.delay_ms(ms);
}

unsafe extern "C" fn delay_us(us: u32) {
    DELAY.delay_us(us);
}

unsafe extern "C" fn enable_interrupt() {
    if let Some(pin) = PIN.as_mut() {
        if let Err(e) = pin.enable_interrupt() {
            info!("Failed to enable interrupt: {:?}", e);
        }
    } else {
        info!("Sensor data not available");
    }
}

unsafe extern "C" fn disable_interrupt() {
    if let Some(pin) = PIN.as_mut() {
        if let Err(e) = pin.disable_interrupt() {
            info!("Failed to disable interrupt: {:?}", e);
        }
    } else {
        info!("Sensor data not available");
    }
}

#[allow(unused_variables)]
unsafe extern "C" fn debug_print(fmt: *const i8, args: ...) {
    if let Ok(fmt) = std::ffi::CStr::from_ptr(fmt).to_str() {
        info!("{}", fmt);
    }
}

pub struct Ds18b20 {
    handle: *mut bind::ds18b20_handle_t,
    response: u8,
    temperature: f32,
    raw_temperature: i16,
}

impl Ds18b20 {
    pub fn new(pin: AnyIOPin) -> Result<Self> {
        unsafe {
            if PIN.is_none() {
                PIN = Some(PinDriver::input_output(pin)?);
            }
        }

        let handle = unsafe { &mut OBJECT as *mut _ };
        let response = 0;
        let temperature: f32 = 0.0;
        let raw_temperature: i16 = 0;

        let mut ds18b20 = Self {
            handle,
            response,
            temperature,
            raw_temperature,
        };

        ds18b20.begin()?;
        Ok(ds18b20)
    }

    fn begin(&mut self) -> Result<()> {
        unsafe {
            self.response = bind::ds18b20_init(self.handle);

            if self.response != 0 {
                return Err(anyhow!("Failed to initialize sensor"));
            }

            self.response =
                bind::ds18b20_set_mode(self.handle, bind::ds18b20_mode_t_DS18B20_MODE_SKIP_ROM);
            if self.response != 0 {
                bind::ds18b20_deinit(self.handle);
                return Err(anyhow!("Failed to set sensor mode"));
            }

            self.response = bind::ds18b20_scratchpad_set_resolution(
                self.handle,
                bind::ds18b20_resolution_t_DS18B20_RESOLUTION_12BIT,
            );
            if self.response != 0 {
                info!("ds18b20 set resolution failed");
                bind::ds18b20_deinit(self.handle);

                return Err(anyhow!("Failed to set sensor resolution"));
            }
        }

        Ok(())
    }

    pub fn get_temp(&mut self) -> Result<f32> {
        unsafe {
            if bind::ds18b20_read(
                self.handle,
                &mut self.raw_temperature as *mut _,
                &mut self.temperature as *mut _,
            ) != 0
            {
                1
            } else {
                0
            };

            if self.response != 0 {
                bind::ds18b20_deinit(self.handle);
                return Err(anyhow!("Failed to read sensor"));
            }
        }

        Ok(self.temperature)
    }
}
