use crate::{img, iwp, parallel_img};
use image::Luma;
use std::{collections::HashSet, thread};

fn update_pixel(
    pixel_coords: (u32, u32),
    mask: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
    marker: &mut image::ImageBuffer<Luma<u8>, Vec<u8>>,
) {
    let pixel_ngbs = img::get_pixel_neighbours(marker, pixel_coords, img::ConnTypes::Eight);

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
    mask: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
    marker: &mut image::ImageBuffer<Luma<u8>, Vec<u8>>,
) -> Vec<(u32, u32)> {
    let width = marker.width();
    let height = marker.height();
    let mut queue = HashSet::new();

    for i in 0..height {
        for j in 0..width {
            update_pixel((j, i), mask, marker);
        }
    }

    for i in (0..height).rev() {
        for j in (0..width).rev() {
            let pixel_coords = (j, i);

            update_pixel((j, i), mask, marker);
            let pixel_marker = marker.get_pixel(pixel_coords.0, pixel_coords.1);
            let pixel_value = pixel_marker.0[0];

            let pixel_ngbs = img::get_pixel_neighbours(marker, pixel_coords, img::ConnTypes::Eight);

            for ngb_coord in pixel_ngbs {
                let ngb = marker.get_pixel(ngb_coord.0, ngb_coord.1);
                let ngb_mask = mask.get_pixel(ngb_coord.0, ngb_coord.1);

                if (ngb.0[0] < pixel_value) && (ngb.0[0] < ngb_mask.0[0]) {
                    queue.insert(ngb_coord);
                }
            }
        }
    }

    let queue = Vec::from_iter(queue); //check complexity of this operation later
    return queue;
}

fn get_initial_pixels_parallel(
    mask: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
    marker: &mut image::ImageBuffer<Luma<u8>, Vec<u8>>,
    num_threads: u32,
) -> (image::ImageBuffer<Luma<u8>, Vec<u8>>, Vec<(u32, u32)>) {
    //let mask_arc = Arc::new(mask.clone());
    let mut sections = parallel_img::arrange(marker, num_threads);
    let mask_sections = parallel_img::arrange(&mut mask.clone(), num_threads);

    for section in sections.iter() {
        println!(
            "Creating of section {:?}, width = {:?}, height = {:?}",
            section.start, section.width, section.height
        );
    }

    let queue = thread::scope(|s| {
        let mut handles = vec![];
        let mut count = 0;
        for section in &mut sections {
            let mask_section_slice = &mask_sections.get(count).unwrap().slice;

            let handle = s.spawn(move || {
                let mut relative_queue =
                    get_initial_pixels(&mask_section_slice, &mut section.slice);
                relative_queue
                    .iter_mut()
                    .for_each(|p| *p = (p.0 + section.start.0, p.1 + section.start.1));
                return relative_queue;
            });

            handles.push(handle);

            count += 1;
        }

        // Chech if creating a HashSet is really necessary - since it adds a little overhead
        let mut queue = HashSet::new();

        for handle in handles {
            let sec_q = handle.join().unwrap();
            for el in sec_q {
                queue.insert(el);
            }
            //queue.append(&mut sec_q);
        }

        return queue;
    });

    let full_img = parallel_img::get_full_img(marker.width(), marker.height(), &sections);

    return (full_img, Vec::from_iter(queue));
}

fn propagation_condition(
    _marker: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
    curr_pixel: img::PixelT<u8>,
    ngb_pixel: img::PixelT<u8>,
    mask: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
) -> bool {
    let mask_ngb = mask.get_pixel(ngb_pixel.coords.0, ngb_pixel.coords.1);
    if (ngb_pixel.value < curr_pixel.value) && (mask_ngb.0[0] != ngb_pixel.value) {
        return true;
    }

    return false;
}

fn update_func(
    _marker: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
    curr_pixel: img::PixelT<u8>,
    ngb_pixel: img::PixelT<u8>,
    mask: &image::ImageBuffer<Luma<u8>, Vec<u8>>,
) -> u8 {
    let mask_ngb = mask.get_pixel(ngb_pixel.coords.0, ngb_pixel.coords.1);
    return std::cmp::min(curr_pixel.value, mask_ngb.0[0]);
}

