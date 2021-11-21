fn main() {
    let x = 10;

    // code block / scope
    // code inside this block `{}` can access all variables outside the block,
    // but the code outside this block can't access any variables inside this block.
    {
        let y = 5;

        // will print x : 10, y: 5
        println!("x : {}, y: {}", x, y);
    }

    // will return error
    // println!("x : {}, y: {}", x, y);
}
