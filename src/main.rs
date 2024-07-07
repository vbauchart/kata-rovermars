fn main() {
    println!("Hello, world!");
}

struct Rover {
    direction: Direction,
    position: Position,
}

impl Rover {
    fn initial() -> Self {
        Self {
            direction: Direction::N,
            position: Position { x: 0, y: 0 },
        }
    }

    fn new(position: Position, direction: Direction) -> Self{
        Self{
            direction,
            position
        }
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        };
    }
}

#[derive(PartialEq, Eq)]
struct Position {
    x: u8,
    y: u8,
}

#[derive(PartialEq, Eq)]
enum Direction {
    N,
    S,
    E,
    W,
}

#[cfg(test)]
mod tests {
    use crate::{Direction, Position, Rover};

    #[test]
    fn had_initial_position() {
        let rover = Rover::initial();
        let initial = Position { x: 0, y: 0 };

        assert!(rover.position == initial)
    }

    #[test]
    fn had_initial_direction() {
        let rover = Rover::initial();
        let initial = Direction::N;

        assert!(rover.direction == initial);
    }

    #[test]
    fn turn_right_from_north_is_east() {
        let mut rover = Rover::new(
            Position { x: 0, y: 0 },
            Direction::N,
        );

        rover.turn_right();

        assert!(rover.direction == Direction::E);
    }
    #[test]
    fn turn_right_from_east_is_south() {
        let mut rover = Rover::new(
            Position { x: 0, y: 0 },
            Direction::E,
        );

        rover.turn_right();

        assert!(rover.direction == Direction::S);
    }
    #[test]
    fn turn_right_from_south_is_west() {
        let mut rover = Rover::new(
            Position { x: 0, y: 0 },
            Direction::S,
        );

        rover.turn_right();

        assert!(rover.direction == Direction::W);
    }
    #[test]
    fn turn_right_from_west_is_north() {
        let mut rover = Rover::new(
            Position { x: 0, y: 0 },
            Direction::W,
        );

        rover.turn_right();

        assert!(rover.direction == Direction::N);
    }
}
