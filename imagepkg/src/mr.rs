use std::collections::HashSet;

use crate::{img, iwp};
use image::Luma;

fn update_pixel(
    pixel_coords: (u32, u32),
    mask: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
    marker: &mut image::ImageBuffer<Luma<u8>, Vec<u8>>,
) {
    let pixel_ngbs = img::get_pixel_neighbours(marker, pixel_coords, img::ConnTypes::Eight);

    let pixel = marker.get_pixel(pixel_coords.0, pixel_coords.1);
    let mut greater = pixel.0[0];
    for ngb_coord in &pixel_ngbs {
        let ngb = marker.get_pixel(ngb_coord.0, ngb_coord.1);

        if ngb.0[0] > greater {
            greater = ngb.0[0];
        }
    }

    let mut pixel = marker.get_pixel_mut(pixel_coords.0, pixel_coords.1);
    let mask_pixel = mask.get_pixel(pixel_coords.0, pixel_coords.1);

    if greater > mask_pixel.0[0] {
        greater = mask_pixel.0[0];
    }

    pixel.0[0] = greater;
}

fn get_initial_pixels(
    mask: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
    marker: &mut image::ImageBuffer<Luma<u8>, Vec<u8>>,
) -> Vec<(u32, u32)> {
    let width = mask.width();
    let height = mask.height();
    let mut queue = HashSet::new();

    for i in 0..height {
        for j in 0..width {
            update_pixel((j, i), mask, marker);
        }
    }

    for i in (0..height).rev() {
        for j in (0..width).rev() {
            let pixel_coords = (j, i);

            update_pixel((j, i), mask, marker);
            let pixel_marker = marker.get_pixel(pixel_coords.0, pixel_coords.1);
            let pixel_value = pixel_marker.0[0];

            let pixel_ngbs = img::get_pixel_neighbours(marker, pixel_coords, img::ConnTypes::Eight);

            for ngb_coord in pixel_ngbs {
                let ngb = marker.get_pixel(ngb_coord.0, ngb_coord.1);
                let ngb_mask = mask.get_pixel(ngb_coord.0, ngb_coord.1);

                if (ngb.0[0] < pixel_value) && (ngb.0[0] < ngb_mask.0[0]) {
                    queue.insert(ngb_coord);
                }
            }
        }
    }

    let queue = Vec::from_iter(queue); //check complexity of this operation later
    return queue;
}

fn propagation_condition(
    _marker: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
    curr_pixel: img::PixelT, // maybe transform those in a struct containing the coords and the reference to the pixel itself (or the pixel value)
    ngb_pixel: img::PixelT,
    mask: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
) -> bool {
    let mask_ngb = mask.get_pixel(ngb_pixel.coords.0, ngb_pixel.coords.1);
    if (ngb_pixel.value < curr_pixel.value) && (mask_ngb.0[0] != ngb_pixel.value) {
        return true;
    }

    return false;
}

fn update_func(
    _marker: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
    curr_pixel: img::PixelT, // maybe transform those in a struct containing the coords and the reference to the pixel itself (or the pixel value)
    ngb_pixel: img::PixelT,
    mask: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
) -> u8 {
    let mask_ngb = mask.get_pixel(ngb_pixel.coords.0, ngb_pixel.coords.1);
    return std::cmp::min(curr_pixel.value, mask_ngb.0[0]);
}

pub fn morph_reconstruction(
    mask: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
    marker: &mut image::ImageBuffer<Luma<u8>, Vec<u8>>,
) {
    let mut initial_queue = get_initial_pixels(&mask, marker);
    iwp::propagate(
        marker,
        propagation_condition,
        update_func,
        &mut initial_queue,
        &mask,
    );
}

mod tests {

    #![allow(unused_imports)]

    use crate::examples::*;
    use crate::format;
    use crate::iwp;
    use crate::mr::*;

    #[test]
    fn test_get_initial_pixels() {
        let mask = _gen_example_img();

        let mut marker = _gen_same_value_image(6, 6, 0);
        marker.put_pixel(4, 4, Luma([1]));

        let mut initial = get_initial_pixels(&mask, &mut marker);
        let mut expected = vec![(1, 1), (2, 1), (2, 2), (1, 2)];

        initial.sort();
        expected.sort();

        assert_eq!(initial, expected);
    }

    #[test]
    fn test_propagation_phase() {
        let mask = _gen_big_mask_img();
        let mut marker = _gen_big_marker_img();

        // did not use the get_initial_pixels function here because it does all the job
        let mut initial: Vec<(u32, u32)> = Vec::new();
        for i in 0..10 {
            for j in 0..10 {
                let pixel = marker.get_pixel(i, j);
                if pixel.0[0] != 8 {
                    initial.push((i, j));
                }
            }
        }

        iwp::propagate(
            &mut marker,
            propagation_condition,
            update_func,
            &mut initial,
            &mask,
        );

        assert_eq!(marker, _gen_expected_img());
    }
}
