fn main() {
    let mut my_string = String::from("hello world");

    // length
    println!("length: {}", my_string.len());

    // is empty ?
    println!("is empty? {}", my_string.is_empty());

    // split by white space
    for (i, str) in my_string.split_whitespace().enumerate() {
        println!("str ({}): {}", i+1, str);
    }

    // does it contain ?
    println!("does it contains 'world' ? {}", my_string.contains("world"));

    // push new string
    my_string.push_str(" of Rust");
    println!("{}", my_string);
}
