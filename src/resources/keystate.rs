use std::collections::HashSet;

#[derive(Default)]
pub struct Keystate {
    pub keys: HashSet<glfw::Key>,
}

impl Keystate {
    pub fn set_key_down(&mut self, key: glfw::Key) {
        self.keys.insert(key);
    }

    pub fn set_key_up(&mut self, key: glfw::Key) {
        self.keys.remove(&key);
    }

    pub fn is_key_down(&self, key: glfw::Key) -> bool {
        self.keys.contains(&key)
    }
}
