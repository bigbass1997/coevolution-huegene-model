
use std::fs::File;
use chrono::Utc;
use image::gif::{GifEncoder, Repeat};

pub(crate) fn new() -> GifEncoder<File> {
    let dt = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let file = File::create(format!("gifs/{}.gif", dt)).unwrap();
    let mut encoder = GifEncoder::new_with_speed(file, 1);
    encoder.set_repeat(Repeat::Infinite).unwrap();
    
    encoder
}