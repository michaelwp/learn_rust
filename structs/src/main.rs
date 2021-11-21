struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    let mut bg = Color { red: 255, green: 0, blue: 0 };

    bg.blue = 125;

    println!("red: {}, green: {}, blue: {}", bg.red, bg.green, bg.blue);
}
