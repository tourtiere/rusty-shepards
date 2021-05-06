type Point = [i64; 2];

fn map_axis<F: Fn(usize) -> i64>(f: F) -> Point {
    return [f(0), f(1)];
}

fn exp_point(point: Point) -> Point {
    map_axis(|axis| point[axis].pow(2))
}
