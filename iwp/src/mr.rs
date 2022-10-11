use std::collections::HashSet;

use crate::{format, iwp};
use image::Luma;

fn update_pixel(
    pixel_coords: (u32, u32),
    mask: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
    marker: &mut image::ImageBuffer<Luma<u8>, Vec<u8>>,
) {
    let pixel_ngbs = iwp::get_pixel_neighbours(marker, pixel_coords, iwp::ConnTypes::Eight);

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

pub fn get_initial_pixels(
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

            let pixel_ngbs = iwp::get_pixel_neighbours(marker, pixel_coords, iwp::ConnTypes::Eight);

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

pub fn propagation_condition(
    marker: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
    curr_pixel_coords: (u32, u32), // maybe transform those in a struct containing the coords and the reference to the pixel itself (or the pixel value)
    ngb_pixel_coords: (u32, u32),
    mask: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
) -> bool {
    let pixel = marker.get_pixel(curr_pixel_coords.0, curr_pixel_coords.1);
    let ngb = marker.get_pixel(ngb_pixel_coords.0, ngb_pixel_coords.1);
    let mask_ngb = mask.get_pixel(ngb_pixel_coords.0, ngb_pixel_coords.1);

    if (ngb.0[0] < pixel.0[0]) && (mask_ngb.0[0] != ngb.0[0]) {
        return true;
    }

    return false;
}

pub fn update_func(
    marker: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
    curr_pixel_coords: (u32, u32),
    ngb_pixel_coords: (u32, u32),
    mask: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
) -> u8 {
    let pixel = marker.get_pixel(curr_pixel_coords.0, curr_pixel_coords.1);
    let mask_ngb = mask.get_pixel(ngb_pixel_coords.0, ngb_pixel_coords.1);

    return std::cmp::min(pixel.0[0], mask_ngb.0[0]);
}

mod tests {
    use image::{GrayImage, ImageBuffer};

    use crate::format::*;
    use crate::iwp;
    use crate::mr::*;

    fn _gen_same_value_image(width: u32, height: u32, value: u8) -> ImageBuffer<Luma<u8>, Vec<u8>> {
        let mut img = GrayImage::new(width, height);
        for i in 0..width {
            for j in 0..height {
                img.put_pixel(i, j, Luma([value]))
            }
        }

        return img;
    }

    /*
    Gens the 6 x 6 image below
    0 0 0 0 0 0
    0 1 1 0 0 0
    0 1 1 0 0 0
    0 0 0 1 1 0
    0 0 0 1 1 0
    0 0 0 0 0 0
    */
    fn _gen_example_img() -> ImageBuffer<Luma<u8>, Vec<u8>> {
        let mut img = _gen_same_value_image(6, 6, 0);

        img.put_pixel(1, 1, Luma([1]));
        img.put_pixel(1, 2, Luma([1]));
        img.put_pixel(2, 1, Luma([1]));
        img.put_pixel(2, 2, Luma([1]));
        img.put_pixel(3, 3, Luma([1]));
        img.put_pixel(3, 4, Luma([1]));
        img.put_pixel(4, 3, Luma([1]));
        img.put_pixel(4, 4, Luma([1]));

        return img;
    }

    /*
    Gens the 10 x 10 image below
    08 08 08 08 08 08 08 08 08 08
    08 12 12 12 08 08 09 08 09 08
    08 12 12 12 08 08 08 09 08 08
    08 12 12 12 08 08 09 08 09 08
    08 08 08 08 08 08 08 08 08 08
    08 09 08 08 08 16 16 16 08 08
    08 08 08 09 08 16 16 16 08 08
    08 08 09 08 08 16 16 16 08 08
    08 09 08 09 08 08 08 08 08 08
    08 08 08 08 08 08 09 08 08 08
    */
    fn _gen_big_marker_img() -> ImageBuffer<Luma<u8>, Vec<u8>> {
        let mut base_img = _gen_same_value_image(10, 10, 8);

        for i in 1..4 {
            for j in 1..4 {
                base_img.put_pixel(i, j, Luma([12]));
            }
        }

        for i in 5..8 {
            for j in 5..8 {
                base_img.put_pixel(i, j, Luma([16]));
            }
        }

        base_img.put_pixel(1, 5, Luma([9]));
        base_img.put_pixel(1, 8, Luma([9]));
        base_img.put_pixel(2, 7, Luma([9]));
        base_img.put_pixel(3, 6, Luma([9]));
        base_img.put_pixel(3, 8, Luma([9]));
        base_img.put_pixel(6, 1, Luma([9]));
        base_img.put_pixel(6, 3, Luma([9]));
        base_img.put_pixel(6, 9, Luma([9]));
        base_img.put_pixel(7, 2, Luma([9]));
        base_img.put_pixel(8, 1, Luma([9]));
        base_img.put_pixel(8, 3, Luma([9]));

        return base_img;
    }

    /*
    Gens the 10 x 10 image below
    10 10 10 10 10 10 10 10 10 10
    10 14 14 14 10 10 11 10 11 10
    10 14 14 14 10 10 10 11 10 10
    10 14 14 14 10 10 11 10 11 10
    10 10 10 10 10 10 10 10 10 10
    10 11 10 10 10 16 16 16 10 10
    10 10 10 11 10 16 16 16 10 10
    10 10 11 10 10 16 16 16 10 10
    10 11 10 11 10 10 10 10 10 10
    10 10 10 10 10 10 11 10 10 10
    */
    fn _gen_big_mask_img() -> ImageBuffer<Luma<u8>, Vec<u8>> {
        let mut base_img = _gen_same_value_image(10, 10, 10);

        for i in 1..4 {
            for j in 1..4 {
                base_img.put_pixel(i, j, Luma([14]));
            }
        }

        for i in 5..8 {
            for j in 5..8 {
                base_img.put_pixel(i, j, Luma([18]));
            }
        }

        base_img.put_pixel(1, 5, Luma([11]));
        base_img.put_pixel(1, 8, Luma([11]));
        base_img.put_pixel(2, 7, Luma([11]));
        base_img.put_pixel(3, 6, Luma([11]));
        base_img.put_pixel(3, 8, Luma([11]));
        base_img.put_pixel(6, 1, Luma([11]));
        base_img.put_pixel(6, 3, Luma([11]));
        base_img.put_pixel(6, 9, Luma([11]));
        base_img.put_pixel(7, 2, Luma([11]));
        base_img.put_pixel(8, 1, Luma([11]));
        base_img.put_pixel(8, 3, Luma([11]));

        return base_img;
    }

    /*
    Gens the 10 x 10 image below
    10 10 10 10 10 10 10 10 10 10
    10 12 12 12 10 10 10 10 10 10
    10 12 12 12 10 10 10 10 10 10
    10 12 12 12 10 10 10 10 10 10
    10 10 10 10 10 10 10 10 10 10
    10 10 10 10 10 16 16 16 10 10
    10 10 10 10 10 16 16 16 10 10
    10 10 10 10 10 16 16 16 10 10
    10 10 10 10 10 10 10 10 10 10
    10 10 10 10 10 10 10 10 10 10
    */
    fn _gen_expected_img() -> ImageBuffer<Luma<u8>, Vec<u8>> {
        let mut base_img = _gen_same_value_image(10, 10, 10);

        for i in 1..4 {
            for j in 1..4 {
                base_img.put_pixel(i, j, Luma([12]));
            }
        }

        for i in 5..8 {
            for j in 5..8 {
                base_img.put_pixel(i, j, Luma([16]));
            }
        }

        return base_img;
    }

    #[test]
    fn test_get_pixel_neighbours() {
        let mask = _gen_example_img();
        let ngbs = iwp::get_pixel_neighbours(&mask, (0, 0), iwp::ConnTypes::Eight);
        let expected = vec![(0, 1), (1, 0), (1, 1)];

        assert_eq!(ngbs, expected);

        let ngbs = iwp::get_pixel_neighbours(&mask, (0, 0), iwp::ConnTypes::Four);
        let expected = vec![(0, 1), (1, 0)];

        assert_eq!(ngbs, expected);

        let ngbs = iwp::get_pixel_neighbours(&mask, (2, 2), iwp::ConnTypes::Eight);
        let expected = vec![
            (1, 1),
            (1, 2),
            (1, 3),
            (2, 1),
            (2, 3),
            (3, 1),
            (3, 2),
            (3, 3),
        ];

        assert_eq!(ngbs, expected);
    }

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

        iwp::iwp(
            &mut marker,
            propagation_condition,
            update_func,
            &mut initial,
            &mask,
        );

        assert_eq!(marker, _gen_expected_img());
    }
}
