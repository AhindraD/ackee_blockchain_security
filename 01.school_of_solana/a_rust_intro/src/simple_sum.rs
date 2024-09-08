fn main() {
    let a: f32 = 1.3;
    let b = 5.6;
    let res = sum(a, b);
    println!("Hello, world! {}", res);
}

fn sum(a: f32, b: f32) -> f32 {
    return a + b;
}
