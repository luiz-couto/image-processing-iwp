use image::{imageops, Luma, Primitive};

use crate::{examples::_gen_same_value_image, parallel_img::ParallelSection};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PixelT<P: Primitive> {
    pub coords: (u32, u32),
    pub value: P,
}

pub enum ConnTypes {
    Four = 4,
    Eight = 8,
}

pub fn get_pixel_neighbours<P: Primitive>(
    img: &image::ImageBuffer<Luma<P>, Vec<P>>,
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

// Check if this functin is correct
pub fn is_pixel_in_section<P: Primitive>(pixel: (u32, u32), section: &ParallelSection<P>) -> bool {
    if (section.start.0 <= pixel.0)
        && (pixel.0 < section.start.0 + section.width)
        && (section.start.1 <= pixel.1)
        && (pixel.1 < section.start.1 + section.height)
    {
        return true;
    }

    return false;
}

pub fn get_upper_border_pixels_coords<P: Primitive>(
    img: &image::ImageBuffer<Luma<P>, Vec<P>>,
) -> Vec<(u32, u32)> {
    let mut border = vec![];
    for i in 0..img.width() {
        for j in 0..1 {
            border.push((i, j));
        }
    }

    return border;
}

pub fn get_left_border_pixels_coords<P: Primitive>(
    img: &image::ImageBuffer<Luma<P>, Vec<P>>,
) -> Vec<(u32, u32)> {
    let mut border = vec![];
    for i in 0..1 {
        for j in 0..img.height() {
            border.push((i, j));
        }
    }

    return border;
}

pub fn get_bottom_border_pixels_coords<P: Primitive>(
    img: &image::ImageBuffer<Luma<P>, Vec<P>>,
) -> Vec<(u32, u32)> {
    let mut border = vec![];
    for i in 0..img.width() {
        for j in (img.height() - 1)..img.height() {
            border.push((i, j));
        }
    }

    return border;
}

pub fn get_right_border_pixels_coords<P: Primitive>(
    img: &image::ImageBuffer<Luma<P>, Vec<P>>,
) -> Vec<(u32, u32)> {
    let mut border = vec![];
    for i in (img.width() - 1)..img.width() {
        for j in 0..img.height() {
            border.push((i, j));
        }
    }

    return border;
}

mod tests {

    #![allow(unused_imports)]

    use crate::examples;
    use crate::examples::_gen_example_img;
    use crate::examples::_gen_seq_img;
    use crate::img;
    use crate::parallel_img;

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
    fn test_is_pixel_in_section() {
        let section = parallel_img::ParallelSection {
            start: (0, 0),
            width: 2,
            height: 2,
            slice: _gen_example_img(),
        };

        assert_eq!(img::is_pixel_in_section((0, 0), &section), true);
        assert_eq!(img::is_pixel_in_section((2, 2), &section), false);
        assert_eq!(img::is_pixel_in_section((1, 2), &section), false);
        assert_eq!(img::is_pixel_in_section((3, 2), &section), false);
    }

    #[test]
    fn test_get_upper_border_pixels_coords() {
        let img = _gen_seq_img();
        let mut upper_border = img::get_upper_border_pixels_coords(&img);
        let mut expected: Vec<(u32, u32)> = vec![(0, 0), (1, 0), (2, 0), (3, 0)];
        upper_border.sort();
        expected.sort();

        assert_eq!(upper_border, expected);
    }

    #[test]
    fn test_get_left_border_pixels_coords() {
        let img = _gen_seq_img();
        let mut upper_border = img::get_left_border_pixels_coords(&img);
        let mut expected: Vec<(u32, u32)> = vec![(0, 0), (0, 1), (0, 2), (0, 3)];
        upper_border.sort();
        expected.sort();

        assert_eq!(upper_border, expected);
    }

    #[test]
    fn test_get_bottom_border_pixels_coords() {
        let img = _gen_seq_img();
        let mut upper_border = img::get_bottom_border_pixels_coords(&img);
        let mut expected: Vec<(u32, u32)> = vec![(0, 3), (1, 3), (2, 3), (3, 3)];
        upper_border.sort();
        expected.sort();

        assert_eq!(upper_border, expected);
    }

    #[test]
    fn test_get_right_border_pixels_coords() {
        let img = _gen_seq_img();
        let mut upper_border = img::get_right_border_pixels_coords(&img);
        let mut expected: Vec<(u32, u32)> = vec![(3, 0), (3, 1), (3, 2), (3, 3)];
        upper_border.sort();
        expected.sort();

        assert_eq!(upper_border, expected);
    }
}
