fn main() {
    let mut x = 0;
    let y = 50;

    loop {
        x += 1;

        if x > y {
            break
        }

        println!("x value is {}", x);
    }
}
