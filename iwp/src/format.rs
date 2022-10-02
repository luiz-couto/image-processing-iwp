use image::Luma;

pub fn print_image_by_row(img: &image::ImageBuffer<Luma<u8>, Vec<u8>>) {
    for row in img.rows() {
        print!("[");
        for pixel in row {
            print!("{:?} ", pixel.0[0]);
        }
        print!("]\n");
    }
}
