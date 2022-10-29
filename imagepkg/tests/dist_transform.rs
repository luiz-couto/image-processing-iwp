use image;
use image::io::Reader as ImageReader;
use imagepkg;

#[test]
fn test_distance_transform() {
    let img = ImageReader::open("./tests/imgs/dist_transform/bin_img.png")
        .unwrap()
        .decode()
        .unwrap();
    let img = img.to_luma8();

    let mut bin_img = imagepkg::convert_to_binary(&img);

    let res = imagepkg::dist_transform(&mut bin_img, imagepkg::DistTypes::Euclidean);

    res.save("./tests/imgs/dist_transform/result.png").unwrap();
}
