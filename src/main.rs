use mapgen::{Coord, Tile};

fn main() {
    let c = Coord::new(-0.1f32, 5f32);
    let tile = Tile::from_coord(c, 5);
    println!("{tile:?} <- {c:?}");
    println!("{:?}", tile.pixel(&c));
}
