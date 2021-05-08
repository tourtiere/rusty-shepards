mod point;

extern crate clap;
extern crate image;
extern crate itertools;
extern crate regex;

use clap::{App, Arg, ArgMatches};
use image::{GenericImageView, ImageBuffer};

fn extract_commandline<'a>() -> ArgMatches<'a> {
    App::new("Shepard's image transformation")
        .author("Tourtiere <https://github.com/tourtiere>")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("POINTS")
                .short("p")
                .long("points")
                .value_name("Point matches")
                .help("Sets a list of matching points")
                .takes_value(true)
                .required(true),
        )
        .get_matches()
}

type Couple = (point::Point, point::Point);

fn extract_points(argument: &str) -> Vec<Couple> {
    let re = regex::Regex::new(r"[ \t]+").unwrap();
    let points: Vec<point::Point> = re
        .split(argument)
        .into_iter()
        .map(|element| {
            let split = element.split(",");
            let parts = split.collect::<Vec<&str>>();
            let x: i32 = parts[0].parse().unwrap();
            let y: i32 = parts[1].parse().unwrap();
            point::Point::new(x as f64, y as f64)
        })
        .collect();
    let mut couples: Vec<Couple> = Vec::new();
    points.iter().enumerate().for_each(|(idx, _)| {
        if idx % 2 == 1 {
            couples.push((points[idx - 1], points[idx]));
        }
    });
    couples
}

fn classic_shepards(point: &point::Point, couples: &Vec<Couple>) -> point::Point {
    let (delta, weight_sum): (point::Point, f64) = couples.iter().fold(
        (point::Point::new(0., 0.), 0_f64),
        |(delta, weight_sum), (source, target)| {
            let weight = (1. / point.distance(&target).powf(2.)).min(1.);
            (delta + (source - target) * weight, weight_sum + weight)
        },
    );
    point + &(delta * (1. / weight_sum))
}

fn main() {
    let matches = extract_commandline();
    let input_path = matches.value_of("INPUT").unwrap();
    let output_path = matches.value_of("OUTPUT").unwrap();
    let points_raw = matches.value_of("POINTS").unwrap();
    let ref_img = image::open(input_path).unwrap();

    let dim = ref_img.dimensions();

    let couples = extract_points(points_raw);

    let mut imgbuf: image::ImageBuffer<image::Rgba<u8>, _> = ImageBuffer::new(dim.0, dim.1);

    imgbuf.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let point = point::Point::new(x as f64, y as f64);
        let target_point = classic_shepards(&point, &couples);
        let bounds = point::Point::new(dim.0 as f64, dim.1 as f64);
        if !target_point.inside_bounds(&bounds) {
            return;
        }
        *pixel = ref_img.get_pixel(target_point.x as u32, target_point.y as u32);
    });

    imgbuf.save(output_path).unwrap();
}
