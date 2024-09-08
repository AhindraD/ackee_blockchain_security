fn main() {
    struct Unit;

    struct Pair(i16, i32);

    struct Point {
        x: f32,
        y: f32,
    }

    let pt1 = Point { x: 1.3, y: 5.6 };

    println!("the coordinates are ({}, {})", pt1.x, pt1.y);
}
