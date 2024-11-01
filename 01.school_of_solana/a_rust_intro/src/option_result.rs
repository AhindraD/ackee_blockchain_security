fn divide(a: f32, b: f32) -> Result<f32, String> {
    //Result<T,E>
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    //Use Result when errors need handling, allowing you to carry information about what went wrong.
    if b == 0.0 {
        Err(String::from("cannot devide by 0"))
    } else {
        Ok(a / b)
    }
}

fn find_char(s: &str, c: char) -> Option<usize> {
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    // Use Option when absence of a value is expected and doesn't necessarily represent an error.
    for (i, ch) in s.chars().enumerate() {
        if ch == c {
            return Some(i);
        }
    }
    None
}
println!("{}", find_char("visxens", 's').unwrap()); //returns the first occurence
