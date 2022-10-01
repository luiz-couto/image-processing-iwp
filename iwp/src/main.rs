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

fn main() -> Result<(), Box<dyn Error>> {
    let img = ImageReader::open("fish.png")?.decode()?;

    let img_in_grey = img.to_luma16();

    let dimensions = img_in_grey.dimensions();
    println!("dimensions: {:?}", dimensions);

    let ngbs = get_pixel_neighbours(&img_in_grey, (7, 0), ConnTypes::Four);

    println!("{:?}", ngbs);
    //println!("Hello, world!");
    Ok(())
}
