struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    let bg = Color { red: 255, green: 0, blue: 0 };

    print_the_color(&bg);
    print_the_color(&bg);
}

fn print_the_color(c: &Color) {
    println!("red: {}, green: {}, blue: {}", c.red, c.green, c.blue);
}