extern crate rogue_rs;
extern crate tcod;

use rogue_rs::map::{make_map, Map, MAP_HEIGHT, MAP_WIDTH};
use rogue_rs::object::Object;
use tcod::colors::{self, Color};
use tcod::console::*;

const SCREEN_HEIGHT: i32 = 80;
const SCREEN_WIDTH: i32 = 80;
const LIMIT_FPS: i32 = 20;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color { r: 50, g: 50, b: 150 };

fn main() {
    let mut root = Root::initializer()
        .font("dejavu16x16_gs_tc.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial")
        .init();
    let mut con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);

    tcod::system::set_fps(LIMIT_FPS);

    let player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', colors::WHITE);
    let npc = Object::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, '@', colors::YELLOW);
    let mut objects = [player, npc];
    let map = make_map();

    while !root.window_closed() {
        render_all(&mut root, &mut con, &objects, &map);
        root.flush();

        for object in &objects {
            object.clear(&mut con)
        }
        let exit = handle_keys(&mut root, &mut objects[0], &map);
        if exit {
            break;
        }
    }
}

fn handle_keys(root: &mut Root, player: &mut Object, map: &Map) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = root.wait_for_keypress(true);
    match key {
        Key { code: Up, .. } => player.move_by(0, -1, map),
        Key { code: Down, .. } => player.move_by(0, 1, map),
        Key { code: Left, .. } => player.move_by(-1, 0, map),
        Key { code: Right, .. } => player.move_by(1, 0, map),

        Key { code: Enter, alt: true, .. } => {
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, .. } => return true,

        _ => {}
    }

    false
}

fn render_all(root: &mut Root, con: &mut Offscreen, objects: &[Object], map: &Map) {
    for object in objects {
        object.draw(con);
    }

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let wall = map[x as usize][y as usize].block_sight;
            if wall {
                con.set_char_background(x, y, COLOR_DARK_WALL, BackgroundFlag::Set)
            } else {
                con.set_char_background(x, y, COLOR_DARK_GROUND, BackgroundFlag::Set)
            }
        }
    }

    blit(con, (0, 0), (MAP_WIDTH, MAP_HEIGHT), root, (0, 0), 1.0, 1.0);
}