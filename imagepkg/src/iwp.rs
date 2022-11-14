use crate::img;
use image::{imageops, Luma};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone, Debug)]
struct ParallelSection {
    section: img::Section,
    queue: Vec<(u32, u32)>,
}

pub fn propagate<T>(
    base_img: &mut image::ImageBuffer<Luma<u8>, Vec<u8>>,
    propagation_condition: fn(
        img: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
        curr_pixel: img::PixelT,
        ngb_pixel: img::PixelT,
        aux_structure: &mut T,
    ) -> bool,
    update_func: fn(
        img: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
        curr_pixel: img::PixelT,
        ngb_pixel: img::PixelT,
        aux_structure: &mut T,
    ) -> u8,
    queue: &mut Vec<(u32, u32)>,
    aux_structure: &mut T,
) where
    T: Clone, // check if this makes sense
{
    while queue.len() != 0 {
        let pixel_coords = queue.remove(0); // change this method to a more efficent one
        let pixel_ngbs = img::get_pixel_neighbours(base_img, pixel_coords, img::ConnTypes::Eight);
        let curr_pixel = img::PixelT {
            coords: pixel_coords,
            value: base_img.get_pixel(pixel_coords.0, pixel_coords.1).0[0],
        };

        for ngb_coord in pixel_ngbs {
            let ngb_pixel = img::PixelT {
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

pub fn propagate_parallel<T>(
    base_img: &mut image::ImageBuffer<Luma<u8>, Vec<u8>>,
    propagation_condition: fn(
        img: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
        curr_pixel: img::PixelT,
        ngb_pixel: img::PixelT,
        aux_structure: &mut Arc<Mutex<T>>,
    ) -> bool,
    update_func: fn(
        img: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
        curr_pixel: img::PixelT,
        ngb_pixel: img::PixelT,
        aux_structure: &mut Arc<Mutex<T>>,
    ) -> u8,
    queue: &mut Vec<(u32, u32)>,
    aux_structure: &mut T,
    num_threads: u32,
) where
    T: Clone + Send + 'static,
{
    let mut parallel_sections: Vec<ParallelSection> = Vec::new();
    let sections = img::arrange(&base_img, num_threads);

    //println!("{:?}", queue);

    for section in sections {
        let mut sec_queue: Vec<(u32, u32)> = Vec::new();
        for (idx, p_coords) in queue.iter().enumerate() {
            if img::is_pixel_in_section(*p_coords, section) {
                //queue.remove(idx);
                sec_queue.push(*p_coords);
            }
        }

        parallel_sections.push(ParallelSection {
            section,
            queue: sec_queue,
        })
    }

    //println!("{:?}", parallel_sections);

    // let img = Arc::new(base_img.clone());
    // let img_2 = Arc::new(Mutex::new(base_img.clone()));
    let aux_s = Arc::new(Mutex::new(aux_structure.clone()));

    let mut handles = vec![];

    for mut section in parallel_sections {
        let mut img_sec = imageops::crop(
            base_img,
            section.section.start.0,
            section.section.start.1,
            section.section.width,
            section.section.height,
        )
        .to_image();

        let mut aux_s = Arc::clone(&aux_s);
        let handle = thread::spawn(move || {
            //println!("Running thread of section {:?}", section.section);

            while section.queue.len() != 0 {
                let pixel_coords_abs = section.queue.remove(0);
                let pixel_coords = (
                    pixel_coords_abs.0 - section.section.start.0,
                    pixel_coords_abs.1 - section.section.start.1,
                );
                let curr_pixel = img::PixelT {
                    coords: pixel_coords,
                    value: img_sec.get_pixel(pixel_coords.0, pixel_coords.1).0[0],
                };
                let pixel_ngbs =
                    img::get_pixel_neighbours(&img_sec, pixel_coords, img::ConnTypes::Eight);

                for ngb_coord in pixel_ngbs {
                    if img::is_pixel_in_section(ngb_coord, section.section) {
                        let ngb_pixel = img::PixelT {
                            coords: ngb_coord,
                            value: img_sec.get_pixel(ngb_coord.0, ngb_coord.1).0[0],
                        };

                        if propagation_condition(&img_sec, curr_pixel, ngb_pixel, &mut aux_s) {
                            let new_value =
                                update_func(&img_sec, curr_pixel, ngb_pixel, &mut aux_s);

                            let mut ngb = img_sec.get_pixel_mut(ngb_coord.0, ngb_coord.1);
                            ngb.0[0] = new_value;

                            section.queue.push(ngb_coord);
                        }
                    }
                }
            }
            //println!("Finish thread of section {:?}", section.section);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

// fn each_thread(
//     section: &mut ParallelSection,
//     base_img: &mut image::ImageBuffer<Luma<u8>, Vec<u8>>
// )
