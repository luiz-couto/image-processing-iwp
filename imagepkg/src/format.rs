use image::{Luma, Primitive};

pub fn print_image_by_row<P: Primitive + std::fmt::Debug>(
    img: &image::ImageBuffer<Luma<P>, Vec<P>>,
) {
    for row in img.rows() {
        print!("[");
        for pixel in row {
            print!("{:?} ", pixel.0[0]);
        }
        print!("]\n");
    }
    println!();
}
