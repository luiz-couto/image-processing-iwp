use std::collections::HashSet;

use crate::format;
use image::Luma;

pub enum ConnTypes {
    Four = 4,
    Eight = 8,
}

fn get_pixel_neighbours(
    img: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
    coords: (u32, u32),
    conn: ConnTypes,
) -> Vec<(u32, u32)> {
    let x = coords.0;
    let y = coords.1;
    let mut neighbours = Vec::new();

    let floor_x = if x != 0 { x - 1 } else { x };
    let floor_y = if y != 0 { y - 1 } else { y };

    for i in (floor_x)..(x + 2) {
        for j in (floor_y)..(y + 2) {
            if !(i == x && j == y) {
                match img.get_pixel_checked(i, j) {
                    Some(_) => match conn {
                        ConnTypes::Four => {
                            if i == x || j == y {
                                neighbours.push((i, j));
                            }
                        }

                        ConnTypes::Eight => neighbours.push((i, j)),
                    },
                    None => continue,
                }
            }
        }
    }

    return neighbours;
}

fn update_pixel(
    pixel_coords: (u32, u32),
    mask: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
    marker: &mut image::ImageBuffer<Luma<u8>, Vec<u8>>,
) {
    let pixel_ngbs = get_pixel_neighbours(marker, pixel_coords, ConnTypes::Eight);

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

            let pixel_ngbs = get_pixel_neighbours(marker, pixel_coords, ConnTypes::Eight);

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

pub fn propagation_phase(
    mask: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
    marker: &mut image::ImageBuffer<Luma<u8>, Vec<u8>>,
    queue: &mut Vec<(u32, u32)>,
) {
    while queue.len() != 0 {
        let pixel_coords = queue.remove(0); // change this method to a more efficent one

        let pixel_ngbs = get_pixel_neighbours(marker, pixel_coords, ConnTypes::Eight);
        let pixel = *marker.get_pixel(pixel_coords.0, pixel_coords.1);

        for ngb_coord in pixel_ngbs {
            let mut ngb = marker.get_pixel_mut(ngb_coord.0, ngb_coord.1);
            let mask_ngb = mask.get_pixel(ngb_coord.0, ngb_coord.1);

            if (ngb.0[0] < pixel.0[0]) && (mask_ngb.0[0] != ngb.0[0]) {
                ngb.0[0] = std::cmp::min(pixel.0[0], mask_ngb.0[0]);
                queue.push(ngb_coord);
            }
        }
    }
}

mod tests {
    use image::{GrayImage, ImageBuffer};

    use crate::format::*;
    use crate::mr::*;

    fn gen_zero_image(width: u32, height: u32) -> ImageBuffer<Luma<u8>, Vec<u8>> {
        let mut img = GrayImage::new(width, height);
        for i in 0..width {
            for j in 0..height {
                img.put_pixel(i, j, Luma([0]))
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
    fn gen_example_img() -> ImageBuffer<Luma<u8>, Vec<u8>> {
        let mut img = gen_zero_image(6, 6);

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

    #[test]
    fn test_get_pixel_neighbours() {
        let mask = gen_example_img();
        let ngbs = get_pixel_neighbours(&mask, (0, 0), ConnTypes::Eight);
        let expected = vec![(0, 1), (1, 0), (1, 1)];

        assert_eq!(ngbs, expected);

        let ngbs = get_pixel_neighbours(&mask, (0, 0), ConnTypes::Four);
        let expected = vec![(0, 1), (1, 0)];

        assert_eq!(ngbs, expected);

        let ngbs = get_pixel_neighbours(&mask, (2, 2), ConnTypes::Eight);
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
        let mask = gen_example_img();

        let mut marker = gen_zero_image(6, 6);
        marker.put_pixel(4, 4, Luma([1]));

        let mut initial = get_initial_pixels(&mask, &mut marker);
        let mut expected = vec![(1, 1), (2, 1), (2, 2), (1, 2)];

        initial.sort();
        expected.sort();

        assert_eq!(initial, expected);
    }

    #[test]
    fn test_propagation_phase() {
        let mask = gen_example_img();

        let mut marker = gen_zero_image(6, 6);
        marker.put_pixel(4, 4, Luma([1]));

        let mut initial = get_initial_pixels(&mask, &mut marker);

        propagation_phase(&mask, &mut marker, &mut initial);

        print_image_by_row(&marker);
    }
}
