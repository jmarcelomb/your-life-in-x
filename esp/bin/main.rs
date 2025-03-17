#![no_std]
#![no_main]

use display_interface_spi::SPIInterface;
use embedded_graphics::prelude::*;
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Input, Level, Output},
    main,
    spi::{
        Mode,
        master::{Config, Spi},
    },
    time::RateExtU32,
};
use weact_studio_epd::graphics::Display290TriColor;
use weact_studio_epd::{WeActStudio290TriColorDriver, graphics::DisplayRotation};
use your_life_in_x::{Container, DateTime};
// epaper connections:
// DC: 21, RST: 22, BUSY: 23, CS/SS: 15, SCK: 6, MISO: -1, MOSI: 7

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();

    log::info!("Initializing SPI Bus...");

    let sclk = peripherals.GPIO2;
    let mosi = peripherals.GPIO3;
    let cs = peripherals.GPIO7;
    let dc = peripherals.GPIO6;
    let rst = peripherals.GPIO11;
    let busy = peripherals.GPIO12;

    let cs = Output::new(cs, Level::High);
    let busy = Input::new(busy, esp_hal::gpio::Pull::Up);
    let dc = Output::new(dc, Level::Low);
    let rst = Output::new(rst, Level::High);

    let spi = Spi::new(
        peripherals.SPI2,
        Config::default()
            .with_frequency(100.kHz())
            .with_mode(Mode::_0),
    )
    .unwrap()
    .with_sck(sclk)
    .with_mosi(mosi);

    log::info!("Initializing SPI Device...");
    let spi_device = ExclusiveDevice::new(spi, cs, delay).expect("SPI device initialize error");
    let spi_interface = SPIInterface::new(spi_device, dc);

    // Setup EPD
    log::info!("Initializing EPD...");
    let mut driver = WeActStudio290TriColorDriver::new(spi_interface, busy, rst, delay);
    let mut display = Display290TriColor::new();
    // display.set_rotation(DisplayRotation::Rotate90);
    driver.init().unwrap();
    log::info!("Display initialized.");

    let width = 128;
    let height = 296;
    let life_in_draw_container = Container {
        point: Point { x: 0, y: 0 },
        width,
        height,
    };
    let birthday = DateTime {
        year: 1998,
        month: 9,
        day: 10,

        hour: 11,
        minute: 30,
        second: 0,
        ms: 0,
    };

    let _ = your_life_in_x::draw_life_in_years(
        &mut display,
        &birthday,
        90,
        2025,
        &life_in_draw_container,
    );

    // Update display
    driver.full_update(&display).unwrap();

    let mut i = 0;
    loop {
        log::info!("Hello world! {i}");
        delay.delay_millis(1000);
        i += 1;
    }
}
