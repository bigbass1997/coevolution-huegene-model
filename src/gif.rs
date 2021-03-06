
use std::fs::File;
use image::codecs::gif::{GifEncoder, Repeat};
use image::codecs::png::{PngEncoder, CompressionType, FilterType};

pub(crate) fn new() -> GifEncoder<File> {
    let dt = time::OffsetDateTime::now_utc().format(&time::format_description::parse("[year]-[month]-[day]_[hour]-[minute]-[second]").unwrap()).unwrap_or("broken".to_owned());
    let file = File::create(format!("gifs/{}.gif", dt)).unwrap();
    let mut encoder = GifEncoder::new_with_speed(file, 30);
    encoder.set_repeat(Repeat::Infinite).unwrap();
    
    encoder
}

pub(crate) fn png_encoder() -> PngEncoder<File> {
    let dt = time::OffsetDateTime::now_utc().format(&time::format_description::parse("[year]-[month]-[day]_[hour]-[minute]-[second]").unwrap()).unwrap_or("broken".to_owned());
    let file = File::create(format!("gifs/{}.png", dt)).unwrap();
    PngEncoder::new_with_quality(file, CompressionType::Fast, FilterType::NoFilter)
}

/*pub struct PngSequencer {
    pub directory: String,
    count: u16,
}
impl PngSequencer {
    pub fn new() -> Self {
    let dt = Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        std::fs::DirBuilder::new().recursive(true).create(format!("gifs-raw/{}/", dt)).unwrap();
        
        
    }
}

pub fn png_sequencer() -> PngSequencer {
    
}*/