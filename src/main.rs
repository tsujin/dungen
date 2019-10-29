extern crate dungen;
use dungen::Dungeon;

fn main() {
    let mut d = Dungeon::new(50, 50);
    let max_features = 35;
    d.generate(max_features);
    d.print_dungeon();
}
