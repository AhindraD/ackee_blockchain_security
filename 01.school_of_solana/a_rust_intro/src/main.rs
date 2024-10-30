//https://github.com/Ackee-Blockchain/school-of-solana/tree/master/2.lesson

mod data_types;
mod enum_basic;
mod lifetimes_rs;
mod loops;
mod option_result;
mod ownership_borrow;
mod structs_basic;
mod trait_basic;

fn main() {
    fn find_char(s: &str, c: char) -> Option<usize> {
        for (i, ch) in s.chars().enumerate() {
            if ch == c {
                return Some(i);
            }
        }
        None
    }
    println!("{}", find_char("visxens", 's').unwrap());
}
