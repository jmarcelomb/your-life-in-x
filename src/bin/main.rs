#![no_std]
#![no_main]

use display_interface_spi::SPIInterface;
use embedded_graphics::{
    geometry::Point, mono_font::MonoTextStyle, text::Text, text::TextStyle, Drawable,
};
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    gpio::{Input, Io, Level, Output, NO_PIN},
    peripherals::Peripherals,
    prelude::*,
    spi::{master::Spi, SpiMode},
    system::SystemControl,
};
use profont::PROFONT_24_POINT;
use weact_studio_epd::graphics::{Display290TriColor, DisplayTriColor};
use weact_studio_epd::{
    graphics::{buffer_len, DisplayRotation},
    TriColor, WeActStudio290TriColorDriver,
};

// epaper connections:
// DC: 21, RST: 22, BUSY: 23, CS/SS: 15, SCK: 6, MISO: -1, MOSI: 7

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    esp_println::logger::init_logger_from_env();

    log::info!("Intializing SPI Bus...");

    let sclk = io.pins.gpio2;
    let mosi = io.pins.gpio3;
    let cs = io.pins.gpio7;
    let dc = io.pins.gpio6;
    let rst = io.pins.gpio11;
    let busy = io.pins.gpio12;

    let mut spi_bus = Spi::new(peripherals.SPI2, 100.kHz(), SpiMode::Mode0, &clocks).with_pins(
        Some(sclk),
        Some(mosi),
        NO_PIN,
        NO_PIN, // cs is handled by the exclusive device?
    );

    // Convert pins into InputPins and OutputPins
    /*
        CS: OutputPin,
        BUSY: InputPin,
        DC: OutputPin,
        RST: OutputPin,
    */
    let cs = Output::new(cs, Level::High);
    let busy = Input::new(busy, esp_hal::gpio::Pull::Up);
    let dc = Output::new(dc, Level::Low);
    let rst = Output::new(rst, Level::High);

    log::info!("Intializing SPI Device...");
    let spi_device = ExclusiveDevice::new(spi_bus, cs, delay).expect("SPI device initialize error");
    let spi_interface = SPIInterface::new(spi_device, dc);

    // Setup EPD
    log::info!("Intializing EPD...");
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

    loop {
        log::info!("Hello world!");
        delay.delay(500.millis());
    }
}
