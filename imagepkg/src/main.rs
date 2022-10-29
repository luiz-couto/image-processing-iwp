use image;
use image::io::Reader as ImageReader;
use imagepkg::{self, convert_to_binary};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let img = ImageReader::open("bin_img.png")?.decode()?;
    let img = img.to_luma8();

    let mut bin_img = convert_to_binary(&img);

    //print_image_by_row(&bin_img);

    let res = imagepkg::dist_transform(&mut bin_img, imagepkg::DistTypes::Euclidean);

    // let img_marker = ImageReader::open("marker.png")?.decode()?;
    // let mut marker = img_marker.to_luma8();

    // let dimensions = mask.dimensions();
    // println!("dimensions: {:?}", dimensions);

    // imagepkg::morph_reconstruction(&mut mask, &mut marker);

    res.save("result_bin_2.png")?;

    Ok(())
}
