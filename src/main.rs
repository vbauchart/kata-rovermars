pub mod rover;

use crate::rover::Rover;

fn main() {
    let mut rover = Rover::initial();

    rover.turn_right();
}
