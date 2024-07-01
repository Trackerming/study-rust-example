#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Pair {
    pub key: i32,
    pub value: String,
}

pub struct ArrayHashMap {
    buckets: Vec<Option<Pair>>,
}

const DEFAULT_BUCKET_SIZE: usize = 100;

impl ArrayHashMap {
    pub fn new() -> Self {
        Self {
            buckets: vec![None; DEFAULT_BUCKET_SIZE],
        }
    }

    /*
     * 模拟hash函数让其落到范围内的数组的index
     */
    pub fn hash_mock(&self, key: i32) -> usize {
        key as usize % DEFAULT_BUCKET_SIZE
    }

    pub fn get(&self, key: i32) -> Option<&String> {
        let index = self.hash_mock(key);
        self.buckets
            .get(index)
            .unwrap()
            .as_ref()
            .map(|pair| &pair.value)
    }

    pub fn put(&mut self, pair: Pair) {
        let index = self.hash_mock(pair.key);
        self.buckets[index] = Some(pair);
    }

    pub fn remove(&mut self, key: i32) {
        let index = self.hash_mock(key);
        self.buckets[index] = None;
    }

    pub fn entry(&self) -> Vec<&Pair> {
        self.buckets
            .iter()
            .filter_map(|pair| pair.as_ref())
            .collect()
    }

    pub fn keys(&self) -> Vec<&i32> {
        self.buckets
            .iter()
            .filter_map(|pair| pair.as_ref().map(|p| &p.key))
            .collect()
    }

    pub fn values(&self) -> Vec<&String> {
        self.buckets
            .iter()
            .filter_map(|pair| pair.as_ref().map(|p| &p.value))
            .collect()
    }

    pub fn print(&self) {
        for pair in self.entry() {
            println!("{}:{}", pair.key, pair.value)
        }
    }
}

#[cfg(test)]
mod test_hash_map {
    use super::*;

    #[test]
    pub fn test_hash_map() {
        let mut array_hash_map = ArrayHashMap::new();
        let string_of_key_134 = "134_string".to_string();
        array_hash_map.put(Pair {
            key: 134,
            value: string_of_key_134.clone(),
        });
        let value = array_hash_map.get(134);
        assert_eq!(value, Some(&string_of_key_134));
        array_hash_map.put(Pair {
            key: 134,
            value: "174_string".to_string(),
        });
        let value = array_hash_map.get(134);
        assert_eq!(value, Some(&"174_string".to_string()));
        array_hash_map.remove(134);
        let value = array_hash_map.get(134);
        assert_eq!(value, None);
    }
}
