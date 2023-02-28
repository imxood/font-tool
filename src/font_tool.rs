use std::{fs::File, io::Write};

use bitvec::{prelude::Msb0, vec::BitVec};
use fontdue::FontSettings;

use crate::FONT_SIZE;

pub fn make_font(chars: &str, image_raw_file: &str) {
    let font = fontdue::Font::from_bytes(
        include_bytes!("assets/HarmonyOS_Sans_SC_Thin.ttf") as &[u8],
        FontSettings {
            scale: 1.0,
            ..Default::default()
        },
    )
    .unwrap();

    let mut bitmap_data = BitVec::<u8, Msb0>::new();

    let repaired_width = (FONT_SIZE + 7) / 8 * 8;

    bitmap_data.resize(FONT_SIZE * repaired_width * chars.chars().count(), false);

    for (glyph_idx, ch) in chars.chars().enumerate() {
        let (metrics, bitmap) = font.rasterize(ch, FONT_SIZE as f32);
        println!("{metrics:?}");

        for i in 0..metrics.height {
            for j in 0..metrics.width {
                let idx = i * metrics.width + j;
                let pixel_idx = (glyph_idx * FONT_SIZE + i) * repaired_width + j;
                bitmap_data.set(pixel_idx, bitmap[idx] != 0);
            }
        }
    }

    println!("bitmap_data len: {}", bitmap_data.len());

    println!("done");

    println!("");
    println!("");
    println!("");
    println!("");

    for j in 0..FONT_SIZE * chars.chars().count() {
        for i in 0..FONT_SIZE {
            if let Some(v) = bitmap_data.get(j * repaired_width + i) {
                if *v {
                    print!("*");
                } else {
                    print!("-");
                }
            }
        }
        println!("");
    }

    println!("done");

    println!(
        "bitmap_data.len: {}, bitmap_data data len: {}, area size {}",
        bitmap_data.len(),
        bitmap_data.as_raw_slice().len(),
        FONT_SIZE * FONT_SIZE * chars.chars().count()
    );

    let mut raw_image = File::create(image_raw_file).unwrap();
    raw_image.write(bitmap_data.as_raw_slice()).unwrap();
}
