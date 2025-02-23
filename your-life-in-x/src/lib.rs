#![no_std]

use core::result::Result;

use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_6X9},
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle, Rectangle},
    text::Text,
};

pub use weact_studio_epd::TriColor;

/// Draws shapes and text on the provided display.
///
/// # Arguments
/// * display - A mutable reference to a display implementing the DrawTarget trait.
pub fn draw<T>(display: &mut T) -> Result<(), T::Error>
where
    T: DrawTarget<Color = TriColor>,
{
    let line_style = PrimitiveStyle::with_stroke(TriColor::Black, 1);
    let red_line_style = PrimitiveStyle::with_stroke(TriColor::Red, 1);
    let text_style = MonoTextStyle::new(&FONT_6X9, TriColor::Black);
    let red_text_style = MonoTextStyle::new(&FONT_6X9, TriColor::Red);

    // Draw a black circle
    Circle::new(Point::new(72, 8), 48)
        .into_styled(line_style)
        .draw(display)?;

    // Draw a red horizontal line
    Line::new(Point::new(48, 16), Point::new(8, 16))
        .into_styled(red_line_style)
        .draw(display)?;

    // Draw a diagonal line in black
    Line::new(Point::new(48, 16), Point::new(64, 32))
        .into_styled(line_style)
        .draw(display)?;

    // Draw a red rectangle
    Rectangle::new(Point::new(79, 15), Size::new(34, 34))
        .into_styled(red_line_style)
        .draw(display)?;

    // Draw text in black
    Text::new("Hello World!", Point::new(5, 5), text_style).draw(display)?;

    // Draw text in red
    Text::new("Hello Red!", Point::new(5, 20), red_text_style).draw(display)?;

    Ok(())
}
