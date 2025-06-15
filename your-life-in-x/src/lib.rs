#![no_std]

use core::result::Result;

use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_6X9},
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, Triangle},
    text::Text,
};

pub use weact_studio_epd::TriColor;

#[derive(Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DateTime {
    // date
    pub year: u16,
    pub month: u8,
    pub day: u8,

    // time
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub ms: u16,
}

// impl DateTime {
//     /// Calculates ms for the day
//     pub fn to_day_unixtime(&self) -> u64 {
//         self.day.checked_sub(1).expect("failed to calc day - 1") as u64 * MS_IN_DAY
//             + self.hour as u64 * MS_IN_HOUR
//             + self.minute as u64 * MS_IN_MIN
//             + self.second as u64 * MS_IN_SEC
//             + self.ms as u64
//     }
// }

/// Represents a rectangular container with an origin point, width, and height.
/// The origin point is the top-left corner of the container.
pub struct Container {
    pub point: Point,
    pub width: u16,
    pub height: u16,
}

/// Draws a filled losango (diamond shape) with a border on the provided display.
///
/// # Arguments
///
/// * `display` - A mutable reference to a display implementing the `DrawTarget` trait.
/// * `center` - The center point of the losango.
/// * `size` - The size of the losango.
/// * `fill_color` - The color to fill the losango with.
///
/// # Returns
///
/// * `Result<(), T::Error>` - Returns `Ok(())` if the drawing is successful, otherwise returns an error.
pub fn draw_filled_losango_with_border<T>(
    display: &mut T,
    center: Point,
    size: i32,
    fill_color: TriColor,
) -> Result<(), T::Error>
where
    T: DrawTarget<Color = TriColor>,
{
    let half_size = size / 2;

    let top = Point::new(center.x, center.y - half_size);
    let right = Point::new(center.x + half_size, center.y);
    let bottom = Point::new(center.x, center.y + half_size);
    let left = Point::new(center.x - half_size, center.y);

    // Define the filled style
    let fill_style = PrimitiveStyleBuilder::new().fill_color(fill_color).build();

    // Define the black outline style
    let outline_style = PrimitiveStyle::with_stroke(TriColor::Black, 1);

    // Fill the losango using two triangles
    Triangle::new(top, right, bottom)
        .into_styled(fill_style)
        .draw(display)?;
    Triangle::new(bottom, left, top)
        .into_styled(fill_style)
        .draw(display)?;

    // Draw the black outline
    Line::new(top, right)
        .into_styled(outline_style)
        .draw(display)?;
    Line::new(right, bottom)
        .into_styled(outline_style)
        .draw(display)?;
    Line::new(bottom, left)
        .into_styled(outline_style)
        .draw(display)?;
    Line::new(left, top)
        .into_styled(outline_style)
        .draw(display)?;

    Ok(())
}

