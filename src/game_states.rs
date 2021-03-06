extern crate core;

use std::rc::Rc;
use std::cell::RefCell;

use rendering::windows::Windows;
use rendering::renderers::RenderingComponent;
use map::Maps;
use game::MoveInfo;
use input::{KeyCode,};
use input::Key::{SpecialKey};
use combat::{Weapon, Boomerang};
use util::Point;

use self::core::ops::Deref;

pub trait GameState {
    fn enter(&self, &mut Windows) {}
    fn exit(&self)  {}

    fn update(&mut self, maps: &mut Maps, windows: &mut Windows, Rc<RefCell<MoveInfo>>);
    fn render(&mut self, renderer: &mut Box<RenderingComponent>, maps: &mut Maps, windows: &mut Windows) {
        renderer.before_render_new_frame();
        let ref mut stats = windows.stats;
        renderer.attach_window(stats);

        let ref mut input = windows.input;
        renderer.attach_window(input);

        let ref mut messages = windows.messages;
        renderer.attach_window(messages);

        let ref mut map = windows.map;
        renderer.attach_window(map);
        maps.render(renderer);
        renderer.after_render_new_frame();
    }

    fn should_update_state(&self) -> bool;
}

pub struct MovementGameState;

impl MovementGameState {
    pub fn new() -> MovementGameState {
        MovementGameState
    }
}

impl GameState for MovementGameState {
    fn should_update_state(&self) -> bool {
        true
    }

    fn enter(&self, windows: &mut Windows) {
        windows.input.flush_buffer();
    }

    fn update(&mut self, maps: &mut Maps, windows: &mut Windows, move_info: Rc<RefCell<MoveInfo>>) {
        let last_keypress = {
            move_info.borrow().deref().last_keypress
        };
        match last_keypress {
            Some(ks) => {
                match ks.key {
                    // Because Shift is used for attack keys we don't want to do
                    // anything when it's pushed. We can check for shift when we
                    // process the next keypress
                    SpecialKey(KeyCode::Shift) => {},
                    _                          => { maps.update(windows); }
                }
            },
            _    => {}
        }
    }
}

pub struct AttackInputGameState {
    should_update_state: bool,
    pub weapon: Box<Weapon + 'static>
}

impl AttackInputGameState {
    pub fn new() -> AttackInputGameState {
        let weapon = Box::new(Boomerang::new());
        AttackInputGameState {
            should_update_state: false,
            weapon: weapon
        }
    }

    pub fn new_with_weapon(weapon: Box<Weapon + 'static>) -> AttackInputGameState {
        AttackInputGameState {
            should_update_state: false,
            weapon: weapon
        }
    }
}

impl GameState for AttackInputGameState {
    fn should_update_state(&self) -> bool {
        self.should_update_state
    }

    fn enter(&self, windows: &mut Windows) {
        windows.input.flush_buffer();
        let mut msg = "Which direction do you want to attack with ".to_string();
        msg.push_str(self.weapon.get_name().as_slice());
        msg.push_str("? [Use the arrow keys to answer]");
        windows.input.buffer_message(msg.as_slice())
    }

    fn update(&mut self, maps: &mut Maps, windows: &mut Windows, move_info: Rc<RefCell<MoveInfo>>) {
        let last_keypress = {
            move_info.borrow().deref().last_keypress
        };
        match last_keypress {
            Some(ks) => {
                let mut msg = "You attack ".to_string();
                let char_point = {
                    move_info.borrow().deref().char_location.clone()
                };
                let mut point = Point::new(0, 0);
                match ks.key {
                    SpecialKey(KeyCode::Up) => {
                        point = char_point.offset_y(-1);
                        msg.push_str("up");
                        self.should_update_state = true;
                    },
                    SpecialKey(KeyCode::Down) => {
                        point = char_point.offset_y(1);
                        msg.push_str("down");
                        self.should_update_state = true;
                    },
                    SpecialKey(KeyCode::Left) => {
                        point = char_point.offset_x(-1);
                        msg.push_str("left");
                        self.should_update_state = true;
                    },
                    SpecialKey(KeyCode::Right) => {
                        point = char_point.offset_x(1);
                        msg.push_str("right");
                        self.should_update_state = true;
                    },
                    _ => {}
                }

                if self.should_update_state {
                    match maps.enemy_at(point) {
                        Some(enemy) => {
                            let pc = maps.pcs.actor_at(char_point).unwrap();
                            println!("{}", pc.display_char);
                            msg.push_str(" with your ");
                            msg.push_str(self.weapon.get_name().as_slice());
                            msg.push_str(" for ");
                            msg.push_str(self.weapon.deal_damage(enemy).to_string().as_slice());
                            msg.push_str(" points of damage!");
                            windows.messages.buffer_message(msg.as_slice());
                        },
                        None => {
                            windows.messages.buffer_message("No enemy in that direction!");
                        }
                    }
                }
            },
            _ => {}
        }
    }
}
