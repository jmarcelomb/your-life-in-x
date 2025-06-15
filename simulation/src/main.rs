use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use your_life_in_x::{Container, DateTime, TriColor, draw_life_in_months, draw_life_in_years};

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

    let current_date = DateTime {
        year: 2025,
        month: 6,
        day: 15,

        hour: 0,
        minute: 0,
        second: 0,
        ms: 0,
    };
    let _ = draw_life_in_years(
        &mut display,
        &birthday,
        &current_date,
        80,
        &life_in_draw_container,
    );

    let output_settings = OutputSettingsBuilder::new().build();
    Window::new("Your Life in Years", &output_settings).show_static(&display);
    let _ = display.clear(TriColor::White);

    let _ = draw_life_in_months(
        &mut display,
        &birthday,
        &current_date,
        80,
        &life_in_draw_container,
    );
    let output_settings = OutputSettingsBuilder::new().build();
    Window::new("Your Life in Months", &output_settings).show_static(&display);
}