/// Draws a life calendar in years on the provided display.
///
/// # Arguments
///
/// * `display` - A mutable reference to a display implementing the `DrawTarget` trait.
/// * `birthday` - The birth date of the person.
/// * `life_expectancy` - The life expectancy in years.
/// * `current_year` - The current year.
/// * `container` - The container defining the area to draw the calendar.
///
/// # Returns
///
/// * `Result<(), T::Error>` - Returns `Ok(())` if the drawing is successful, otherwise returns an error.
pub fn draw_life_in_years<T>(
    display: &mut T,
    birthday: &DateTime,
    current_date: &DateTime,
    life_expectancy: u32,
    container: &Container,
) -> Result<(), T::Error>
where
    T: DrawTarget<Color = TriColor>,
{
    let birth_year = birthday.year;
    let mut age = current_date.year.saturating_sub(birth_year); // Years lived

    // Adjust age based on month and day
    if current_date.month < birthday.month
        || (current_date.month == birthday.month && current_date.day < birthday.day)
    {
        age = age.saturating_sub(1);
    }

    // Define padding and losango size
    let padding = 5;
    let losango_per_row = (life_expectancy as usize).max(2) / 2;
    let losango_size = (container.width.min(container.height)) as usize / losango_per_row;
    let losango_size = losango_size.max(14) as i32; // Ensure it's not too small

    // Compute number of columns and rows
    let cols = container.width as i32 / (losango_size + padding);
    let rows = (life_expectancy as i32 + cols - 1) / cols; // Round up

    // Compute the total grid size
    let total_width = cols * (losango_size + padding);
    let total_height = rows * (losango_size + padding);

    // Compute centering offsets
    let offset_x = ((container.width as i32) - total_width) / 2;
    let offset_y = ((container.height as i32) - total_height) / 2;

    let start_x = container.point.x + offset_x;
    let start_y = container.point.y + offset_y;

    for i in 0..life_expectancy {
        let row = i as i32 / cols;
        let col = i as i32 % cols;

        let center_x = start_x + col * (losango_size + padding) + losango_size / 2;
        let center_y = start_y + row * (losango_size + padding) + losango_size / 2;

        let color = if i < age.into() {
            TriColor::Red // Past years
        } else {
            TriColor::Black // Future years
        };

        draw_filled_losango_with_border(
            display,
            Point::new(center_x, center_y),
            losango_size,
            color,
        )?;
    }

    Ok(())
}

/// Draws a life calendar in months on the provided display using circles.
///
/// # Arguments
///
/// * `display` - A mutable reference to a display implementing the `DrawTarget` trait.
/// * `birthday` - The birth date of the person.
/// * `current_date` - The current date.
/// * `life_expectancy` - The life expectancy in years.
/// * `container` - The container defining the area to draw the calendar.
///
/// # Returns
///
/// * `Result<(), T::Error>` - Returns `Ok(())` if the drawing is successful, otherwise returns an error.
pub fn draw_life_in_months<T>(
    display: &mut T,
    birthday: &DateTime,
    current_date: &DateTime,
    life_expectancy: u32,
    container: &Container,
) -> Result<(), T::Error>
where
    T: DrawTarget<Color = TriColor>,
{
    let months_lived = {
        let mut months = (current_date.year - birthday.year) as u32 * 12;
        if current_date.month < birthday.month
            || (current_date.month == birthday.month && current_date.day < birthday.day)
        {
            months = months.saturating_sub(1);
        }
        months + current_date.month as u32 - birthday.month as u32
    };

    let total_months = life_expectancy * 12;

    let padding = 2;
    let max_per_row = (total_months as f32).sqrt().ceil() as u32;
    let circle_diameter =
        (u32::from(container.width.min(container.height)) / max_per_row.max(1)) as i32;

    let circle_diameter = circle_diameter.max(2); // Minimum size
    let radius = circle_diameter / 2;

    let cols = container.width as i32 / (circle_diameter + padding);
    let rows = (total_months as i32 + cols - 1) / cols;

    let total_width = cols * (circle_diameter + padding);
    let total_height = rows * (circle_diameter + padding);

    let offset_x = (container.width as i32 - total_width) / 2;
    let offset_y = (container.height as i32 - total_height) / 2;

    let start_x = container.point.x + offset_x;
    let start_y = container.point.y + offset_y;

    let filled_style = PrimitiveStyleBuilder::new()
        .fill_color(TriColor::Red)
        .stroke_color(TriColor::Black)
        .stroke_width(1)
        .build();

    let outline_style = PrimitiveStyle::with_stroke(TriColor::Black, 1);

    for i in 0..total_months {
        let row = i as i32 / cols;
        let col = i as i32 % cols;

        let center_x = start_x + col * (circle_diameter + padding) + radius;
        let center_y = start_y + row * (circle_diameter + padding) + radius;

        let style = if i < months_lived {
            filled_style
        } else {
            outline_style
        };

        Circle::new(Point::new(center_x, center_y), circle_diameter as u32)
            .into_styled(style)
            .draw(display)?;
    }

    Ok(())
}
