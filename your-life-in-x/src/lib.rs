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

/// Draws a life calendar on the provided display.
///
/// # Arguments
///
/// * `display` - A mutable reference to a display implementing the `DrawTarget` trait.
/// * `birthday` - The birth date of the person.
/// * `life_expectancy` - The life expectancy in years.
/// * `current_year` - The current year.
/// * `grid_size` - The size of the grid (columns, rows).
/// * `cell_size` - The size of each cell in the grid.
///
/// # Returns
///
/// * `Result<(), T::Error>` - Returns `Ok(())` if the drawing is successful, otherwise returns an error.
pub fn draw_life_calendar<T>(
    display: &mut T,
    birthday: &DateTime,
    life_expectancy: u32,
    current_year: u16,
    grid_size: (u32, u32),
    cell_size: u32,
) -> Result<(), T::Error>
where
    T: DrawTarget<Color = TriColor>,
{
    let (cols, rows) = grid_size; // (52 weeks, 90 years) or (12 months, 90 years)

    let past_style = PrimitiveStyle::with_fill(TriColor::Red);
    let future_style = PrimitiveStyle::with_stroke(TriColor::Black, 1);

    for row in 0..rows {
        for col in 0..cols {
            let x = col * (cell_size + 2);
            let y = row * (cell_size + 2);

            let age = row as u16;
            let week_number = col as u16;
            let year = birthday.year + age;

            let style = if year < current_year {
                past_style
            } else {
                future_style
            };

            Rectangle::new(
                Point::new(x as i32, y as i32),
                Size::new(cell_size, cell_size),
            )
            .into_styled(style)
            .draw(display)?;
        }
    }

    Ok(())
}
