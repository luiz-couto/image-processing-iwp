use image;
use image::io::Reader as ImageReader;
use std::error::Error;

mod format;
mod iwp;
mod mr;

fn main() -> Result<(), Box<dyn Error>> {
    let img_mask = ImageReader::open("mask.png")?.decode()?;
    let mask = img_mask.to_luma8();

    let img_marker = ImageReader::open("marker.png")?.decode()?;
    let mut marker = img_marker.to_luma8();

    let dimensions = mask.dimensions();
    println!("dimensions: {:?}", dimensions);

    let mut i = mr::get_initial_pixels(&mask, &mut marker);
    iwp::iwp(
        &mut marker,
        mr::propagation_condition,
        mr::update_func,
        &mut i,
        &mask,
    );
    println!("{:?}", i);
    marker.save("result.png")?;

    Ok(())
}
