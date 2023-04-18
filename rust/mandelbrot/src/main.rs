use image::RgbImage;
use std::time::Instant;
use std::cmp::{min, max};

fn save_img(raw_data: &mut Vec<f64>, h: usize, w: usize, filename: &str) {
    let mut data: Vec<u8> = vec![0; h*w*3];

    for i in 0..h*w {
        for j in 0..3 {
            data[i*3 + j] = (raw_data[i].min(1.0).max(0.0) * 255.0).round() as u8;
        }
    }

    let image = RgbImage::from_raw(h as u32, w as u32, data)
        .expect("container should have the right size for the image dimensions");
    image.save(filename);
}

fn mandelbrot_serial() {
    let now = Instant::now();

    let elapsed = now.elapsed();
    println!("Mandelbrot serial run time: {}s", elapsed.as_secs_f64());
}

fn mandelbrot_parallel() {
    let now = Instant::now();

    let elapsed = now.elapsed();
    println!("Mandelbrot parallel run time: {}s", elapsed.as_secs_f64());
}

fn main() {
    let n_threads: usize = 10;
    let img_height: usize = 4096;
    let img_width: usize = 4096;
    let x_min: f64 = -2.0;
    let x_max: f64 = 1.0;
    let y_min: f64 = -1.5;
    let y_max: f64 = 1.5;
    let max_iterations = 100;
    let divergence_threshold: f64 = 4.0;

    let mut output: Vec<f64> = vec![0.0; img_height*img_width];
    // mandelbrot_serial(output);
    save_img(&mut output, img_height, img_width, "mandelbrol_rust_serial.png");

    // output = vec![0.0; img_height*img_width];
    // mandelbrot_parallel(output, n_threads);
    // save_img(output);
}
