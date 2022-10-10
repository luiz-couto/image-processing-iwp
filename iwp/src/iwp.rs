use image::Luma;

use crate::mr::{get_pixel_neighbours, ConnTypes};

pub fn iwp<T>(
    base_img: &mut image::ImageBuffer<Luma<u8>, Vec<u8>>,
    propagation_condition: fn(
        img: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
        curr_pixel_coords: (u32, u32),
        ngb_pixel_coords: (u32, u32),
        aux_structure: T,
    ) -> bool,
    update_func: fn(
        img: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
        curr_pixel_coords: (u32, u32),
        ngb_pixel_coords: (u32, u32),
        aux_structure: T,
    ) -> u8,
    queue: &mut Vec<(u32, u32)>,
    aux_structure: T,
) where
    T: Copy, // check if this makes sense
{
    while queue.len() != 0 {
        let pixel_coords = queue.remove(0); // change this method to a more efficent one
        let pixel_ngbs = get_pixel_neighbours(base_img, pixel_coords, ConnTypes::Eight);

        for ngb_coord in pixel_ngbs {
            if propagation_condition(base_img, pixel_coords, ngb_coord, aux_structure) {
                let new_value = update_func(base_img, pixel_coords, ngb_coord, aux_structure);

                let mut ngb = base_img.get_pixel_mut(ngb_coord.0, ngb_coord.1);
                ngb.0[0] = new_value;

                queue.push(ngb_coord);
            }
        }
    }
}
