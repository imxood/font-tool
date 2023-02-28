use embedded_graphics::{
    image::ImageRaw,
    mono_font::{mapping::StrGlyphMapping, MonoFont, DecorationDimensions},
    prelude::Size,
};

pub const CUSTOM_FONT: MonoFont = MonoFont {
    image: ImageRaw::new_binary(include_bytes!("assets/test.raw"), 31),
    glyph_mapping: &StrGlyphMapping::new("_", 0),
    character_size: Size::new(31, 31),
    character_spacing: 4,
    baseline: 0,
    underline: DecorationDimensions::default_underline(40),
    strikethrough: DecorationDimensions::default_strikethrough(80),
};
