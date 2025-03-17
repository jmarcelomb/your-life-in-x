use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use your_life_in_x::{
    Container, DateTime, TriColor, draw_filled_losango_with_border, draw_life_calendar,
    draw_life_in_years,
};

fn main() {
    // Use with_default_color instead of new
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
    let mut display = SimulatorDisplay::<TriColor>::with_default_color(
        Size::new(width as u32, height as u32),
        TriColor::White,
    );
    // let _ = draw_filled_losango_with_border(&mut display, Point::new(100, 50), 40, TriColor::Red);
    let _ = draw_life_in_years(&mut display, &birthday, 90, 2025, &life_in_draw_container);
    // let _ = draw_life_calendar(&mut display, &birthday, 90, 2025, (52, 90), 3);

    let output_settings = OutputSettingsBuilder::new().build();
    Window::new("Your Life in Weeks", &output_settings).show_static(&display);
}
