fn main() {
    let mut sign1: i32 = -222;
    sign1 = 222;
    let unsign1: u32 = 111;
    const FLT1: f32 = 3.14;
    let chr1: char = 'A';
    let b1: bool = true;

    let mut str1 = String::from("rust");
    str1.push_str(" rover");
    let sliced1: &str = &str1[0..4];

    const tup1: (i32, u32, f32) = (-11, 33, 55.77);
    const arr1: [i32; 5] = [1, -2, 3, 4, 5];
}
