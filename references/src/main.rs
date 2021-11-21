fn main() {
    let mut x = 10;

    {
        // xr is a mutable references(&) of x
        // !!!important x must set mutable too in order to change the value
        let xr = &mut x;

        // dereference(*) xr to get the x value then add 1
        *xr += 1;

    }

    println!("x is {}", x);
}
