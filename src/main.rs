extern crate tcod;
use tcod::{Console, RootConsole, BackgroundFlag};
use tcod::console::Offscreen;
use tcod::colors::{self, Color};
use tcod::input::Key;
use tcod::input::KeyCode::{Up, Down, Left, Right, Escape};

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const MAP_WIDTH: usize = 80;
const MAP_HEIGHT: usize = 45;

const COLOR_DARK_WALL: Color = Color {
    r: 0,
    g: 0,
    b: 100,
};
const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};

type Map = Vec<Vec<Tile>>;

struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

impl Object {
    fn new(x: i32, y: i32, char: char, color: Color) -> Object {
        Object {
            x: x,
            y: y,
            char: char,
            color: color,
        }
    }

    fn move_by(&mut self, dx: i32, dy: i32, map: &Map) {
        if map[(self.x + dx) as usize][(self.y + dy) as usize].blocked != true {
            self.x = self.x + dx;
            self.y = self.y + dy;
        }
    }

    fn draw(&self, con: &mut Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }

    fn clear(&self, con: &mut Console) {
        con.put_char(self.x, self.y, ' ', BackgroundFlag::None);
    }
}

#[derive(Copy, Clone)]
struct Tile {
    blocked: bool,
    block_site: bool,
}

impl Tile {
    fn new(blocked: bool, block_site: bool) -> Tile {
        Tile {
            blocked: blocked,
            block_site: block_site,
        }
    }
}

fn main() {
    let mut root = RootConsole::initializer()
                       .size(SCREEN_WIDTH, SCREEN_HEIGHT)
                       .title("libtcod Rust tutorial")
                       .init();

    let mut con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', colors::WHITE);
    let npc = Object::new(SCREEN_WIDTH / 2 - 5,
                          SCREEN_HEIGHT / 2 - 5,
                          '@',
                          colors::YELLOW);

    let mut objects = [player, npc];

    let map = make_map();

    let mut exit = false;
    while !(root.window_closed() || exit) {
        render_all(&objects, &mut root, &mut con, &map);
        root.flush();

        for object in &objects {
            object.clear(&mut con);
        }
        exit = handle_keys(&mut root, &mut objects[0], &map);
    }
}

fn handle_keys(console: &mut RootConsole, player: &mut Object, map: &Map) -> bool {
    let keypress = console.wait_for_keypress(true);
    let mut ret = false;

    match keypress {
        Key { code: Up, .. } => player.move_by(0, -1, map),
        Key { code: Down, .. } => player.move_by(0, 1, map),
        Key { code: Left, .. } => player.move_by(-1, 0, map),
        Key { code: Right, .. } => player.move_by(1, 0, map),
        Key { code: Escape, .. } => {
            ret = true;
        }
        _ => {}
    };

    return ret;
}

fn make_map() -> Map {
    let mut map = vec![];

    for _ in 0..MAP_WIDTH {
        let col = vec![Tile::new(false, false); MAP_HEIGHT];
        map.push(col);
    }

    map[30][22].blocked = true;
    map[30][22].block_site = true;
    map[50][22].blocked = true;
    map[50][22].block_site = true;

    map
}

fn render_all(objects: &[Object], root: &mut RootConsole, con: &mut Offscreen, map: &Map) {
    for object in objects {
        object.draw(con);
    }

    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            let wall = map[x][y].block_site;
            let (x, y) = (x as i32, y as i32);

            if wall {
                con.set_char_background(x, y, COLOR_DARK_WALL, BackgroundFlag::Set);
            } else {
                con.set_char_background(x, y, COLOR_DARK_GROUND, BackgroundFlag::Set);
            }
        }
    }

    tcod::console::blit(con,
                        (0, 0),
                        (SCREEN_WIDTH, SCREEN_HEIGHT),
                        root,
                        (0, 0),
                        1.0,
                        1.0);
}
