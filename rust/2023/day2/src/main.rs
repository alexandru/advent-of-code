struct GameSet(i32, i32, i32);

impl GameSet {
    fn includes(&self, other: &GameSet) -> bool {
        self.0 >= other.0 && self.1 >= other.1 && self.2 >= other.2
    }
}

struct Game {
    id: i32,
    sets: Vec<GameSet>,
}

fn main() {
    // let lines: Vec<&str> = include_str!("input.txt").lines().collect();

    let g1 = GameSet(3, 3, 3);
    let g2 = GameSet(4, 4, 2);

}
