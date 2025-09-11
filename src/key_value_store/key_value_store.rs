use std::str::FromStr;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use crate::key_value_store::key_value_pair::KeyValuePair;


#[derive(Debug, PartialEq, Clone)]
pub struct KeyValueStore {
    name_: String,
    pairs_: HashMap<String, String>
}

impl KeyValueStore {
    pub fn new(name: &str) -> KeyValueStore {
        KeyValueStore {
            name_: String::from_str(name).expect("Cannot accept name"),
            pairs_: HashMap::new()
        }
    }

    pub fn get(&self, key: &str) -> Option<KeyValuePair> {
        let returnable: Option<KeyValuePair>;
        match self.pairs_.get(key) {
            Some(x) => {
                returnable = Some(
                    KeyValuePair::new(key, x));
            },
            None => {
                returnable = None;
            }
        }
        return returnable;
    }

    pub fn add(&mut self, pair: KeyValuePair) -> bool {
        if let Entry::Vacant(v) = self.pairs_.entry(
            pair.key().to_string()) {
            v.insert(pair.value().to_string());
            return true;
        }
        return false;
    }

    pub fn update(&mut self, pair: KeyValuePair) {
        self.pairs_.insert(
            pair.key().to_string(),
            pair.value().to_string()
        );
    }

    pub fn delete(&mut self, key: &str) -> bool {
        match self.pairs_.remove(key) {
            Some(_) => true,
            None => false
        }
    }

    pub fn name(&self) -> &str {
        &self.name_.as_str()
    }

    pub fn all(&self) -> HashMap<String, String> {
        // This is _such_ a waste of space.
        return self.pairs_.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crud() {
        let store_name = "test_store";
        let mut store = KeyValueStore::new(store_name);
        let first_key = "one";
        let first_pair = KeyValuePair::new(
            first_key, "uno");
        store.add(first_pair);

        // test read idempotency
        {
            let val = store.get(first_key).expect(
                "Expected value in store!");
            let val_2 = store.get(first_key).expect(
                "Expected value in store!");
            assert_eq!(val.key(), val_2.key());
            assert_eq!(val.value(), val_2.value());
            
            // try reading something that does not exist
            assert_eq!(store.get("nonexistent"), None);
            assert_eq!(store.get("nonexistent"), None);
        }

        // test add - ensure that adding duplicates returns false
        {
            let duplicate_add = store.add(KeyValuePair::new(
                first_key, "uno again"
            ));
            assert_eq!(duplicate_add, false);
            let acceptable_add = store.add(KeyValuePair::new(
                "two", "dos"
            ));
            assert_eq!(acceptable_add, true);
        }

        // test update
        {
            // with a key that exists
            store.update(KeyValuePair::new(
                first_key, "one_again" 
            ));
            let new_val = store.get(first_key).expect(
                "Expected value in store!");
            assert_eq!(new_val.value(), "one_again");

            // with a key that does not exist
            store.update(KeyValuePair::new(
                "another_key", "another_value"
            ));
            let new_val_2 = store.get("another_key").expect(
                "Expected value in store!");
            assert_eq!(new_val_2.value(), "another_value");
        }

        // test delete
        {
            // on a key that exists
            let res = store.delete(first_key);
            assert_eq!(res, true);

            // on a key that does not exist
            let res_2 = store.delete("404");
            assert_eq!(res_2, false);
        }

        assert_eq!(store.name(), store_name);
    }
}
