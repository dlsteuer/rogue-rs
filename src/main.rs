extern crate rogue_rs;
extern crate tcod;

use rogue_rs::map::{make_map, Map, MAP_HEIGHT, MAP_WIDTH};
use rogue_rs::object::Object;
use tcod::colors::{self, Color};
use tcod::console::*;
use tcod::map::{FovAlgorithm, Map as FovMap};
use rogue_rs::object::PLAYER;
use rogue_rs::object::move_by;
use rogue_rs::object::PlayerAction;
use rogue_rs::object::Fighter;
use rogue_rs::object::ai_take_turn;

const SCREEN_HEIGHT: i32 = 80;
const SCREEN_WIDTH: i32 = 80;
const LIMIT_FPS: i32 = 20;

const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic;
const FOV_LIGHT_WALLS: bool = true;
const TORCH_RADIUS: i32 = 10;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_LIGHT_WALL: Color = Color { r: 130, g: 110, b: 50 };
const COLOR_DARK_GROUND: Color = Color { r: 50, g: 50, b: 150 };
const COLOR_LIGHT_GROUND: Color = Color { r: 200, g: 180, b: 50 };

fn main() {
    let mut root = Root::initializer()
        .font("dejavu16x16_gs_tc.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial")
        .init();
    let mut con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);

    tcod::system::set_fps(LIMIT_FPS);


    let mut player = Object::new(0, 0, '@', "player", colors::WHITE, true);
    player.alive = true;
    player.fighter = Some(Fighter{
        max_hp: 30,
        hp: 30,
        defense: 2,
        power: 5
    });
    let mut objects = vec![player];

    let mut map = make_map(&mut objects);

    let mut fov_map = FovMap::new(MAP_WIDTH, MAP_HEIGHT);
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            fov_map.set(x, y,
                        !map[x as usize][y as usize].block_sight,
                        !map[x as usize][y as usize].blocked)
        }
    }

    let mut previous_player_position = (-1, -1);

    while !root.window_closed() {
        let fov_recompute = previous_player_position != (objects[0].x, objects[0].y);
        render_all(&mut root, &mut con, &objects, &mut map, &mut fov_map, fov_recompute);
        root.flush();

        for object in &objects {
            object.clear(&mut con)
        }
        previous_player_position = (objects[0].x, objects[0].y);
        let player_action = handle_keys(&mut root, &map, &mut objects);
        if player_action == PlayerAction::Exit {
            break
        }

        if objects[PLAYER].alive && player_action != PlayerAction::DidntTakeTurn {
            for id in 0..objects.len() {
                if objects[id].ai.is_some() {
                    ai_take_turn(id, &map, &mut objects, &fov_map);
                }
            }
        }
    }
}

fn handle_keys(root: &mut Root, map: &Map, objects: &mut [Object]) -> PlayerAction {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = root.wait_for_keypress(true);
    let player_alive = objects[PLAYER].alive;
    match (key, player_alive) {
        (Key { code: Up, .. }, true) => {
            player_move_or_attack(0, -1, map, objects);
            PlayerAction::TookTurn
        },
        (Key { code: Down, .. }, true) => {
            player_move_or_attack(0, 1, map, objects);
            PlayerAction::TookTurn
        },
        (Key { code: Left, .. }, true) => {
            player_move_or_attack(-1, 0, map, objects);
            PlayerAction::TookTurn
        },
        (Key { code: Right, .. }, true) => {
            player_move_or_attack(1, 0, map, objects);
            PlayerAction::TookTurn
        },

        (Key { code: Enter, alt: true, .. }, _) => {
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
            PlayerAction::DidntTakeTurn
        }
        (Key { code: Escape, .. }, _) => PlayerAction::Exit,

        _ => PlayerAction::DidntTakeTurn
    }
}

fn player_move_or_attack(dx: i32, dy: i32, map: &Map, objects: &mut [Object]) {
    let x = objects[PLAYER].x + dx;
    let y = objects[PLAYER].y + dy;

    let target_id = objects.iter().position(|object| {
        object.pos() == (x,y)
    });

    match target_id {
        Some(target_id) => {
            println!("The {} laughs at your puny efforts to attack him!", objects[target_id].name)
        }
        None => {
            move_by(PLAYER, dx, dy, map, objects)
        }
    }
}

fn render_all(root: &mut Root, con: &mut Offscreen, objects: &[Object], map: &mut Map, fov_map: &mut FovMap, fov_recompute: bool) {
    if fov_recompute {
        let player = &objects[0];
        fov_map.compute_fov(player.x, player.y, TORCH_RADIUS, FOV_LIGHT_WALLS, FOV_ALGO);

        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let visible = fov_map.is_in_fov(x, y);
                let wall = map[x as usize][y as usize].block_sight;
                let color = match (visible, wall) {
                    (false, true) => COLOR_DARK_WALL,
                    (false, false) => COLOR_DARK_GROUND,
                    (true, true) => COLOR_LIGHT_WALL,
                    (true, false) => COLOR_LIGHT_GROUND,
                };
                let explored = &mut map[x as usize][y as usize].explored;
                if visible {
                    *explored = true;
                }

                if *explored {
                    con.set_char_background(x, y, color, BackgroundFlag::Set)
                }
            }
        }
    }

    for object in objects {
        if fov_map.is_in_fov(object.x, object.y) {
            object.draw(con);
        }
    }

    blit(con, (0, 0), (MAP_WIDTH, MAP_HEIGHT), root, (0, 0), 1.0, 1.0);
}