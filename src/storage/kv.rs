use std::collections::BTreeMap;

struct Store {
    data: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl Store {
    fn new() -> Self {
        Store {
            data: BTreeMap::new(),
        }
    }

    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<(), ()> {
        self.data.insert(key.to_vec(), value.to_vec());
        Ok(())
    }

    fn get(self, key: &[u8]) -> Option<Vec<u8>> {
        match self.data.get(key) {
            Some(value) => Some(value.to_vec()),
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
        let key = [12, 13, 14];
        let value = [120, 130, 140];
        s.set(&key, &value).unwrap();
        let result = s.get(&key).unwrap();
        assert_eq!(value.to_vec(), result);
    }
}
