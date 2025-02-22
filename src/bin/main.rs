#![no_std]
#![no_main]

use display_interface_spi::SPIInterface;
use embedded_graphics::{
    geometry::Point, mono_font::MonoTextStyle, text::Text, text::TextStyle, Drawable,
};
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Input, Level, Output},
    main,
    spi::{
        master::{Config, Spi},
        Mode,
    },
    time::RateExtU32,
};
use profont::PROFONT_24_POINT;
use weact_studio_epd::graphics::Display290TriColor;
use weact_studio_epd::{graphics::DisplayRotation, TriColor, WeActStudio290TriColorDriver};

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
    display.set_rotation(DisplayRotation::Rotate90);
    driver.init().unwrap();
    log::info!("Display initialized.");

    // Write hello world
    let black_style = MonoTextStyle::new(&PROFONT_24_POINT, TriColor::Black);
    let red_style = MonoTextStyle::new(&PROFONT_24_POINT, TriColor::Red);
    let _ = Text::with_text_style(
        "Hello World!",
        Point::new(8, 40),
        black_style,
        TextStyle::default(),
    )
    .draw(&mut display);
    let _ = Text::with_text_style(
        "Hello World!",
        Point::new(8, 80),
        red_style,
        TextStyle::default(),
    )
    .draw(&mut display);

    // Update display
    driver.full_update(&display).unwrap();

    let mut i = 0;
    loop {
        log::info!("Hello world! {i}");
        delay.delay_millis(1000);
        i += 1;
    }
}
