fn _main() {
    let my_array = [1, 2, 3, 4, 5];
    //iteration
    for n in my_array {
        println!("count: {}", n);
    }
    for i in 3..9{
        println!("{}",i)
    }
    

    //infinity loop
    loop {
        println!("hello");
        break;
    }

    //while loop
    let mut i: u32 = 0;

    while i <= 5 {
        println!("i: {}", i);
        i += 1;
    }
}
