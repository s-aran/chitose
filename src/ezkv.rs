use std::collections::HashMap;

type Key = (String, String, String);

pub struct EzKeyValue {
    data: HashMap<Key, String>,
}

impl EzKeyValue {
    pub fn new() -> Self {
        Self {
            data: HashMap::<Key, String>::new(),
        }
    }

    pub fn set(&mut self, key: Key, value: String) {
        self.data.insert(key, value);
    }

    pub fn try_get(&self, key: &Key) -> Option<&String> {
        self.data.get(key)
    }

    pub fn get_or<'a>(&'a self, key: &Key, default: &'a String) -> &'a String {
        match self.try_get(key) {
            Some(v) => v,
            None => default,
        }
    }

    pub fn includes(&self, key: &Key) -> bool {
        self.data.contains_key(key)
    }

    pub fn insert(&mut self, key: Key, value: String) {
        self.data.insert(key, value);
    }

    pub fn make_key(name: String, domain: String, path: String) -> Key {
        (name, domain, path)
    }

    pub fn insert_part(&mut self, name: String, domain: String, path: String, value: String) {
        let key = EzKeyValue::make_key(name, domain, path);
        self.insert(key, value);
    }

    pub fn update(&mut self, key: &Key, value: String) -> Option<String> {
        if !self.includes(key) {
            return None;
        }

        let old = self.data.get(key).unwrap().to_owned();
        *self.data.get_mut(key).unwrap() = value;

        Some(old)
    }
}
