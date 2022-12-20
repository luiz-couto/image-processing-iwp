use crate::parallel_img::ParallelSection;
use crate::{img, parallel_img};
use image::{ImageBuffer, Luma, Primitive};
use std::collections::VecDeque;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

#[derive(Debug)]
struct IWPSection<'a, P: Primitive> {
    section: &'a mut parallel_img::ParallelSection<P>,
    queue: VecDeque<(u32, u32)>,
}

pub fn propagate<T, P: Primitive>(
    base_img: &mut image::ImageBuffer<Luma<P>, Vec<P>>,
    propagation_condition: fn(
        img: &image::ImageBuffer<Luma<P>, Vec<P>>,
        curr_pixel: img::PixelT<P>,
        ngb_pixel: img::PixelT<P>,
        aux_structure: &T,
    ) -> bool,
    update_func: fn(
        img: &image::ImageBuffer<Luma<P>, Vec<P>>,
        curr_pixel: img::PixelT<P>,
        ngb_pixel: img::PixelT<P>,
        aux_structure: &T,
    ) -> P,
    queue: &mut VecDeque<(u32, u32)>,
    aux_structure: &T,
) where
    T: Clone, // check if this makes sense
{
    while queue.len() != 0 {
        let pixel_coords = queue.pop_front().unwrap(); // change this method to a more efficent one
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

                queue.push_back(ngb_coord);
            }
        }
    }
}

pub fn propagate_parallel<T, P: Primitive + Send + 'static>(
    base_img: &mut image::ImageBuffer<Luma<P>, Vec<P>>,
    propagation_condition: fn(
        img: &image::ImageBuffer<Luma<P>, Vec<P>>,
        curr_pixel: img::PixelT<P>,
        ngb_pixel: img::PixelT<P>,
        aux_structure: &T,
    ) -> bool,
    update_func: fn(
        img: &image::ImageBuffer<Luma<P>, Vec<P>>,
        curr_pixel: img::PixelT<P>,
        ngb_pixel: img::PixelT<P>,
        aux_structure: &T,
    ) -> P,
    queue: &mut VecDeque<(u32, u32)>,
    aux_structure: &T,
    num_threads: u32,
) -> ImageBuffer<Luma<P>, Vec<P>>
where
    T: Clone + Send + Sync + 'static,
{
    let mut parallel_sections: Vec<IWPSection<P>> = Vec::new();
    let mut sections = parallel_img::arrange(base_img, num_threads);

    for section in &mut sections {
        let mut sec_queue: VecDeque<(u32, u32)> = VecDeque::new();
        for (_, p_coords) in queue.iter().enumerate() {
            if img::is_pixel_in_section(*p_coords, &section) {
                //queue.remove(idx);
                sec_queue.push_back(*p_coords);
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
                println!(
                    "Running thread of section {:?}, queue length = {:?}",
                    section.section.start,
                    section.queue.len()
                );
                let mut total_time: u128 = 0;
                let mut count: u128 = 0;
                let mut get_pixel_ngb_total_time = 0;
                let mut inner_loop_total_time = 0;
                while section.queue.len() != 0 {
                    let now = Instant::now();
                    let pixel_coords_abs = section.queue.pop_front().unwrap();
                    let pixel_coords = (
                        pixel_coords_abs.0 - section.section.start.0,
                        pixel_coords_abs.1 - section.section.start.1,
                    );

                    let curr_pixel = section
                        .section
                        .get_relative_pixel(pixel_coords.0, pixel_coords.1);

                    let curr_pixel_abs = img::PixelT {
                        coords: pixel_coords_abs,
                        value: curr_pixel.value,
                    };

                    let now_2 = Instant::now();
                    let pixel_ngbs = img::get_pixel_neighbours(
                        &section.section.slice,
                        pixel_coords,
                        img::ConnTypes::Eight,
                    );
                    let elapsed_2 = now_2.elapsed().as_nanos();

                    let now_3 = Instant::now();
                    for ngb_coord in pixel_ngbs {
                        let ngb_pixel_abs = section.section.get_abs_pixel(ngb_coord.0, ngb_coord.1);

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

                            section.queue.push_back(ngb_pixel_abs.coords);
                        }
                    }
                    let elapsed_3 = now_3.elapsed().as_nanos();

                    let elapsed = now.elapsed().as_nanos();
                    
                    total_time += elapsed;
                    get_pixel_ngb_total_time += elapsed_2;
                    inner_loop_total_time += elapsed_3;
                    count += 1;
                }
                let avg_iter_time = total_time / count;
                let avg_get_pixel_ngb = get_pixel_ngb_total_time / count;
                let avg_inner_loop_time = inner_loop_total_time / count; 
                println!(
                    "Finish thread of section {:?}, total_time = {:?}, count = {:?}, average_iter_time = {:?}, average_get_pixel_ngb = {:?}, avg_inner_loop = {:?}",
                    section.section.start, total_time, count, avg_iter_time, avg_get_pixel_ngb, avg_inner_loop_time
                );
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    });

    println!("Final Stage...");

    let mut queue = VecDeque::new();
    for section in sections.iter() {
        let mut active_border_pixels = get_section_active_borders(base_img, section);
        queue.append(&mut active_border_pixels);
    }

    let mut full_img = parallel_img::get_full_img(base_img.width(), base_img.height(), &sections);

    propagate(
        &mut full_img,
        propagation_condition,
        update_func,
        &mut queue,
        aux_structure,
    );

    return full_img;
}

fn get_section_active_borders<P: Primitive>(
    base_img: &image::ImageBuffer<Luma<P>, Vec<P>>,
    section: &ParallelSection<P>,
) -> VecDeque<(u32, u32)> {
    let mut border_pixels = VecDeque::new();
    let x = section.start.0;
    let y = section.start.1;

    if x != 0 {
        border_pixels.append(&mut img::get_left_border_pixels_coords(&section.slice));
    }

    if y != 0 {
        border_pixels.append(&mut img::get_upper_border_pixels_coords(&section.slice));
    }

    if y + section.height != base_img.height() {
        border_pixels.append(&mut img::get_bottom_border_pixels_coords(&section.slice));
    }

    if x + section.width != base_img.width() {
        border_pixels.append(&mut img::get_right_border_pixels_coords(&section.slice));
    }

    border_pixels
        .iter_mut()
        .for_each(|p| *p = section.get_abs_pixel(p.0, p.1).coords);

    return border_pixels;
}
