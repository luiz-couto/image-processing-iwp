use crate::{img, parallel_img};
use image::Luma;
use std::sync::Arc;
use std::thread;

#[derive(Debug)]
struct IWPSection<'a> {
    section: &'a mut parallel_img::ParallelSection,
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
        aux_structure: &T,
    ) -> bool,
    update_func: fn(
        img: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
        curr_pixel: img::PixelT,
        ngb_pixel: img::PixelT,
        aux_structure: &T,
    ) -> u8,
    queue: &mut Vec<(u32, u32)>,
    aux_structure: &mut T,
    num_threads: u32,
) where
    T: Clone + Send + Sync + 'static,
{
    let mut parallel_sections: Vec<IWPSection> = Vec::new();
    let mut sections = parallel_img::arrange(base_img, num_threads);

    for section in &mut sections {
        let mut sec_queue: Vec<(u32, u32)> = Vec::new();
        for (_, p_coords) in queue.iter().enumerate() {
            if img::is_pixel_in_section(*p_coords, &section) {
                //queue.remove(idx);
                sec_queue.push(*p_coords);
            }
        }

        parallel_sections.push(IWPSection {
            section,
            queue: sec_queue,
        })
    }

    thread::scope(|s| {
        let mut handles = vec![];
        let aux_s = Arc::new(aux_structure.clone());
        for section in &mut parallel_sections {
            let aux_s = Arc::clone(&aux_s);
            let handle = s.spawn(move || {
                println!("Running thread of section {:?}", section.section.start);
                while section.queue.len() != 0 {
                    let pixel_coords_abs = section.queue.remove(0);
                    let pixel_coords = (
                        pixel_coords_abs.0 - section.section.start.0,
                        pixel_coords_abs.1 - section.section.start.1,
                    );

                    let curr_pixel = img::PixelT {
                        coords: pixel_coords,
                        value: section
                            .section
                            .slice
                            .get_pixel(pixel_coords.0, pixel_coords.1)
                            .0[0],
                    };

                    let curr_pixel_abs = img::PixelT {
                        coords: pixel_coords_abs,
                        value: curr_pixel.value,
                    };

                    let pixel_ngbs = img::get_pixel_neighbours(
                        &section.section.slice,
                        pixel_coords,
                        img::ConnTypes::Eight,
                    );

                    for ngb_coord in pixel_ngbs {
                        if img::is_pixel_in_section(ngb_coord, section.section) {
                            let ngb_pixel = img::PixelT {
                                coords: ngb_coord,
                                value: section.section.slice.get_pixel(ngb_coord.0, ngb_coord.1).0
                                    [0],
                            };

                            let ngb_pixel_abs = img::PixelT {
                                coords: (
                                    ngb_pixel.coords.0 + section.section.start.0,
                                    ngb_pixel.coords.1 + section.section.start.1,
                                ),
                                value: ngb_pixel.value,
                            };

                            if propagation_condition(
                                &section.section.slice,
                                curr_pixel_abs,
                                ngb_pixel_abs,
                                &aux_s,
                            ) {
                                let new_value = update_func(
                                    &section.section.slice,
                                    curr_pixel_abs,
                                    ngb_pixel_abs,
                                    &aux_s,
                                );

                                let mut ngb = section
                                    .section
                                    .slice
                                    .get_pixel_mut(ngb_coord.0, ngb_coord.1);
                                ngb.0[0] = new_value;

                                section.queue.push(ngb_pixel_abs.coords);
                            }
                        }
                    }
                }
                println!("Finish thread of section {:?}", section.section.start);
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    });

    let full_img = parallel_img::get_full_img(base_img.width(), base_img.height(), &sections);
    //full_img.save(path)
    //print_image_by_row(&full_img);
}
