use sdl2::keyboard::Scancode;

use crate::primitives::input::{KeyStack, MouseMovement};

use crate::primitives::input::Groups;
use std::collections::{HashMap, HashSet};

pub type KeyMap = HashMap<Scancode, GameKey>;
pub(crate) type GameKeyStack = KeyStack<GameKey>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameKey {
    NoOp,
    RollModifier,
    CameraModifier,
    Right,
    Left,
    Up,
    Down,
    VsyncToggle,
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
            GameKey::Up | GameKey::Down => [GameKeyGroup::Vertical],
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
        &[Scancode::W, Scancode::Up][..] => GameKey::Up,
        &[Scancode::S, Scancode::Down][..] => GameKey::Down,
        &[Scancode::V][..] => GameKey::VsyncToggle,
        &[Scancode::LShift, Scancode::RShift][..] => GameKey::RollModifier,
        &[Scancode::LCtrl, Scancode::RCtrl][..] => GameKey::CameraModifier,
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
