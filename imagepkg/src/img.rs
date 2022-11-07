use image::Luma;

use crate::examples::_gen_same_value_image;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PixelT {
    pub coords: (u32, u32),
    pub value: u8,
}

pub enum ConnTypes {
    Four = 4,
    Eight = 8,
}

pub fn get_pixel_neighbours(
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

pub fn convert_to_binary(
    img: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
) -> image::ImageBuffer<Luma<u8>, Vec<u8>> {
    let width = img.width();
    let height = img.height();

    let mut binary_img = _gen_same_value_image(width, height, 0);

    for i in 0..height {
        for j in 0..width {
            let pixel_coords = (j, i);
            let pixel_value = img.get_pixel(pixel_coords.0, pixel_coords.1).0[0];
            if pixel_value > 128 {
                binary_img.put_pixel(pixel_coords.0, pixel_coords.1, Luma([1]));
            }
        }
    }

    return binary_img;
}

pub fn arrange(img: &image::ImageBuffer<Luma<u8>, Vec<u8>>, num_windows: u32) {
    let columns = (num_windows as f32).sqrt().ceil() as u32;
    let full_rows = num_windows / columns;
    let orphans = num_windows % columns;

    let aux = if orphans == 0 {
        full_rows
    } else {
        full_rows + 1
    };

    let width = img.width() / columns;
    let height = img.height() / aux;

    for y in 0..full_rows {
        for x in 0..columns {
            println!("({:?}, {:?})", x * width, y * height);
        }
    }

    if orphans > 0 {
        let orphan_width = img.width() / orphans;
        let y = full_rows;
        for x in 0..orphans {
            println!("{:?}, {:?}", x * orphan_width, y * height);
        }
    }
}

mod tests {

    #![allow(unused_imports)]

    use crate::examples;
    use crate::img;

    use super::arrange;

    #[test]
    fn test_get_pixel_neighbours() {
        let mask = examples::_gen_example_img();
        let ngbs = img::get_pixel_neighbours(&mask, (0, 0), img::ConnTypes::Eight);
        let expected = vec![(0, 1), (1, 0), (1, 1)];

        assert_eq!(ngbs, expected);

        let ngbs = img::get_pixel_neighbours(&mask, (0, 0), img::ConnTypes::Four);
        let expected = vec![(0, 1), (1, 0)];

        assert_eq!(ngbs, expected);

        let ngbs = img::get_pixel_neighbours(&mask, (2, 2), img::ConnTypes::Eight);
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
    fn test_arrange() {
        let img = examples::_gen_same_value_image(37, 37, 0);
        arrange(&img, 8);
    }
}
