fn _main() {
    let mut _sign1: i32 = -222;
    _sign1 = 222;
    let _unsign1: u32 = 111;
    const FLT1: f32 = 3.14;
    let _chr1: char = 'A';
    let _b1: bool = true;

    let mut str1 = String::from("rust");
    str1.push_str(" rover");
    let _sliced1: &str = &str1[0..4];

    let _tup1: (i32, u32, f32) = (-11, 33, 55.77);
    let _arr1: [i32; 5] = [1, -2, 3, 4, 5];
}
