use crate::{examples::_gen_same_value_image, PixelT};
use image::{imageops, ImageBuffer, Luma, Primitive};

#[derive(Clone, Debug)]
pub struct ParallelSection<P: Primitive> {
    pub start: (u32, u32),
    pub width: u32,
    pub height: u32,
    pub slice: image::ImageBuffer<Luma<P>, Vec<P>>,
}

#[derive(Clone, Debug)]
pub struct ParallelImg<P: Primitive> {
    pub sections: Vec<ParallelSection<P>>,
    pub witdh: u32,
    pub height: u32,
}

pub fn get_full_img<P: Primitive>(
    width: u32,
    height: u32,
    sections: &Vec<ParallelSection<P>>,
) -> image::ImageBuffer<Luma<P>, Vec<P>> {
    let mut img: ImageBuffer<Luma<P>, Vec<P>> = ImageBuffer::new(width, height);
    for ps in sections {
        for i in 0..ps.height {
            for j in 0..ps.width {
                let pixel = ps.get_abs_pixel(j, i);
                img.put_pixel(pixel.coords.0, pixel.coords.1, Luma([pixel.value]));
            }
        }
    }
    return img;
}

pub fn arrange<P: Primitive + 'static>(
    img: &mut image::ImageBuffer<Luma<P>, Vec<P>>,
    num_sections: u32,
) -> Vec<ParallelSection<P>> {
    let mut sections: Vec<ParallelSection<P>> = Vec::new();
    let columns = (num_sections as f32).sqrt().ceil() as u32;
    let full_rows = num_sections / columns;
    let orphans = num_sections % columns;

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

            sections.push(ParallelSection {
                start: (x * base_width, y * base_height),
                width,
                height,
                slice: imageops::crop(img, x * base_width, y * base_height, width, height)
                    .to_image(),
            });
        }
    }

    if orphans > 0 {
        let orphan_width = img.width() / orphans;
        let orphan_width_leftover = img.width() % orphans;
        let y = full_rows;
        for x in 0..orphans {
            let width = if x == orphans - 1 {
                orphan_width + orphan_width_leftover
            } else {
                orphan_width
            };

            sections.push(ParallelSection {
                start: (x * orphan_width, y * base_height),
                width,
                height: base_height + height_leftover,
                slice: imageops::crop(
                    img,
                    x * orphan_width,
                    y * base_height,
                    width,
                    base_height + height_leftover,
                )
                .to_image(),
            });
        }
    }

    return sections;
}

impl<P: Primitive> ParallelSection<P> {
    pub fn get_relative_pixel(&self, x: u32, y: u32) -> PixelT<P> {
        PixelT {
            coords: (x, y),
            value: self.slice.get_pixel(x, y).0[0],
        }
    }

    pub fn get_abs_pixel(&self, x: u32, y: u32) -> PixelT<P> {
        PixelT {
            coords: (x + self.start.0, y + self.start.1),
            value: self.slice.get_pixel(x, y).0[0],
        }
    }
}

mod tests {
    #![allow(unused_imports)]
    use crate::{
        examples::_gen_seq_img,
        format::print_image_by_row,
        parallel_img::{self, *},
    };

    #[test]
    fn test_parallel_img_assemble() {
        let mut base_img = _gen_seq_img();

        let sections = parallel_img::arrange(&mut base_img, 4);

        let assembled_img =
            parallel_img::get_full_img(base_img.width(), base_img.height(), &sections);
        assert_eq!(base_img, assembled_img);
    }
}
