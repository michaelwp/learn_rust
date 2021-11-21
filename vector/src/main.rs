fn main() {

    // other way to declare vector `Vec<i32> = Vec::new()`
    let mut my_vector = vec!(1,2,3,4);

    // push new element to vector
    my_vector.push(10);

    // remove element in a vector
    my_vector.remove(1);

    // print an element in a vector
    println!("element[2] : {} \n", my_vector[2]);

    // print all elements in a vector
    for (i, n) in my_vector.iter().enumerate() {
        println!("element[{}]: {}", i, n);
    }
}
