enum Direction {
    North,
    South,
    East,
    West
}

fn movement(d: Direction) {
    match d {
        Direction::North => println!("Moving North"),
        Direction::South => println!("Moving South"),
        Direction::East => println!("Moving East"),
        Direction::West => println!("Moving West")
    }

}

pub fn run() {
    movement(Direction::North);
    movement(Direction::South);
    movement(Direction::East);
    movement(Direction::West);
}