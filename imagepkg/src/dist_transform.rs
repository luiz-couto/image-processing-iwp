use std::collections::{HashMap, HashSet};

use image::Luma;

use crate::img;

const BG: u8 = 0;
const FR: u8 = 1;
const INF_PIXEL: (u32, u32) = (u32::MAX, u32::MAX);

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

mod tests {

    #![allow(unused_imports)]

    use crate::{
        dist_transform::*,
        examples::{_gen_example_img, _gen_same_value_image},
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

        println!("{:?}", vr_diagram);
        assert_eq!(queue.sort(), expected.sort());

        match vr_diagram.get(&(1, 1)) {
            Some(fg_pixel_vr) => assert_eq!(*fg_pixel_vr, INF_PIXEL),
            None => panic!("Test failed: pixel (1,1) not found in vr map"),
        }
    }
}
