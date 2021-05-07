mod point;

use image::{DynamicImage, GenericImageView, ImageBuffer};

extern crate image;

fn load() -> DynamicImage {
    image::open("./data/test.tiff").unwrap()
}

//type Point<T> = [T; 2];

fn main() {
    let ref_img = load();
    let dim = ref_img.dimensions();

    let arguments: [[[f64; 2]; 2]; 3] = [
        [[0., 0.], [0., 64.]],
        [[128., 128.], [128. + 64., 128. + 64.]],
        [[511., 511.], [511. - 64., 511.]],
    ];

    let couples: Vec<[point::Point; 2]> = arguments
        .iter()
        .map(|[source, target]| {
            [
                point::Point::new(source[0], source[1]),
                point::Point::new(target[0], target[1]),
            ]
        })
        .collect();

    let mut imgbuf: image::ImageBuffer<image::Rgba<u8>, _> = ImageBuffer::new(dim.0, dim.1);

    imgbuf.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let point = point::Point::new(x as f64, y as f64);

        let weights: Vec<f64> = couples
            .iter()
            .map(|[_, target]| (1. / point.distance(target).powf(2.)).min(1.))
            .collect();
        let weight_sum: f64 = weights.iter().sum();

        let delta: point::Point = couples.iter().enumerate().fold(
            point::Point::new(0., 0.),
            |delta, (idx, [source, target])| delta + (source - target) * weights[idx],
        );

        let normalized_delta = delta * (1. / weight_sum);
        let result = point + normalized_delta;

        let bounds = point::Point::new(dim.0 as f64, dim.1 as f64);
        if result.inside_bounds(&bounds) {
            let prev_pixel = ref_img.get_pixel(result.x as u32, result.y as u32);
            *pixel = prev_pixel;
        }
    });

    imgbuf.save("./data/output.png").unwrap();
    println!("dimensions {}", dim.0);
}
