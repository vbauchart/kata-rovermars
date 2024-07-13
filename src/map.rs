pub struct Map {
    size_x: u32,
    size_y: u32,
}

impl Map {
    pub fn new(size_x: u32, size_y: u32) -> Map {
        Map { size_x, size_y }
    }

    pub fn count_position(&self) -> u32 {
        self.size_x * self.size_y
    }

    pub fn display(&self) {
        for _ in 0..=self.size_x {
            for _ in 0..self.size_y {
                print!("X")
            }
            println!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_count_position() {
        let map = Map::new(10, 10);

        assert_eq!(map.count_position(), 100)
    }
}
