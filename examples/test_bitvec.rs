use bitvec::{prelude::Msb0, vec::BitVec};

fn main() {
    let test_data = [255, 0, 80, 80, 80, 80, 80, 80, 80, 0, 80];

    let mut bitmap = BitVec::<u8, Msb0>::new();
    bitmap.push(true);
    bitmap.push(false);
    println!("bitmap: {}", bitmap.as_bitslice());
    println!("bitmap raw_slice: {:?}", bitmap.as_raw_slice());

    for (i, v) in test_data.iter().enumerate() {
        if i / 8 + 1 > bitmap.as_raw_slice().len() {
            println!("resize {}", i / 8 + 1);
            bitmap.resize(i / 8 + 1, false);
        }

        bitmap.insert(i, *v != 0);
    }

    println!("bitmap: {}", bitmap.as_bitslice());
    println!("bitmap raw_slice: {:?}", bitmap.as_raw_slice());

    println!("{}", "你好".chars().count());
}
