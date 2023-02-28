use embedded_graphics::mono_font::ascii::{FONT_10X20, FONT_6X9};
use embedded_graphics::mono_font::{MonoFont, MonoTextStyle};
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, PrimitiveStyle, Triangle};
use embedded_graphics::{
    mono_font::MonoTextStyleBuilder,
    pixelcolor::BinaryColor,
    prelude::{Point, Size},
    primitives::{Primitive, PrimitiveStyleBuilder, Rectangle},
    text::{Text, TextStyle},
    Drawable,
};
use embedded_layout::layout::linear::LinearLayout;
use embedded_layout::prelude::*;

pub fn ui(display: &mut impl DrawTarget<Color = BinaryColor>, custom_font: MonoFont) {
    // let custom_font_style = MonoTextStyleBuilder::new()
    //     .font(&custom_font)
    //     .text_color(BinaryColor::On)
    //     .background_color(BinaryColor::Off)
    //     .build();

    // let font_style = MonoTextStyleBuilder::new()
    //     .font(&FONT_10X20)
    //     .text_color(BinaryColor::On)
    //     .background_color(BinaryColor::Off)
    //     .build();

    // let line_style = PrimitiveStyleBuilder::new()
    //     .stroke_width(1)
    //     .stroke_color(BinaryColor::On)
    //     .build();

    // Rectangle::new(Point::new(1, 1), Size::new(127, 63))
    //     .into_styled(line_style)
    //     .draw(display)
    //     .ok();

    // Text::with_text_style("160℃", Point::new(3, 23), font_style, TextStyle::default())
    //     .draw(display)
    //     .ok();

    // Text::with_text_style(
    //     "加热",
    //     Point::new(3, 25),
    //     custom_font_style,
    //     TextStyle::default(),
    // )
    // .draw(display)
    // .ok();

    // let text = Text::new("embedded-layout", Point::zero(), font_style);

    // Create a Rectangle from the display's dimensions
    let display_area = display.bounding_box();

    // Style objects
    let text_style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);

    let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    let thick_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 3);
    let fill_on = PrimitiveStyle::with_fill(BinaryColor::On);
    let fill_off = PrimitiveStyle::with_fill(BinaryColor::Off);

    // Primitives to be displayed
    let triangle = Triangle::new(Point::new(0, 0), Point::new(12, 0), Point::new(6, 12))
        .into_styled(thin_stroke);

    let circle = Circle::new(Point::zero(), 11).into_styled(thick_stroke);
    let circle2 = Circle::new(Point::zero(), 15).into_styled(fill_on);
    let triangle2 =
        Triangle::new(Point::new(0, 0), Point::new(10, 0), Point::new(5, 8)).into_styled(fill_off);
    let text = Text::new("embedded-layout", Point::zero(), text_style);

    // The layout
    LinearLayout::vertical(
        Chain::new(text)
            .append(LinearLayout::horizontal(Chain::new(triangle).append(circle)).arrange())
            .append(
                Chain::new(triangle2.align_to(&circle2, horizontal::Center, vertical::Top))
                    .append(circle2),
            ),
    )
    .with_alignment(horizontal::Center)
    .arrange()
    .align_to(&display_area, horizontal::Center, vertical::Center)
    .draw(display)
    .ok();
}
