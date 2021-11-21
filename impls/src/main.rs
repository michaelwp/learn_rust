struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area_of_rectangle(&self) -> u32 {
        return self.height * self.width;
    }
}

fn main() {
    let rect = Rectangle{width: 5, height: 5};
    let area = rect.area_of_rectangle();

    println!("area: {} m2", area);
}

