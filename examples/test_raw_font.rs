#![feature(iter_array_chunks)]

///// 10x20 pixel monospace font.
// pub const FONT_10X20: MonoFont = crate::mono_font::MonoFont {
//     image: ImageRaw::new(
//         include_bytes!("../../../fonts/raw/ascii/font_10x20.raw"),
//         160,
//     ),
//     glyph_mapping: &crate::mono_font::mapping::ASCII,
//     character_size: crate::geometry::Size::new(10, 20),
//     character_spacing: 0,
//     baseline: 15,
//     underline: crate::mono_font::DecorationDimensions::new(15 + 2, 1),
//     strikethrough: crate::mono_font::DecorationDimensions::new(20 / 2, 1),
// };

use std::{fs::File, io::Read};

use bitvec::{prelude::Msb0, vec::BitVec};

fn print_raw_font(raw_file: &str, width_pixel: u32) {
    let mut raw_buf = Vec::new();
    File::open(raw_file)
        .unwrap()
        .read_to_end(&mut raw_buf)
        .unwrap();

    let bitmap = BitVec::<_, Msb0>::from_vec(raw_buf);
    println!("bitmap len: {}", bitmap.len());

    for i in 0..bitmap.len() as u32 / width_pixel {
        for j in 0..width_pixel {
            if let Some(v) = bitmap.get((i * width_pixel + j) as usize) {
                if *v {
                    print!("*");
                } else {
                    print!("-");
                }
            }
        }
        println!("");
    }
}

fn main() {
    print_raw_font("font_10x20.raw", 160);
    print_raw_font("test.raw", 50);
}
