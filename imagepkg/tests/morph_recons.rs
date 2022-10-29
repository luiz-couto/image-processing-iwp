use image;
use image::io::Reader as ImageReader;
use imagepkg;

use crate::common::rmse_between_imgs;

mod common;

// Images are considered very smimilar if the mean difference between
// their pixels are less than 0.5
const IMG_DIFF_THRESHOLD: f64 = 0.5;

#[test]
fn test_morphological_reconstruction() {
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

    let dimensions = mask.dimensions();
    println!("dimensions: {:?}", dimensions);

    imagepkg::morph_reconstruction(&mut mask, &mut marker);

    let expected = ImageReader::open("./tests/imgs/mr/result_matlab.png")
        .unwrap()
        .decode()
        .unwrap()
        .to_luma8();

    let rmse = rmse_between_imgs(&expected, &marker);

    assert!(rmse <= IMG_DIFF_THRESHOLD);
    //marker.save("./tests/imgs/mr/result.png").unwrap();
}
