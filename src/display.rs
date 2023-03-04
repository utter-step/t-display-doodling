use std::error::Error;

use display_interface_spi::SPIInterfaceNoCS;
use esp_idf_hal::{
    delay::Ets,
    gpio::{
        AnyIOPin,
        PinDriver,
        Gpio18,
        Gpio19,
        Gpio23,
        Gpio16,
        Gpio5,
        InputOutput,
    },
    spi::{
        config::Config,
        Dma,
        SpiDeviceDriver,
        SpiDriver,
        SPI2,
    },
    units::FromValueType,
};
use mipidsi::{Builder};

type Display<'d> = mipidsi::Display<
    SPIInterfaceNoCS<
        SpiDeviceDriver<
            'd,
            SpiDriver<'d>,
        >,
        PinDriver<'d, Gpio16, InputOutput>,
    >,
    mipidsi::models::ST7789,
    PinDriver<'d, Gpio23, InputOutput>,
>;

pub(crate) fn create_st7789_pico(
    spi: SPI2,
    sclk: Gpio18,
    sdo: Gpio19,
    rst: Gpio23,
    dc: Gpio16,
    cs: Gpio5,
) -> Result<Display<'static>, Box<dyn Error>> {
    let spi_driver = SpiDriver::new(
        spi,
        sclk,
        sdo,
        None::<AnyIOPin>,
        Dma::Channel1(240 * 135 * 2 + 8),
    )
    .map_err(|e| format!("Failed to initialize SPI driver: {e:?}"))?;

    let spi_config = Config::new()
        .baudrate(80.MHz().into())
        .write_only(true);

    let spi = SpiDeviceDriver::new(spi_driver, Some(cs), &spi_config)
        .map_err(|e| format!("Failed to initialize SPI device: {e:?}"))?;

    let rst = PinDriver::input_output_od(rst)?;
    let dc = PinDriver::input_output_od(dc)?;
    let mut delay = Ets;

    let di = SPIInterfaceNoCS::new(spi, dc);

    let mut display = Builder::st7789_pico1(di)
        .init(&mut delay, Some(rst))
        .map_err(|e| format!("Failed to initialize display: {e:?}"))?;

    display.set_tearing_effect(mipidsi::TearingEffect::HorizontalAndVertical)
        .map_err(|e| format!("Failed to set tearing effect: {e:?}"))?;

    Ok(display)
}

macro_rules! create {
    ($peripherals: expr) => {{
        let spi = $peripherals.spi2;
        let sclk = $peripherals.pins.gpio18;
        let sdo = $peripherals.pins.gpio19;

        let rst = $peripherals.pins.gpio23;
        let dc = $peripherals.pins.gpio16;

        let cs = $peripherals.pins.gpio5;

        crate::display::create_st7789_pico(spi, sclk, sdo, rst, dc, cs)?
    }};
}

pub(crate) use create;
