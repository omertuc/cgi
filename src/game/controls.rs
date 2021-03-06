use std::collections::HashSet;
use std::convert::From;

pub type MouseMovement = (i32, i32);

/// A KeyStack is a vector of keys that records the currently pressed keys, and the order in which
/// they were pressed. Keys are of type KeyType, which must implement the Groups trait so the groups
/// to which that key belongs can be retrieved. Groups are used to create a normalized copy of the
/// KeyStack, see the [`Self::normalize()`] implementation for more information.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct KeyStack<KeyType> {
    stack: Vec<KeyType>,
}

pub trait Groups {
    type GroupType: Eq + std::hash::Hash;
    fn groups(&self) -> HashSet<Self::GroupType>;
}

impl<KeyType> KeyStack<KeyType> where
    KeyType: Copy + PartialEq + Groups {
    pub fn new() -> Self {
        KeyStack {
            stack: vec![],
        }
    }

    pub fn press(&self, key: KeyType) -> Self {
        if self.is_pressed(key) {
            self.clone()
        } else {
            let mut new = self.clone();
            new.stack.push(key);
            new
        }
    }

    pub fn depress(&self, key: KeyType) -> Self {
        self.clone().stack.into_iter().filter(
            |other| &key != other).collect::<Vec<KeyType>>().clone().into()
    }

    /// Returns a normalized version of the KeyStack. Normalizing a KeyStack involves
    /// keeping only 1 key from each group, with other keys from that same group lower in the stack
    /// removed.
    /// This is used when keys which cancel each are pressed at the same time - we want to ignore
    /// all the keys that were pressed earlier, giving priority to those which were pressed later.
    pub fn normalize(&self) -> KeyStack<KeyType> {
        if self.stack.len() == 0 {
            return KeyStack::new();
        }

        let mut encountered_groups = HashSet::new();

        let mut new = KeyStack::new();

        for current in self.stack.iter().rev() {
            let groups = current.groups();
            if groups.intersection(&encountered_groups).count() != 0 {
                continue;
            }

            encountered_groups.extend(groups.into_iter());

            new.stack.push(*current)
        }

        new.stack.into_iter().rev().collect::<Vec<KeyType>>().into()
    }

    pub fn is_pressed(&self, key: KeyType) -> bool {
        self.stack.contains(&key)
    }
}

impl<KeyType> From<Vec<KeyType>> for KeyStack<KeyType> {
    fn from(other_vec: Vec<KeyType>) -> KeyStack<KeyType> {
        KeyStack::<KeyType> {
            stack: other_vec,
        }
    }
}
