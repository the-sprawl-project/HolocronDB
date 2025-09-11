use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct KeyValuePair {
    key_: String,
    value_: String
}

impl KeyValuePair {
    pub fn new(k: &str, v: &str) -> KeyValuePair {       
        KeyValuePair {
            key_: String::from_str(k).expect("Cannot parse key string!"),
            value_: String::from_str(v).expect("Cannot parse value string!")
         }
    }

    pub fn key(&self) -> &str {
        self.key_.as_str()
    }

    pub fn value(&self) -> &str {
        self.value_.as_str()
    }

    pub fn update_value(&mut self, new_val: &str) {
        self.value_ = String::from_str(new_val).expect(
            "Cannot parse new value!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_value_pair_created() {
        let key = "Hello";
        let value = "World";
        let item = KeyValuePair::new(key, value);
        assert_eq!(item.key(), key);
        assert_eq!(item.value(), value);
    }

    #[test]
    fn test_key_value_pair_updated() {
        let key = "Spicy";
        let value = "Pepper";
        let value_2 = "Curry";
        let mut item = KeyValuePair::new(key, value);
        item.update_value(value_2);
        assert_eq!(item.key(), key);
        assert_eq!(item.value(), value_2);
    }
}