pub fn morph_reconstruction(
    mask: &mut image::ImageBuffer<Luma<u8>, Vec<u8>>,
    marker: &mut image::ImageBuffer<Luma<u8>, Vec<u8>>,
) {
    let mut initial_queue = get_initial_pixels(&mask, marker);
    iwp::propagate(
        marker,
        propagation_condition,
        update_func,
        &mut initial_queue,
        mask,
    );
}

pub fn morph_reconstruction_parallel(
    mask: &mut image::ImageBuffer<Luma<u8>, Vec<u8>>,
    marker: &mut image::ImageBuffer<Luma<u8>, Vec<u8>>,
    num_threads: u32,
) -> image::ImageBuffer<image::Luma<u8>, Vec<u8>> {
    let (mut base_img, mut initial_queue) = get_initial_pixels_parallel(&mask, marker, num_threads);
    let result = iwp::propagate_parallel(
        &mut base_img,
        propagation_condition,
        update_func,
        &mut initial_queue,
        mask,
        num_threads,
    );
    return result;
}

mod tests {

    #![allow(unused_imports)]

    use std::time::Instant;

    use crate::examples::*;
    use crate::format;
    use crate::img::is_pixel_in_section;
    use crate::iwp;
    use crate::mr::*;
    use image::io::Reader as ImageReader;

    #[test]
    fn test_get_initial_pixels() {
        let mask = _gen_example_img();

        let mut marker = _gen_same_value_image(6, 6, 0);
        marker.put_pixel(4, 4, Luma([1]));

        let mut initial = get_initial_pixels(&mask, &mut marker);
        let mut expected = vec![(1, 1), (2, 1), (2, 2), (1, 2)];

        initial.sort();
        expected.sort();

        assert_eq!(initial, expected);
    }

    #[test]
    fn test_propagation_phase() {
        let mut mask = _gen_big_mask_img();
        let mut marker = _gen_big_marker_img();

        // did not use the get_initial_pixels function here because it does all the job
        let mut initial: Vec<(u32, u32)> = Vec::new();
        for i in 0..10 {
            for j in 0..10 {
                let pixel = marker.get_pixel(i, j);
                if pixel.0[0] != 8 {
                    initial.push((i, j));
                }
            }
        }

        iwp::propagate(
            &mut marker,
            propagation_condition,
            update_func,
            &mut initial,
            &mut mask,
        );

        assert_eq!(marker, _gen_expected_img());
    }

    #[test]
    fn test_get_initial_pixels_parallel() {
        let num_threads = 8;

        let img_mask = ImageReader::open("./tests/imgs/mr/mask.png")
            .unwrap()
            .decode()
            .unwrap();
        let mask = img_mask.to_luma8();

        let img_marker = ImageReader::open("./tests/imgs/mr/marker.png")
            .unwrap()
            .decode()
            .unwrap();
        let mut marker = img_marker.to_luma8();

        let (mut marker_new, mut initial) =
            get_initial_pixels_parallel(&mask, &mut marker, num_threads);

        let mut marker_new_sections = parallel_img::arrange(&mut marker_new, num_threads);

        let mut exp_sections = parallel_img::arrange(&mut marker, num_threads);
        let mut exp_queue = HashSet::new();
        let mut count = 0;
        for section in &mut exp_sections {
            let marker_new_sec = marker_new_sections.get_mut(count).unwrap().slice.clone();
            let exp_sec_initial = get_initial_pixels(&mask, &mut section.slice);
            for val in exp_sec_initial {
                exp_queue.insert(section.get_abs_pixel(val.0, val.1).coords);
            }

            assert_eq!(section.slice, marker_new_sec);

            count += 1;
        }

        // WARNING: THIS IS DOING NOTHING -> .sort() returns () -> always true
        assert_eq!(Vec::from_iter(exp_queue).sort(), initial.sort());
    }

