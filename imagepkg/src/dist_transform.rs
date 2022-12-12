use std::collections::{HashSet, VecDeque};

use image::{ImageBuffer, Luma};

use crate::{examples::_gen_same_value_image, img, iwp};

const BG: u8 = 0;
const FR: u8 = 1;
const INF_PIXEL: u32 = u32::MAX;

#[derive(Clone)]
pub enum DistTypes {
    Euclidean,
    CityBlock,
    Chessboard,
}

type DistFunc = fn(p1: (u32, u32), p2: (u32, u32)) -> u32;

fn aprox_euclidean_distance(p1: (u32, u32), p2: (u32, u32)) -> u32 {
    let exp = ((p1.0 as f64 - p2.0 as f64).powi(2) + (p1.1 as f64 - p2.1 as f64).powi(2)).sqrt();
    return exp.round() as u32;
}

fn city_block_distance(p1: (u32, u32), p2: (u32, u32)) -> u32 {
    let exp = (p1.0 as i64 - p2.0 as i64).abs() + (p1.1 as i64 - p2.1 as i64).abs();
    return exp as u32;
}

fn chessboard_distance(p1: (u32, u32), p2: (u32, u32)) -> u32 {
    let exp = std::cmp::max(
        (p1.0 as i64 - p2.0 as i64).abs(),
        (p1.1 as i64 - p2.1 as i64).abs(),
    );

    return exp as u32;
}

fn get_one_dimension_coords(
    img: &image::ImageBuffer<Luma<u32>, Vec<u32>>,
    coords: (u32, u32),
) -> u32 {
    return (coords.1 * img.width()) + coords.0;
}

fn get_two_dimensions_coords(
    img: &image::ImageBuffer<Luma<u32>, Vec<u32>>,
    coords: u32,
) -> (u32, u32) {
    return (coords % img.width(), coords / img.width());
}

fn get_initial_pixels(
    img: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
    vr_diagram: &mut image::ImageBuffer<Luma<u32>, Vec<u32>>,
) -> VecDeque<(u32, u32)> {
    let width = img.width();
    let height = img.height();
    let mut queue = HashSet::new();

    for i in 0..height {
        for j in 0..width {
            let pixel_coords = (j, i);
            let curr_pixel = img.get_pixel(pixel_coords.0, pixel_coords.1);
            let pixel_value = curr_pixel.0[0];

            if pixel_value == BG {
                vr_diagram.put_pixel(
                    pixel_coords.0,
                    pixel_coords.1,
                    Luma([get_one_dimension_coords(vr_diagram, pixel_coords)]),
                );
                let pixel_ngbs =
                    img::get_pixel_neighbours(img, pixel_coords, img::ConnTypes::Eight);

                for ngb_coord in pixel_ngbs {
                    let ngb_value = img.get_pixel(ngb_coord.0, ngb_coord.1).0[0];
                    if ngb_value == FR {
                        queue.insert(pixel_coords);
                        break;
                    }
                }

                continue;
            }

            vr_diagram.put_pixel(pixel_coords.0, pixel_coords.1, Luma([INF_PIXEL]));
        }
    }

    let queue = VecDeque::from_iter(queue); //check complexity of this operation later
    return queue;
}

fn propagation_condition(
    img: &image::ImageBuffer<Luma<u32>, Vec<u32>>,
    curr_pixel: img::PixelT<u32>,
    ngb_pixel: img::PixelT<u32>,
    dist_func: &DistFunc,
) -> bool {
    let vr_p = get_two_dimensions_coords(img, curr_pixel.value);
    let vr_q = get_two_dimensions_coords(img, ngb_pixel.value);

    return dist_func(ngb_pixel.coords, vr_p) < dist_func(ngb_pixel.coords, vr_q);
}

fn update_func(
    _img: &image::ImageBuffer<Luma<u32>, Vec<u32>>,
    curr_pixel: img::PixelT<u32>,
    _ngb_pixel: img::PixelT<u32>,
    _dist_func: &DistFunc,
) -> u32 {
    return curr_pixel.value;
}

fn get_final_dist_img(
    width: u32,
    height: u32,
    vr_diagram: &mut image::ImageBuffer<Luma<u32>, Vec<u32>>,
    dist_func: &DistFunc,
) -> image::ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut img = _gen_same_value_image(width, height, 0);
    for i in 0..height {
        for j in 0..width {
            let pixel_coords = (j, i);
            let vr_p = get_two_dimensions_coords(vr_diagram, vr_diagram.get_pixel(j, i).0[0]);
            let value = dist_func(pixel_coords, vr_p);

            img.put_pixel(pixel_coords.0, pixel_coords.1, Luma([value as u8]));
        }
    }

    return img;
}

pub fn dist_transform(
    img: &mut image::ImageBuffer<Luma<u8>, Vec<u8>>,
    dist_type: DistTypes,
) -> image::ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut vr_diagram = ImageBuffer::new(img.width(), img.height());
    let mut queue = get_initial_pixels(img, &mut vr_diagram);

    let dist_func = match dist_type {
        DistTypes::Euclidean => aprox_euclidean_distance,
        DistTypes::Chessboard => chessboard_distance,
        DistTypes::CityBlock => city_block_distance,
    };

    iwp::propagate(
        &mut vr_diagram,
        propagation_condition,
        update_func,
        &mut queue,
        &dist_func,
    );

    return get_final_dist_img(img.width(), img.height(), &mut vr_diagram, &dist_func);
}

