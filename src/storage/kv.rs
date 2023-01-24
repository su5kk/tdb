use std::collections::BTreeMap;

struct Store {
    data: BTreeMap<String, u8>
}

impl Store {
    fn new() -> Self {
        Store { data: BTreeMap::new() }
    }

    fn set(&mut self, key: &str, value: u8) -> Result<String, ()> {
        self.data.insert(key.to_owned(), value);
        Ok(key.into())
    }

    fn get(self, key: &str) -> Option<u8> {
        match self.data.get(key) {
            Some(value) => Some(*value),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut s = Store::new();
        s.set("a", 1).unwrap();
        assert_eq!(1, s.get("a").unwrap())
    }
}
