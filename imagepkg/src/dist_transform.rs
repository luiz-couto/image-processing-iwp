use std::collections::{HashMap, HashSet};

use image::Luma;

use crate::{examples::_gen_same_value_image, img, iwp};

const BG: u8 = 0;
const FR: u8 = 1;
const INF_PIXEL: (u32, u32) = (u32::MAX, u32::MAX);

#[derive(Clone)]
pub enum DistTypes {
    Euclidean,
    CityBlock,
    Chessboard,
}

#[derive(Clone)]
struct DistTransform {
    dist_func: fn(p1: (u32, u32), p2: (u32, u32)) -> u32,
    vr_diagram: HashMap<(u32, u32), (u32, u32)>,
}

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

fn get_initial_pixels(
    img: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
    vr_diagram: &mut HashMap<(u32, u32), (u32, u32)>,
) -> Vec<(u32, u32)> {
    let width = img.width();
    let height = img.height();
    let mut queue = HashSet::new();

    for i in 0..height {
        for j in 0..width {
            let pixel_coords = (j, i);
            let curr_pixel = img.get_pixel(pixel_coords.0, pixel_coords.1);
            let pixel_value = curr_pixel.0[0];

            if pixel_value == BG {
                vr_diagram.insert(pixel_coords, pixel_coords);
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

            vr_diagram.insert(pixel_coords, INF_PIXEL);
        }
    }

    let queue = Vec::from_iter(queue); //check complexity of this operation later
    return queue;
}

fn propagation_condition(
    _img: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
    curr_pixel: img::PixelT,
    ngb_pixel: img::PixelT,
    dist_struct: &mut DistTransform,
) -> bool {
    let vr_p = dist_struct.vr_diagram.get(&curr_pixel.coords).unwrap();
    let vr_q = dist_struct.vr_diagram.get(&ngb_pixel.coords).unwrap();

    return (dist_struct.dist_func)(ngb_pixel.coords, *vr_p)
        < (dist_struct.dist_func)(ngb_pixel.coords, *vr_q);
}

fn update_func(
    _img: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
    curr_pixel: img::PixelT,
    ngb_pixel: img::PixelT,
    dist_struct: &mut DistTransform,
) -> u8 {
    let vr_p = dist_struct.vr_diagram.get(&curr_pixel.coords).unwrap();
    dist_struct.vr_diagram.insert(ngb_pixel.coords, *vr_p);
    return ngb_pixel.value;
}

fn get_final_dist_img(
    width: u32,
    height: u32,
    vr_diagram: &HashMap<(u32, u32), (u32, u32)>,
) -> image::ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut img = _gen_same_value_image(width, height, 0);
    for i in 0..height {
        for j in 0..width {
            let pixel_coords = (j, i);
            let vr_p = vr_diagram.get(&pixel_coords).unwrap();
            let value = aprox_euclidean_distance(pixel_coords, *vr_p);

            img.put_pixel(pixel_coords.0, pixel_coords.1, Luma([value as u8]));
        }
    }

    return img;
}

pub fn dist_transform(
    img: &mut image::ImageBuffer<Luma<u8>, Vec<u8>>,
    dist_type: DistTypes,
) -> image::ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut vr_diagram = HashMap::<(u32, u32), (u32, u32)>::new();
    let mut queue = get_initial_pixels(img, &mut vr_diagram);

    let dist_func = match dist_type {
        DistTypes::Euclidean => aprox_euclidean_distance,
        DistTypes::Chessboard => chessboard_distance,
        DistTypes::CityBlock => city_block_distance,
    };

    let mut dist_struct = DistTransform {
        dist_func,
        vr_diagram,
    };

    iwp::propagate(
        img,
        propagation_condition,
        update_func,
        &mut queue,
        &mut dist_struct,
    );

    return get_final_dist_img(img.width(), img.height(), &dist_struct.vr_diagram);
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
        let mut vr_diagram = HashMap::<(u32, u32), (u32, u32)>::new();
        let mut img = _gen_same_value_image(3, 3, 0);

        img.put_pixel(1, 1, Luma([1]));

        let mut queue = get_initial_pixels(&img, &mut vr_diagram);
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

        assert_eq!(queue.sort(), expected.sort());

        match vr_diagram.get(&(1, 1)) {
            Some(fg_pixel_vr) => assert_eq!(*fg_pixel_vr, INF_PIXEL),
            None => panic!("Test failed: pixel (1,1) not found in vr map"),
        }
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
}
