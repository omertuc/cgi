use core::convert::From;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameKey {
    RollModifier,
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum GameKeyGroup {
    Horizontal,
    Vertical,
    Modifiers,
}

impl GameKey {
    fn groups(self) -> HashSet<GameKeyGroup> {
        match self {
            GameKey::RollModifier => { [GameKeyGroup::Modifiers] }
            GameKey::Right => { [GameKeyGroup::Horizontal] }
            GameKey::Left => { [GameKeyGroup::Horizontal] }
            GameKey::Up => { [GameKeyGroup::Vertical] }
            GameKey::Down => { [GameKeyGroup::Vertical] }
        }.iter().copied().collect()
    }
}

pub type MouseMovement = (i32, i32);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct KeyStack {
    stack: Vec<GameKey>,
}

impl KeyStack {
    pub fn new() -> KeyStack {
        KeyStack {
            stack: vec![],
        }
    }

    pub fn press(&self, key: GameKey) -> KeyStack {
        if self.is_pressed(key) {
            self.clone()
        } else {
            let mut new = self.clone();
            new.stack.push(key);
            new.into()
        }
    }

    pub fn depress(&self, key: GameKey) -> KeyStack {
        self.stack.clone().into_iter().filter(
            |other| &key != other).collect::<Vec<GameKey>>().into()
    }

    /// Returns a normalized version of the KeyStack. Normalizing a KeyStack involves
    /// keeping only 1 key from each group, with the ones lower in the stack removed.
    pub fn normalize(&self) -> KeyStack {
        if self.stack.len() == 0 {
            return KeyStack::new();
        }

        let mut encountered_groups = vec![].into_iter().collect();

        let mut new = KeyStack::new();

        for i in (0..self.stack.len()).rev() {
            let current = self.stack[i];
            let groups = current.groups();
            if groups.intersection(&encountered_groups).count() != 0 {
                continue;
            }

            for group in groups {
                encountered_groups.insert(group);
            }

            new.stack.push(current)
        }

        new.stack.into_iter().rev().collect::<Vec<GameKey>>().into()
    }

    pub fn is_pressed(&self, key: GameKey) -> bool {
        self.stack.contains(&key)
    }
}

impl From<Vec<GameKey>> for KeyStack {
    fn from(other_vec: Vec<GameKey>) -> KeyStack {
        KeyStack {
            stack: other_vec
        }
    }
}
