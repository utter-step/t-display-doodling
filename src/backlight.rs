use std::error::Error;

use esp_idf_hal::{
    ledc::{
        config::TimerConfig,
        LedcDriver, LedcTimerDriver,
        CHANNEL0,
        TIMER0,
    },
    units::FromValueType,
    gpio::Gpio4,
};
use log::{info, warn};

pub struct Backlight {
    driver: LedcDriver<'static>,
}

impl Backlight {
    pub fn create_backlight(timer0: TIMER0, channel0: CHANNEL0, bl_pin: Gpio4) -> Result<Self, Box<dyn Error>> {
        let timer_config = TimerConfig::default().frequency(25.kHz().into());
        let timer = LedcTimerDriver::new(timer0, &timer_config)?;
        let mut driver = LedcDriver::new(channel0, &timer, bl_pin)?;

        let max_duty = driver.get_max_duty();
        info!("Setting backlight to max duty (100% = {max_duty})");
        driver.set_duty(max_duty)?;

        let _self = Self { driver };

        Ok(_self)
    }

    pub fn set(&mut self, mut percents: u32) -> Result<(), Box<dyn Error>> {
        if percents > 100 {
            warn!("Backlight percentage is too high ({percents}), setting to 100%");
            percents = 100;
        }

        let max_duty = self.driver.get_max_duty();
        let target_duty = max_duty * percents / 100;
        info!("Setting backlight to {percents}% (duty = {target_duty})");

        Ok(self.driver.set_duty(target_duty)?)
    }
}

macro_rules! create {
    ($peripherals: expr) => {
        crate::backlight::Backlight::create_backlight(
            $peripherals.ledc.timer0,
            $peripherals.ledc.channel0,
            $peripherals.pins.gpio4,
        )?
    };
}

pub(crate) use create;
