use super::tile::Tile;

pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 45;

pub type Map = Vec<Vec<Tile>>;

pub fn make_map() -> Map {
    let mut map = vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    map[30][22] = Tile::wall();
    map[50][22] = Tile::wall();

    map
}