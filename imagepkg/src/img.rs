use image::Luma;

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
}
