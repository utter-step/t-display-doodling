use std::error::Error;

use embedded_graphics::{
    image::Image,
    pixelcolor::Rgb565,
    prelude::{DrawTarget, Point},
    Drawable,
};
use esp_idf_hal::{
    delay::FreeRtos,
    peripherals::Peripherals,
    gpio::PinDriver,
};
use log::{info, debug};
use tinybmp::Bmp;

use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

mod display;

fn main() -> Result<(), Box<dyn Error>> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().expect("Failed to take peripherals");
    let mut display = display::create!(peripherals);

    let anya_bytes = include_bytes!("../anya.bmp");
    let anya_bmp =
        Bmp::from_slice(anya_bytes).map_err(|e| format!("Failed to parse BMP: {e:?}"))?;
    let anya_img = Image::new(&anya_bmp, Point::new(0, 0));

    let vlad_bytes = include_bytes!("../vlad.bmp");
    let vlad_bmp =
        Bmp::from_slice(vlad_bytes).map_err(|e| format!("Failed to parse BMP: {e:?}"))?;
    let vlad_img = Image::new(&vlad_bmp, Point::new(0, 0));

    display
        .clear(Rgb565::new(0, 255, 255))
        .map_err(|e| format!("Failed to clear display: {e:?}"))?;

    info!("cleared display, press two buttons to clear it again");

    let left_button = PinDriver::input(peripherals.pins.gpio0)?;
    let right_button = PinDriver::input(peripherals.pins.gpio35)?;

    let mut anya_shown = false;
    let mut vlad_shown = false;

    info!("initialized, waiting for button presses...");

    loop {
        if left_button.is_low() || right_button.is_low() {
            debug!("button pressed, waiting (50ms) for the whole sequence to finish");
            FreeRtos::delay_ms(50);
            debug!("delay finished, checking data");
        }

        match (left_button.is_low(), right_button.is_low()) {
            (true, true) => {
                if !(anya_shown || vlad_shown) {
                    continue;
                }

                info!("Both buttons pressed, clearing display");
                display
                    .clear(Rgb565::new(0, 255, 255))
                    .map_err(|e| format!("Failed to clear display: {e:?}"))?;

                anya_shown = false;
                vlad_shown = false;

                info!("starting 250ms delay to allow the user to release the buttons");
                FreeRtos::delay_ms(250);
                info!("delay finished");
            }
            (true, false) => {
                if anya_shown {
                    continue;
                }

                info!("Left button pressed, drawing Anya");
                anya_img
                    .draw(&mut display)
                    .map_err(|e| format!("Failed to draw image: {e:?}"))?;

                anya_shown = true;
                vlad_shown = false;
            }
            (false, true) => {
                if vlad_shown {
                    continue;
                }

                info!("Right button pressed, drawing Vlad");
                vlad_img
                    .draw(&mut display)
                    .map_err(|e| format!("Failed to draw image: {e:?}"))?;

                anya_shown = false;
                vlad_shown = true;
            }
            _ => {}
        }
    }
}
