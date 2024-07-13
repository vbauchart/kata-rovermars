pub mod map;
pub mod rover;

use crate::map::Map;
use crate::rover::Rover;

fn main() {
    let mut rover = Rover::initial();

    rover.turn_right();

    let map = Map::new(10, 10);

    map.display();
}
