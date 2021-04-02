
use std::fs::File;
use chrono::Utc;
use image::gif::{GifEncoder, Repeat};
use image::codecs::png::{PngEncoder, CompressionType, FilterType};

pub(crate) fn new() -> GifEncoder<File> {
    let dt = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let file = File::create(format!("gifs/{}.gif", dt)).unwrap();
    let mut encoder = GifEncoder::new_with_speed(file, 30);
    encoder.set_repeat(Repeat::Infinite).unwrap();
    
    encoder
}

pub(crate) fn png_encoder() -> PngEncoder<File> {
    let dt = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let file = File::create(format!("gifs/{}.png", dt)).unwrap();
    PngEncoder::new_with_quality(file, CompressionType::Fast, FilterType::NoFilter)
}