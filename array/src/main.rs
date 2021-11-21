fn main() {
    // !!!important element in array must have same data type
    let numbers = [1, 2, 3, 4, 5, 6];
    let colors: [&str; 3] = ["red", "green", "blue"];
    let reps = ["rust"; 100];

    for n in 0..numbers.len() {
        println!("number: {}", numbers[n]);
    }

    for c in colors.iter() {
        println!("color: {}", c);
    }

    // this will print a hundred of "rust" words
    for (i, r) in reps.iter().enumerate() {
        println!("rep: {}.{}", i+1, r);
    }
}