pub fn dist_transform_parallel(
    img: &mut image::ImageBuffer<Luma<u8>, Vec<u8>>,
    dist_type: DistTypes,
    num_threads: u32,
) -> image::ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut vr_diagram = ImageBuffer::new(img.width(), img.height());
    let mut queue = get_initial_pixels(img, &mut vr_diagram);

    let dist_func = match dist_type {
        DistTypes::Euclidean => aprox_euclidean_distance,
        DistTypes::Chessboard => chessboard_distance,
        DistTypes::CityBlock => city_block_distance,
    };

    let mut result = iwp::propagate_parallel(
        &mut vr_diagram,
        propagation_condition,
        update_func,
        &mut queue,
        &dist_func,
        num_threads,
    );

    return get_final_dist_img(img.width(), img.height(), &mut result, &dist_func);
}

mod tests {

    #![allow(unused_imports)]

    use crate::{
        dist_transform::*,
        examples::{_gen_example_img, _gen_same_value_image},
        format::print_image_by_row,
    };

    /*
    Testing with the image:
    0 0 0
    0 1 0
    0 0 0
    */
    #[test]
    fn test_get_initial_pixels() {
        let mut vr_diagram = ImageBuffer::new(3, 3);
        let mut img = _gen_same_value_image(3, 3, 0);

        img.put_pixel(1, 1, Luma([1]));

        let queue = get_initial_pixels(&img, &mut vr_diagram);
        let mut expected: Vec<(u32, u32)> = vec![
            (0, 2),
            (2, 1),
            (0, 1),
            (2, 2),
            (1, 0),
            (2, 0),
            (1, 2),
            (0, 0),
        ];

        assert_eq!(Vec::from_iter(queue).sort(), expected.sort());

        let p_value = vr_diagram.get_pixel(1, 1).0[0];
        assert_eq!(p_value, INF_PIXEL);
    }

    #[test]
    fn test_aprox_euclidean_distance() {
        let res = aprox_euclidean_distance((1, 1), (1, 1));
        let exp = 0;
        assert_eq!(exp, res);

        let res = aprox_euclidean_distance((1, 1), (3, 1));
        let exp = 2;
        assert_eq!(exp, res);

        let res = aprox_euclidean_distance((2, 2), (3, 1));
        let exp = 1;
        assert_eq!(exp, res);

        let res = aprox_euclidean_distance((2, 2), (4, 0));
        let exp = 3;
        assert_eq!(exp, res);
    }

    #[test]
    fn test_city_block_distance() {
        let res = city_block_distance((1, 1), (1, 1));
        let exp = 0;
        assert_eq!(exp, res);

        let res = city_block_distance((1, 1), (3, 1));
        let exp = 2;
        assert_eq!(exp, res);

        let res = city_block_distance((2, 2), (3, 1));
        let exp = 2;
        assert_eq!(exp, res);

        let res = city_block_distance((1, 0), (7, 6));
        let exp = 12;
        assert_eq!(exp, res);

        let res = city_block_distance((0, 0), (2, 2));
        let exp = 4;
        assert_eq!(exp, res);
    }

    #[test]
    fn test_chessboard_distance() {
        let res = chessboard_distance((1, 1), (1, 1));
        let exp = 0;
        assert_eq!(exp, res);

        let res = chessboard_distance((1, 1), (3, 1));
        let exp = 2;
        assert_eq!(exp, res);

        let res = chessboard_distance((2, 2), (3, 1));
        let exp = 1;
        assert_eq!(exp, res);

        let res = chessboard_distance((2, 2), (4, 0));
        let exp = 2;
        assert_eq!(exp, res);
    }

    #[test]
    fn test_euclidean_dist_transform() {
        let mut img = _gen_same_value_image(3, 3, 1);
        img.put_pixel(2, 2, Luma([0]));

        let dis_img = dist_transform(&mut img, DistTypes::Euclidean);

        let mut expected = _gen_same_value_image(3, 3, 2);
        expected.put_pixel(0, 0, Luma([3]));
        expected.put_pixel(1, 1, Luma([1]));
        expected.put_pixel(2, 1, Luma([1]));
        expected.put_pixel(1, 2, Luma([1]));
        expected.put_pixel(2, 2, Luma([0]));

        assert_eq!(dis_img, expected);
    }

    #[test]
    fn test_city_block_dist_transform() {
        let mut img = _gen_same_value_image(3, 3, 1);
        img.put_pixel(2, 2, Luma([0]));

        let dis_img = dist_transform(&mut img, DistTypes::CityBlock);

        let mut expected = _gen_same_value_image(3, 3, 2);
        expected.put_pixel(0, 0, Luma([4]));
        expected.put_pixel(0, 1, Luma([3]));
        expected.put_pixel(1, 0, Luma([3]));
        expected.put_pixel(1, 2, Luma([1]));
        expected.put_pixel(2, 1, Luma([1]));
        expected.put_pixel(2, 2, Luma([0]));

        assert_eq!(dis_img, expected);
    }

    #[test]
    fn test_chessboard_dist_transform() {
        let mut img = _gen_same_value_image(3, 3, 1);
        img.put_pixel(2, 2, Luma([0]));

        let dis_img = dist_transform(&mut img, DistTypes::Chessboard);

        let mut expected = _gen_same_value_image(3, 3, 2);
        expected.put_pixel(1, 1, Luma([1]));
        expected.put_pixel(1, 2, Luma([1]));
        expected.put_pixel(2, 1, Luma([1]));
        expected.put_pixel(2, 2, Luma([0]));

        assert_eq!(dis_img, expected);
    }
}
