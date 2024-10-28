fn main() {
    let mut username = String::from("Avon");
    immute_borrow(&username);
    mutate_borrow(&mut username);
    println!("final username: {}", username);

    let s3: i8 = 127;
    {
        let s4 = s3; //owener changed
        println!("{}", s4);
    } //s4 dropped, memory freed
    println!("{}", s3); //s3 not valid, memory already freed as owner s4 dropped
}

fn immute_borrow(username: &String) {
    println!("immutable borrower, can be multiple: {}", username);
}

fn mutate_borrow(username: &mut String) {
    username.push_str("_mutant");
    println!("mutable borrower, can be only one: {}", username);
}
