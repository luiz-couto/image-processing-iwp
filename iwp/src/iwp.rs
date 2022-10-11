use image::Luma;

pub enum ConnTypes {
    Four = 4,
    Eight = 8,
}

#[derive(Clone, Copy)]
pub struct PixelT {
    pub coords: (u32, u32),
    pub value: u8,
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

pub fn iwp<T>(
    base_img: &mut image::ImageBuffer<Luma<u8>, Vec<u8>>,
    propagation_condition: fn(
        img: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
        curr_pixel: PixelT,
        ngb_pixel: PixelT,
        aux_structure: T,
    ) -> bool,
    update_func: fn(
        img: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
        curr_pixel: PixelT,
        ngb_pixel: PixelT,
        aux_structure: T,
    ) -> u8,
    queue: &mut Vec<(u32, u32)>,
    aux_structure: T,
) where
    T: Copy, // check if this makes sense
{
    while queue.len() != 0 {
        let pixel_coords = queue.remove(0); // change this method to a more efficent one
        let pixel_ngbs = get_pixel_neighbours(base_img, pixel_coords, ConnTypes::Eight);
        let curr_pixel = PixelT {
            coords: pixel_coords,
            value: base_img.get_pixel(pixel_coords.0, pixel_coords.1).0[0],
        };

        for ngb_coord in pixel_ngbs {
            let ngb_pixel = PixelT {
                coords: ngb_coord,
                value: base_img.get_pixel(ngb_coord.0, ngb_coord.1).0[0],
            };

            if propagation_condition(base_img, curr_pixel, ngb_pixel, aux_structure) {
                let new_value = update_func(base_img, curr_pixel, ngb_pixel, aux_structure);

                let mut ngb = base_img.get_pixel_mut(ngb_coord.0, ngb_coord.1);
                ngb.0[0] = new_value;

                queue.push(ngb_coord);
            }
        }
    }
}
