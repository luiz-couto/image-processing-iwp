use image::Luma;

use crate::examples::_gen_same_value_image;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PixelT {
    pub coords: (u32, u32),
    pub value: u8,
}

#[derive(Clone, Copy)]
pub struct Section {
    pub start: (u32, u32),
    pub width: u32,
    pub height: u32,
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

pub fn arrange(img: &image::ImageBuffer<Luma<u8>, Vec<u8>>, num_windows: u32) -> Vec<Section> {
    let mut sections: Vec<Section> = Vec::new();
    let columns = (num_windows as f32).sqrt().ceil() as u32;
    let full_rows = num_windows / columns;
    let orphans = num_windows % columns;

    let aux = if orphans == 0 {
        full_rows
    } else {
        full_rows + 1
    };

    let base_width = img.width() / columns;
    let base_height = img.height() / aux;

    let width_leftover = img.width() % columns;
    let height_leftover = img.height() % aux;

    for y in 0..full_rows {
        for x in 0..columns {
            let width = if x == columns - 1 {
                base_width + width_leftover
            } else {
                base_width
            };

            let height = if orphans == 0 && y == full_rows - 1 {
                base_height + height_leftover
            } else {
                base_height
            };

            sections.push(Section {
                start: (x * base_width, y * base_height),
                width,
                height,
            });

            // println!(
            //     "({:?}, {:?}), width: {:?}, height: {:?}",
            //     x * base_width,
            //     y * base_height,
            //     width,
            //     height
            // );
        }
    }

    if orphans > 0 {
        let orphan_width = img.width() / orphans;
        let y = full_rows;
        for x in 0..orphans {
            let width = if x == orphans - 1 {
                base_width + width_leftover
            } else {
                base_width
            };

            sections.push(Section {
                start: (x * orphan_width, y * base_height),
                width,
                height: base_height + height_leftover,
            });
            // println!(
            //     "({:?}, {:?}), width: {:?}, height: {:?}",
            //     x * orphan_width,
            //     y * base_height,
            //     width,
            //     base_height + height_leftover
            // );
        }
    }

    return sections;
}

pub fn is_pixel_in_section(pixel: (u32, u32), section: Section) -> bool {
    if (section.start.0 <= pixel.0)
        && (pixel.0 <= section.start.0 + section.width)
        && (section.start.1 <= pixel.1)
        && (pixel.1 <= section.start.1 + section.height)
    {
        return true;
    }

    return false;
}

mod tests {

    #![allow(unused_imports)]

    use crate::examples;
    use crate::img;

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
        let img = examples::_gen_same_value_image(200, 37, 0);
        img::arrange(&img, 4);
    }

    #[test]
    fn test_is_pixel_in_section() {
        let section = img::Section {
            start: (0, 0),
            width: 2,
            height: 2,
        };

        assert_eq!(img::is_pixel_in_section((0, 0), section), true);
        assert_eq!(img::is_pixel_in_section((2, 2), section), true);
        assert_eq!(img::is_pixel_in_section((3, 2), section), false);
    }
}
