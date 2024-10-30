fn divide(a: f32, b: f32) -> Result<f32, String> {
    //Result<T,E>
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    if b == 0.0 {
        Err(String::from("cannot devide by 0"))
    } else {
        Ok(a / b)
    }
}

fn find_char(s: &str, c: char) -> Option<usize> {
    for (i, ch) in s.chars().enumerate() {
        if ch == c {
            return Some(i);
        }
    }
    None
}
println!("{}", find_char("visxens", 's').unwrap()); //returns the first occurence
