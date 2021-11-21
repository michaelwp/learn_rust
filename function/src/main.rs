fn main() {

    // call print numbers to function
    print_numbers_to(20);
}

fn print_numbers_to(num: u32) {

    // print number from 0 to n
    for n in 0..num {

        // call is_even function
        if is_even(n) {
            println!("even number: {}", n);
        }
    }
}

fn is_even(num: u32) -> bool {
    return num % 2 == 0;
}
