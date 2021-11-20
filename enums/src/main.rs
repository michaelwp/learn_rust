enum Direction {
    North, South, East, West
}

fn main() {
    let car_direction: Direction = Direction::North;

    match car_direction {
        Direction::East => println!("car heading east"),
        Direction::West => println!("car heading west"),
        Direction::North => println!("car heading north"),
        Direction::South => println!("car heading south"),
    }

}
