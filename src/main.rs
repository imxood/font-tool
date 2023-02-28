use app::App;
use embedded_graphics::{
    image::ImageRaw,
    mono_font::{mapping::StrGlyphMapping, DecorationDimensions, MonoFont},
    prelude::Size,
};
use font_tool::make_font;
use ui::ui;
use window::WindowFrame;

mod app;
mod font;
mod font_tool;
mod ui;
mod window;

use eframe::egui;

const FONT_SIZE: usize = 20;

fn main() {
    let raw_image = "test.raw";
    let font_chars = "_恒温正在加热未1234567890℃";
    // make_font(font_chars, raw_image);

    let options = eframe::NativeOptions {
        maximized: true,
        min_window_size: Some(egui::vec2(400., 300.)),
        ..Default::default()
    };

    eframe::run_native(
        "generate embedded_graph fonts",
        options,
        Box::new(|ctx| {
            let ctx = ctx.egui_ctx.clone();
            ctx.set_visuals(egui::Visuals::light());
            Box::new(App::default())
        }),
    )
    .unwrap();

    let raw_image = std::fs::read(raw_image).unwrap();

    let custom_font = MonoFont {
        // data 表示 image raw 图像 的字节数
        // width 表示 image raw 图像宽度上的像素数 pixels, 由于存储的格式, 每个像素保存n个bits, 所以一行的字节数就是 (width * n + 7) / 8, bits数 进一取整
        //
        image: ImageRaw::new_binary(&raw_image, FONT_SIZE as u32),
        glyph_mapping: &StrGlyphMapping::new(font_chars, 0),
        // 一个字体 宽和高 上的像素
        character_size: Size::new(FONT_SIZE as u32, FONT_SIZE as u32),
        character_spacing: 0,
        baseline: 0,
        underline: DecorationDimensions::default_underline(0),
        strikethrough: DecorationDimensions::default_strikethrough(FONT_SIZE as u32),
    };

    let mut display = WindowFrame::new(128, 64);

    ui(&mut display, custom_font);

    display.update();
}
