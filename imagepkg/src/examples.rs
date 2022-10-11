use image::{GrayImage, ImageBuffer, Luma};

pub fn _gen_same_value_image(width: u32, height: u32, value: u8) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut img = GrayImage::new(width, height);
    for i in 0..width {
        for j in 0..height {
            img.put_pixel(i, j, Luma([value]))
        }
    }

    return img;
}

/*
Gens the 6 x 6 image below
0 0 0 0 0 0
0 1 1 0 0 0
0 1 1 0 0 0
0 0 0 1 1 0
0 0 0 1 1 0
0 0 0 0 0 0
*/
pub fn _gen_example_img() -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut img = _gen_same_value_image(6, 6, 0);

    img.put_pixel(1, 1, Luma([1]));
    img.put_pixel(1, 2, Luma([1]));
    img.put_pixel(2, 1, Luma([1]));
    img.put_pixel(2, 2, Luma([1]));
    img.put_pixel(3, 3, Luma([1]));
    img.put_pixel(3, 4, Luma([1]));
    img.put_pixel(4, 3, Luma([1]));
    img.put_pixel(4, 4, Luma([1]));

    return img;
}

/*
Gens the 10 x 10 image below
08 08 08 08 08 08 08 08 08 08
08 12 12 12 08 08 09 08 09 08
08 12 12 12 08 08 08 09 08 08
08 12 12 12 08 08 09 08 09 08
08 08 08 08 08 08 08 08 08 08
08 09 08 08 08 16 16 16 08 08
08 08 08 09 08 16 16 16 08 08
08 08 09 08 08 16 16 16 08 08
08 09 08 09 08 08 08 08 08 08
08 08 08 08 08 08 09 08 08 08
*/
pub fn _gen_big_marker_img() -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut base_img = _gen_same_value_image(10, 10, 8);

    for i in 1..4 {
        for j in 1..4 {
            base_img.put_pixel(i, j, Luma([12]));
        }
    }

    for i in 5..8 {
        for j in 5..8 {
            base_img.put_pixel(i, j, Luma([16]));
        }
    }

    base_img.put_pixel(1, 5, Luma([9]));
    base_img.put_pixel(1, 8, Luma([9]));
    base_img.put_pixel(2, 7, Luma([9]));
    base_img.put_pixel(3, 6, Luma([9]));
    base_img.put_pixel(3, 8, Luma([9]));
    base_img.put_pixel(6, 1, Luma([9]));
    base_img.put_pixel(6, 3, Luma([9]));
    base_img.put_pixel(6, 9, Luma([9]));
    base_img.put_pixel(7, 2, Luma([9]));
    base_img.put_pixel(8, 1, Luma([9]));
    base_img.put_pixel(8, 3, Luma([9]));

    return base_img;
}

/*
Gens the 10 x 10 image below
10 10 10 10 10 10 10 10 10 10
10 14 14 14 10 10 11 10 11 10
10 14 14 14 10 10 10 11 10 10
10 14 14 14 10 10 11 10 11 10
10 10 10 10 10 10 10 10 10 10
10 11 10 10 10 16 16 16 10 10
10 10 10 11 10 16 16 16 10 10
10 10 11 10 10 16 16 16 10 10
10 11 10 11 10 10 10 10 10 10
10 10 10 10 10 10 11 10 10 10
*/
pub fn _gen_big_mask_img() -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut base_img = _gen_same_value_image(10, 10, 10);

    for i in 1..4 {
        for j in 1..4 {
            base_img.put_pixel(i, j, Luma([14]));
        }
    }

    for i in 5..8 {
        for j in 5..8 {
            base_img.put_pixel(i, j, Luma([18]));
        }
    }

    base_img.put_pixel(1, 5, Luma([11]));
    base_img.put_pixel(1, 8, Luma([11]));
    base_img.put_pixel(2, 7, Luma([11]));
    base_img.put_pixel(3, 6, Luma([11]));
    base_img.put_pixel(3, 8, Luma([11]));
    base_img.put_pixel(6, 1, Luma([11]));
    base_img.put_pixel(6, 3, Luma([11]));
    base_img.put_pixel(6, 9, Luma([11]));
    base_img.put_pixel(7, 2, Luma([11]));
    base_img.put_pixel(8, 1, Luma([11]));
    base_img.put_pixel(8, 3, Luma([11]));

    return base_img;
}

/*
Gens the 10 x 10 image below
10 10 10 10 10 10 10 10 10 10
10 12 12 12 10 10 10 10 10 10
10 12 12 12 10 10 10 10 10 10
10 12 12 12 10 10 10 10 10 10
10 10 10 10 10 10 10 10 10 10
10 10 10 10 10 16 16 16 10 10
10 10 10 10 10 16 16 16 10 10
10 10 10 10 10 16 16 16 10 10
10 10 10 10 10 10 10 10 10 10
10 10 10 10 10 10 10 10 10 10
*/
pub fn _gen_expected_img() -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut base_img = _gen_same_value_image(10, 10, 10);

    for i in 1..4 {
        for j in 1..4 {
            base_img.put_pixel(i, j, Luma([12]));
        }
    }

    for i in 5..8 {
        for j in 5..8 {
            base_img.put_pixel(i, j, Luma([16]));
        }
    }

    return base_img;
}
