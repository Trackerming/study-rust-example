#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct Pair {
    pub key: i32,
    pub value: String,
}

pub struct ArrayHashMap {
    size: i32,
    capacity: usize,
    load_factor_threshold: f32,
    extend_ratio: i32,
    // 采用动态数组模拟链表，简化实现
    buckets: Vec<Vec<Pair>>,
}

const DEFAULT_BUCKET_SIZE: usize = 4;

impl ArrayHashMap {
    pub fn new() -> Self {
        Self {
            size: 0,
            capacity: DEFAULT_BUCKET_SIZE,
            load_factor_threshold: 2.0 / 3.0,
            extend_ratio: 2,
            buckets: vec![vec![]; DEFAULT_BUCKET_SIZE],
        }
    }

    fn load_factor(&self) -> f32 {
        self.size as f32 / self.capacity as f32
    }

    fn extend(&mut self) {
        // 临时存储原hash表
        let temp_buckets = std::mem::replace(&mut self.buckets, vec![]);
        // 初始化扩容的hash表
        self.capacity = self.capacity * self.extend_ratio as usize;
        self.buckets = vec![Vec::new(); self.capacity];
        self.size = 0;
        // 迁移原数据
        for bucket in temp_buckets {
            for pair in bucket {
                self.put(pair);
            }
        }
    }

    /*
     * 模拟hash函数让其落到范围内的数组的index
     */
    pub fn hash_mock(&self, key: i32) -> usize {
        key as usize % self.capacity
    }

    pub fn get(&self, key: i32) -> Option<&String> {
        let index = self.hash_mock(key);
        let bucket = self.buckets.get(index).unwrap();
        for pair in bucket {
            if pair.key == key {
                return Some(&pair.value);
            }
        }
        None
    }

    pub fn put(&mut self, pair: Pair) {
        // 若超过阈值，进行扩容操作
        if self.load_factor() > self.load_factor_threshold {
            self.extend();
        }
        let index = self.hash_mock(pair.key);
        let bucket = self.buckets.get_mut(index).unwrap();
        for p in bucket {
            if p.key == pair.key {
                p.value = pair.value.clone();
                return;
            }
        }
        let bucket = self.buckets.get_mut(index).unwrap();
        self.size += 1;
        bucket.push(pair);
    }

    pub fn remove(&mut self, key: i32) -> Option<String> {
        let index = self.hash_mock(key);
        let mut bucket = self.buckets.get_mut(index).unwrap();
        for i in 0..bucket.len() {
            if bucket[i].key == key {
                let pair = bucket.remove(i);
                self.size -= 1;
                return Some(pair.value);
            }
        }
        None
    }

    pub fn entry(&self) -> Vec<&Pair> {
        self.buckets
            .iter()
            .flat_map(|pair_bucket| pair_bucket.iter().map(|pair| pair))
            .collect()
    }

    pub fn keys(&self) -> Vec<&i32> {
        self.buckets
            .iter()
            .flat_map(|pair_bucket| pair_bucket.iter().map(|p| &p.key))
            .collect()
    }

    pub fn values(&self) -> Vec<&String> {
        self.buckets
            .iter()
            .flat_map(|pair_bucket| pair_bucket.iter().map(|p| &p.value))
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

    #[test]
    pub fn test_extend() {
        let mut array_hash_map = ArrayHashMap::new();
        array_hash_map.put(Pair {
            key: 1,
            value: "1_string".to_string(),
        });
        array_hash_map.put(Pair {
            key: 2,
            value: "2_string".to_string(),
        });
        assert_eq!(array_hash_map.capacity, 4);
        assert_eq!(array_hash_map.size, 2);
        array_hash_map.put(Pair {
            key: 3,
            value: "3_string".to_string(),
        });
        array_hash_map.put(Pair {
            key: 5,
            value: "5_string".to_string(),
        });
        assert_eq!(array_hash_map.capacity, 8);
        assert_eq!(array_hash_map.size, 4);
        let value = array_hash_map.get(1);
        assert_eq!(value, Some(&"1_string".to_string()));
        array_hash_map.put(Pair {
            key: 1,
            value: "1_string_updated".to_string(),
        });
        array_hash_map.put(Pair {
            key: 9,
            value: "9_string".to_string(),
        });
        let index = array_hash_map.hash_mock(9);
        let bucket = array_hash_map.buckets.get(index).unwrap();
        assert_eq!(bucket.len(), 2);
        let value = array_hash_map.get(1);
        assert_eq!(value, Some(&"1_string_updated".to_string()));
        array_hash_map.remove(5);
        assert_eq!(array_hash_map.size, 4);
    }
}
