use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use your_life_in_x::{TriColor, draw};

fn main() {
    // Use with_default_color instead of new
    let mut display =
        SimulatorDisplay::<TriColor>::with_default_color(Size::new(296, 128), TriColor::White);
    let _ = draw(&mut display);

    let output_settings = OutputSettingsBuilder::new().build();
    Window::new("Hello World", &output_settings).show_static(&display);
}
