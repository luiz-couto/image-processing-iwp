use image;
use image::io::Reader as ImageReader;
use imagepkg;

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

    marker.save("./tests/imgs/mr/result.png").unwrap();
}
