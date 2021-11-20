fn main() {

    // range of numbers
    let numbers = 25..50;
    for i in numbers {
        println!("the number is {}", i);
    }

    println!("");

    // array of strings
    let animals = ["tiger","bird","elephant","horse","tortoise"];
    for a in animals.iter() {
        println!("the animal is {}", a);
    }

    println!("");

    // print with index of array
    for (i, a) in animals.iter().enumerate() {
        println!("the animal is {}.{}", (i+1), a);
    }
}