    #[test]
    fn test_propagation_phase_parallel() {
        let num_threads = 8;

        let img_mask = ImageReader::open("./tests/imgs/mr/mask.png")
            .unwrap()
            .decode()
            .unwrap();
        let mut mask = img_mask.to_luma8();

        let img_marker = ImageReader::open("./tests/imgs/mr/marker.png")
            .unwrap()
            .decode()
            .unwrap();
        let mut marker = img_marker.to_luma8();

        // did not use the get_initial_pixels function here because it does all the job
        let mut initial = get_initial_pixels(&mask, &mut marker);

        let mut result = iwp::propagate_parallel(
            &mut marker,
            propagation_condition,
            update_func,
            &mut initial,
            &mut mask,
            num_threads,
        );

        let mut result_sections = parallel_img::arrange(&mut result, num_threads);
        let mut exp_sections = parallel_img::arrange(&mut marker, num_threads);
        let mut mask_sections = parallel_img::arrange(&mut mask, num_threads);

        let mut count = 0;
        for section in &mut exp_sections {
            let result_sec = result_sections.get_mut(count).unwrap();
            let result_sec_slice = result_sec.slice.clone();
            let mut mask_slice = mask_sections.get_mut(count).unwrap().slice.clone();

            let mut exp_sec_initial = vec![];

            for c in initial.clone() {
                if is_pixel_in_section(c, section) {
                    exp_sec_initial.push((c.0 - section.start.0, c.1 - section.start.1));
                }
            }

            iwp::propagate(
                &mut section.slice,
                propagation_condition,
                update_func,
                &mut exp_sec_initial,
                &mut mask_slice,
            );

            assert_eq!(section.slice, result_sec_slice);

            count += 1;
        }
    }

    #[test]
    fn test_propagation_phase_parallel_time() {
        let img_mask = ImageReader::open("./tests/imgs/mr/50-percent-mask.jpg")
            .unwrap()
            .decode()
            .unwrap();
        let mut mask = img_mask.to_luma8();

        let img_marker = ImageReader::open("./tests/imgs/mr/50-percent-marker.jpg")
            .unwrap()
            .decode()
            .unwrap();
        let mut marker = img_marker.to_luma8();

        //print_image_by_row(&marker);

        let num_threads = 15;
        let (mut marker_new, mut initial) =
            get_initial_pixels_parallel(&mask, &mut marker, num_threads);

        //print_image_by_row(&markerr);

        marker_new.save("marker_new.jpg").unwrap();

        let result = iwp::propagate_parallel(
            &mut marker_new,
            propagation_condition,
            update_func,
            &mut initial,
            &mut mask,
            num_threads,
        );

        result.save("result.jpg").unwrap();
    }

    #[test]
    fn test_propagation_phase_parallel_2() {
        let now = Instant::now();

        let img_mask = ImageReader::open("./tests/imgs/mr/50-percent-mask.jpg")
            .unwrap()
            .decode()
            .unwrap();
        let mut mask = img_mask.to_luma8();

        let img_marker = ImageReader::open("./tests/imgs/mr/50-percent-marker.jpg")
            .unwrap()
            .decode()
            .unwrap();
        let mut marker = img_marker.to_luma8();

        let num_threads = 12;
        let (mut marker_new, mut initial) =
            get_initial_pixels_parallel(&mask, &mut marker, num_threads);

        let result = iwp::propagate_parallel(
            &mut marker_new,
            propagation_condition,
            update_func,
            &mut initial,
            &mut mask,
            num_threads,
        );

        println!("parallel = {:?}", now.elapsed().as_secs_f32());
        let now_2 = Instant::now();

        let mut initial = get_initial_pixels(&mask, &mut marker);
        iwp::propagate(
            &mut marker,
            propagation_condition,
            update_func,
            &mut initial,
            &mut mask,
        );

        println!("sequential = {:?}", now_2.elapsed().as_secs_f32());

        assert_eq!(marker, result);
    }
}
