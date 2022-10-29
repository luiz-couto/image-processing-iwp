use image::Luma;

pub fn rmse_between_imgs(
    img_a: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
    img_b: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
) -> f64 {
    let width = img_a.width();
    let height = img_a.height();
    let n = width as f64 * height as f64;

    let mut sum = 0.0;

    for i in (0..height).rev() {
        for j in (0..width).rev() {
            let pixel_coords = (j, i);
            let pixel_a_value = img_a.get_pixel(pixel_coords.0, pixel_coords.1).0[0] as f64;
            let pixel_b_value = img_b.get_pixel(pixel_coords.0, pixel_coords.1).0[0] as f64;

            sum += (pixel_a_value - pixel_b_value).powi(2);
        }
    }

    let mse = sum / n;
    let rmse = mse.sqrt();

    return rmse;
}
