use image;
use image::io::Reader as ImageReader;
use imagepkg;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let img_mask = ImageReader::open("mask.png")?.decode()?;
    let mask = img_mask.to_luma8();

    let img_marker = ImageReader::open("marker.png")?.decode()?;
    let mut marker = img_marker.to_luma8();

    let dimensions = mask.dimensions();
    println!("dimensions: {:?}", dimensions);

    imagepkg::morph_reconstruction(&mask, &mut marker);

    marker.save("result.png")?;

    Ok(())
}
