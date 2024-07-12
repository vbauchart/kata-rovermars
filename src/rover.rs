pub struct Rover {
    direction: Direction,
    position: Position,
}

impl Rover {
    pub fn initial() -> Self {
        Self {
            direction: Direction::N,
            position: Position { x: 0, y: 0 },
        }
    }

    fn new(position: Position, direction: Direction) -> Self {
        Self {
            direction,
            position,
        }
    }

    pub fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        };
    }

    pub fn move_forward(&mut self) {
        self.position = match self.direction {
            Direction::N => self.position.go_up(),
            Direction::S => self.position.go_down(),
            Direction::E => self.position.go_right(),
            Direction::W => self.position.go_left(),
        }
    }

    fn move_forward_with_limit(&mut self, limit: Position) {
        let can_move = match self.direction {
            Direction::N => {
                if self.position.x == 0 {
                    false
                } else {
                    true
                }
            }
            Direction::S => {
                if self.position.x == limit.x {
                    false
                } else {
                    true
                }
            }
            Direction::E => {
                if self.position.y == 0 {
                    false
                } else {
                    true
                }
            }
            Direction::W => {
                if self.position.y == limit.y {
                    false
                } else {
                    true
                }
            }
        };

        if can_move {
            self.move_forward();
        }
    }
}

impl Position {
    fn go_up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn go_right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }
    fn go_down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn go_left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }
}

#[derive(PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
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
    use crate::rover::{Direction, Position, Rover};

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
        let mut rover = Rover::new(Position { x: 0, y: 0 }, Direction::N);

        rover.turn_right();

        assert!(rover.direction == Direction::E);
    }
    #[test]
    fn turn_right_from_east_is_south() {
        let mut rover = Rover::new(Position { x: 0, y: 0 }, Direction::E);

        rover.turn_right();

        assert!(rover.direction == Direction::S);
    }
    #[test]
    fn turn_right_from_south_is_west() {
        let mut rover = Rover::new(Position { x: 0, y: 0 }, Direction::S);

        rover.turn_right();

        assert!(rover.direction == Direction::W);
    }
    #[test]
    fn turn_right_from_west_is_north() {
        let mut rover = Rover::new(Position { x: 0, y: 0 }, Direction::W);

        rover.turn_right();

        assert!(rover.direction == Direction::N);
    }

    #[test]
    fn move_forward_where_north() {
        let mut rover = Rover::new(Position { x: 0, y: 0 }, Direction::N);

        rover.move_forward();

        assert!(rover.position == Position { x: 0, y: 1 })
    }

    #[test]
    fn move_forward_where_north_1() {
        let mut rover = Rover::new(Position { x: 0, y: 1 }, Direction::N);

        rover.move_forward();

        assert!(rover.position == Position { x: 0, y: 2 })
    }

    #[test]
    fn move_forward_where_east() {
        let mut rover = Rover::new(Position { x: 0, y: 0 }, Direction::E);

        rover.move_forward();

        assert!(rover.position == Position { x: 1, y: 0 })
    }

    #[test]
    fn move_forward_where_south() {
        let mut rover = Rover::new(Position { x: 0, y: 0 }, Direction::S);

        rover.move_forward();

        assert!(rover.position == Position { x: 0, y: -1 })
    }

    #[test]
    fn move_forward_with_limit_() {
        let mut rover = Rover::initial();

        rover.move_forward_with_limit(Position { x: 0, y: 0 });

        assert!(rover.position == Position { x: 0, y: 0 })
    }

    #[test]
    fn move_forward_north_with_limit_reached() {
        let mut rover = Rover::new(Position { x: 0, y: 0 }, Direction::N);

        rover.move_forward_with_limit(Position { x: 10, y: 10 });

        assert!(rover.position == Position { x: 0, y: 0 })
    }
}
