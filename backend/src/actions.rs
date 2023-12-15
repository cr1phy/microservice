use std::io::Cursor;
use image::io::Reader as ImageReader;

fn create_poster() {
    let img = ImageReader::open("./static/bg.png")?.decode()?;
    println!(img);
}