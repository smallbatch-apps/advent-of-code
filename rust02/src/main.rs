enum Directions {
    Forward,
    Up,
    Down,
}

fn main() {
    let mut depth = 0;
    let mut position = 0;

    let inputs = [
        (Directions::Forward, 5),
        (Directions::Down, 5),
        (Directions::Forward, 8),
        (Directions::Up, 3),
        (Directions::Down, 8),
        (Directions::Forward, 2),
    ];

    for input in inputs.iter() {
        let (dir, num) = input;
        match dir {
            Directions::Forward => position += num,
            Directions::Up => depth -= num,
            Directions::Down => depth += num,
        }
    }

    println!("Current Position: {} down, {} across", depth, position);

    println!("Position Multiplier: {}", depth * position);
}
