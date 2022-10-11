use crate::img;
use image::Luma;

pub fn propagate<T>(
    base_img: &mut image::ImageBuffer<Luma<u8>, Vec<u8>>,
    propagation_condition: fn(
        img: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
        curr_pixel: img::PixelT,
        ngb_pixel: img::PixelT,
        aux_structure: T,
    ) -> bool,
    update_func: fn(
        img: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
        curr_pixel: img::PixelT,
        ngb_pixel: img::PixelT,
        aux_structure: T,
    ) -> u8,
    queue: &mut Vec<(u32, u32)>,
    aux_structure: T,
) where
    T: Copy, // check if this makes sense
{
    while queue.len() != 0 {
        let pixel_coords = queue.remove(0); // change this method to a more efficent one
        let pixel_ngbs = img::get_pixel_neighbours(base_img, pixel_coords, img::ConnTypes::Eight);
        let curr_pixel = img::PixelT {
            coords: pixel_coords,
            value: base_img.get_pixel(pixel_coords.0, pixel_coords.1).0[0],
        };

        for ngb_coord in pixel_ngbs {
            let ngb_pixel = img::PixelT {
                coords: ngb_coord,
                value: base_img.get_pixel(ngb_coord.0, ngb_coord.1).0[0],
            };

            if propagation_condition(base_img, curr_pixel, ngb_pixel, aux_structure) {
                let new_value = update_func(base_img, curr_pixel, ngb_pixel, aux_structure);

                let mut ngb = base_img.get_pixel_mut(ngb_coord.0, ngb_coord.1);
                ngb.0[0] = new_value;

                queue.push(ngb_coord);
            }
        }
    }
}
