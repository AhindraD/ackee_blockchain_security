fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //'a is a lifetime parameter. It is not an actual lifetime but a label representing the relationship between the lifetimes of x, y, and the return value.
    //if any arg goest out of scope, rust throws error on compile, so there's no dangling references and ensuring memory safety.
    return if x.len() > y.len() { x } else { y };
}

fn _main() {
    let string1 = String::from("longer str");
    let res;
    {
        let string2 = String::from("shorter");
        res = longest(&string1.as_str(), &string2.as_str())
        println!("the longest sr is: {}",res)
    }// string2 goes out of scope here
}
