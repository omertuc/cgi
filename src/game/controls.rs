use std::collections::{HashMap, HashSet};

use sdl2::keyboard::Scancode;

use crate::primitives::input::Groups;
use crate::primitives::input::KeyStack;

pub type KeyMap = HashMap<Scancode, GameKey>;
pub(crate) type GameKeyStack = KeyStack<GameKey>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameKey {
    NoOp,
    Run,
    Walk,
    Right,
    Left,
    Forward,
    Backwards,
    VsyncToggle,
    Quit,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GameKeyGroup {
    Horizontal,
    Vertical,
}

impl Groups for GameKey {
    type GroupType = GameKeyGroup;
    fn groups(&self) -> HashSet<GameKeyGroup> {
        match self {
            GameKey::Right | GameKey::Left => [GameKeyGroup::Horizontal],
            GameKey::Forward | GameKey::Backwards => [GameKeyGroup::Vertical],
            _ => {
                return HashSet::<GameKeyGroup>::new();
            }
        }
        .iter()
        .copied()
        .collect()
    }
}

pub fn init_key_map() -> HashMap<Scancode, GameKey> {
    let game_keys = hashmap! {
        &[Scancode::A, Scancode::Left][..] => GameKey::Left,
        &[Scancode::D, Scancode::Right][..] => GameKey::Right,
        &[Scancode::W, Scancode::Up][..] => GameKey::Forward,
        &[Scancode::S, Scancode::Down][..] => GameKey::Backwards,
        &[Scancode::V][..] => GameKey::VsyncToggle,
        &[Scancode::LShift, Scancode::RShift][..] => GameKey::Run,
        &[Scancode::LCtrl, Scancode::RCtrl][..] => GameKey::Walk,
        &[Scancode::Q, Scancode::Escape][..] => GameKey::Quit,
    };

    // Flatten
    let mut final_map = HashMap::new();
    for (game_key, value) in game_keys {
        for key in game_key.iter() {
            final_map.insert(*key, value);
        }
    }

    final_map
}
