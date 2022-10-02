use image::io::Reader as ImageReader;
use image::{self, Luma};
use std::error::Error;

pub enum ConnTypes {
    Four = 4,
    Eight = 8,
}

fn get_pixel_neighbours(
    img: &image::ImageBuffer<Luma<u16>, Vec<u16>>,
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
    mask: &image::ImageBuffer<Luma<u16>, Vec<u16>>,
    marker: &mut image::ImageBuffer<Luma<u16>, Vec<u16>>,
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

fn get_initial_pixels(
    mask: &image::ImageBuffer<Luma<u16>, Vec<u16>>,
    marker: &mut image::ImageBuffer<Luma<u16>, Vec<u16>>,
) -> Vec<(u32, u32)> {
    let width = mask.width();
    let height = mask.height();
    let mut queue = Vec::new();

    for i in 0..height {
        for j in 0..width {
            let pixel_coords = (j, i);
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
    }

    for i in (0..height).rev() {
        for j in (0..width).rev() {
            let pixel_coords = (j, i);
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
            let pixel_value = pixel.0[0];

            ////////////////////

            for ngb_coord in pixel_ngbs {
                let ngb = marker.get_pixel(ngb_coord.0, ngb_coord.1);

                if (ngb.0[0] < pixel_value) && (ngb.0[0] < mask_pixel.0[0]) {
                    queue.push(ngb_coord);
                }
            }
        }
    }

    return queue;
}

fn main() -> Result<(), Box<dyn Error>> {
    let img = ImageReader::open("fish.png")?.decode()?;

    let mask = img.to_luma16();
    let mut marker = img.to_luma16();

    let dimensions = mask.dimensions();
    println!("dimensions: {:?}", dimensions);

    get_initial_pixels(&mask, &mut marker);

    Ok(())
}
