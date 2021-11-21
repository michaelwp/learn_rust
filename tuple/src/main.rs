fn main() {
    let tupples = (20, "rust", 3.4, false, (1,"2"));

    println!("{}", (tupples.4).1);

    let (number, string, float, boolean) =
        (tupples.0, tupples.1, tupples.2, tupples.3);

    println!("{},{},{},{}", number, string, float, boolean);
}
