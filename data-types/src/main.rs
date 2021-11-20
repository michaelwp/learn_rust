fn main() {

    // the tuple type
    let t: (i32, [char; 2], u32, f32, bool, &str) = (-10, ['x','y'], 100, 10.5, true, "tes");
    println!("the value of i32: {}", t.0);
    println!("the value of array: [{},{}]", t.1[0], t.1[1]);
    println!("the value of u32: {}", t.2);
    println!("the value of f32: {}", t.3);
    println!("the value of bool: {}", t.4);
    println!("the value of str: {}", t.5);
}
