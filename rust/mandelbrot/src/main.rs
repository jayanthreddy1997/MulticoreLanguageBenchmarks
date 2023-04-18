use image::RgbImage;
use ndarray::Array3;
use ndarray_linalg::random; // TODO: maybe remove?


fn main() {

    // saving array to image
    let a: Array3<u8> = Array3::zeros((250, 250, 3));
    let b = a.clone().into_raw_vec();
    // println!("{}", a);
    // println!("{:?}", b);
    let image = RgbImage::from_raw(250 as u32, 250 as u32, b)
        .expect("container should have the right size for the image dimensions");
    image.save("out.png");
}
