fn main() {
    let mut username = String::from("Avon");
    immute_borrow(&username);
    mutate_borrow(&mut username);
    println!("final username: {}", username);
}

fn immute_borrow(username: &String) {
    println!("immutable borrower, can be multiple: {}", username);
}

fn mutate_borrow(username: &mut String) {
    username.push_str("_mutant");
    println!("mutable borrower, can be only one: {}", username);
}
