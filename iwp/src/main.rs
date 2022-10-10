use image;
use image::io::Reader as ImageReader;
use std::error::Error;

mod format;
mod iwp;
mod mr;

fn main() -> Result<(), Box<dyn Error>> {
    let img = ImageReader::open("fish.png")?.decode()?;

    let mask = img.to_luma8();
    let mut marker = img.to_luma8();

    let dimensions = mask.dimensions();
    println!("dimensions: {:?}", dimensions);

    let i = mr::get_initial_pixels(&mask, &mut marker);
    println!("{:?}", i.len());

    Ok(())
}
