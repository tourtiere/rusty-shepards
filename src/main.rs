use image::{DynamicImage, GenericImageView, ImageBuffer};
use std::ops::{Index, IndexMut};

extern crate image;

fn load() -> DynamicImage {
    image::open("./data/koala.gif").unwrap()
}

type Point<T> = [T; 2];

fn map_point<T, F: Fn(usize) -> T>(f: F) -> Point<T> {
    return [f(0), f(1)];
}

fn main() {
    let ref_img = load();
    let dim = ref_img.dimensions();

    let arguments: [[Point<f64>; 2]; 2] = [
        /*
        [[0., 0.], [0., 0.]],
        [[100., 100.], [100., 400.]],
        [[510., 510.], [510., 510.]],
        */
        [[30., 11.], [20., 11.]],
        [[48., 29.], [58., 29.]],
    ];

    let mut imgbuf: image::ImageBuffer<image::Rgba<u8>, _> = ImageBuffer::new(dim.0, dim.1);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let point: Point<f64> = [x as f64, y as f64];

        let weights: Vec<f64> = arguments
            .iter()
            .map(|[source, _]| {
                let weight = (source[0] - point[0]).powf(2.) + (source[1] - point[1]).powf(2.);
                if weight < 1. {
                    return 1.;
                };
                return 1. / weight;
            })
            .collect();

        let delta: Point<f64> =
            arguments
                .iter()
                .enumerate()
                .fold([0., 0.], |delta, (idx, [source, target])| {
                    map_point(|axis| delta[axis] + (source[axis] - target[axis]) * weights[idx])
                });

        let weight_sum: f64 = weights.iter().sum();
        let normalized_delta = map_point(|axis| delta[axis] / weight_sum);
        let result: Point<f64> = map_point(|axis| (normalized_delta[axis] + point[axis]));

        if result[0] < dim.0 as f64
            && result[0] >= 0.
            && result[1] < dim.1 as f64
            && result[1] >= 0.
        {
            let prev_pixel = ref_img.get_pixel(result[0] as u32, result[1] as u32);
            *pixel = prev_pixel;
        }
    }

    imgbuf.save("./data/output.jpg").unwrap();
    println!("dimensions {}", dim.0);
}
