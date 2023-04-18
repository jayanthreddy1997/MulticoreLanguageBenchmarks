use image::RgbImage;
use std::time::Instant;
use rayon::iter::ParallelIterator;
use rayon::iter::IntoParallelIterator;


fn save_img(raw_data: &Vec<f64>, h: usize, w: usize, filename: &str) {
    let mut data: Vec<u8> = vec![0; h*w*3];

    for i in 0..h*w {
        data[i*3] = (raw_data[i].min(1.0).max(0.0) * 255.0).round() as u8;
        data[i*3+1] = 0;
        data[i*3+2] = 0;
    }

    let image = RgbImage::from_raw(h as u32, w as u32, data)
        .expect("container should have the right size for the image dimensions");
    image.save(filename);
}

fn mandel(c_re: f64, c_im: f64, max_iterations: usize, divergence_threshold: f64) -> f64 {
    let mut count: usize = 0;
    let mut z_re: f64 = 0.0;
    let mut z_im: f64 = 0.0;
    let mut z_re_new: f64;
    let mut z_im_new: f64;

    while count < max_iterations {
        if (z_re*z_re + z_im*z_im) > divergence_threshold {
            break;
        }
        z_re_new = z_re*z_re - z_im*z_im;
        z_im_new = 2.0 * z_re * z_im;
        z_re = z_re_new + c_re;
        z_im = z_im_new + c_im;
        count += 1;
    }
    return count as f64/max_iterations as f64;
}

fn mandelbrot_serial(x_min: f64, x_max: f64, y_min: f64, y_max: f64, img_height: usize,
                     img_width: usize, max_iterations: usize, divergence_threshold: f64) -> Vec<f64> {
    let now = Instant::now();
    let dx: f64 = (x_max - x_min) / (img_width as f64);
    let dy: f64 = (y_max - y_min) / (img_height as f64);

    let res: Vec<f64> = (0..(img_width*img_height)).into_iter()
        .map(|i| {
            mandel(
                x_min + ((i % img_width) as f64) * dx,
                y_min + ((i / img_width) as f64) * dy,
                max_iterations,
                divergence_threshold
            )
        })
        .collect::<Vec<f64>>();

    let elapsed = now.elapsed();
    println!("Mandelbrot serial run time: {}s", elapsed.as_secs_f64());
    return res;
}

fn mandelbrot_parallel(x_min: f64, x_max: f64, y_min: f64, y_max: f64, img_height: usize,
                       img_width: usize, max_iterations: usize, divergence_threshold: f64, n_threads: usize) -> Vec<f64> {
    rayon::ThreadPoolBuilder::new().num_threads(n_threads).build_global().unwrap();

    let now = Instant::now();
    let dx: f64 = (x_max - x_min) / (img_width as f64);
    let dy: f64 = (y_max - y_min) / (img_height as f64);

    let res: Vec<f64> = (0..(img_width*img_height)).into_par_iter()
        .map(|i| {
            mandel(
                x_min + ((i % img_width) as f64) * dx,
                y_min + ((i / img_width) as f64) * dy,
                max_iterations,
                divergence_threshold
            )
        })
        .collect::<Vec<f64>>();

    let elapsed = now.elapsed();
    println!("Mandelbrot parallel run time: {}s", elapsed.as_secs_f64());
    return res;
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

    println!("Running Mandelbrot in serial");
    let mut output_serial: Vec<f64> = mandelbrot_serial(x_min, x_max, y_min, y_max, img_height, img_width, max_iterations, divergence_threshold);
    save_img(&output_serial, img_height, img_width, "mandelbrol_rust_serial.png");

    println!("Running Mandelbrot in parallel");
    let mut output_parallel: Vec<f64> = mandelbrot_parallel(x_min, x_max, y_min, y_max, img_height, img_width, max_iterations, divergence_threshold, n_threads);
    save_img(&output_parallel, img_height, img_width, "mandelbrol_rust_parallel.png");
}
