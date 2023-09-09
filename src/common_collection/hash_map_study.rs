use std::collections::{hash_map, HashMap};

#[derive(Eq, Hash, PartialEq, Debug)]
enum CoinEnum {
    BTC,
    ETH,
    USDT,
}
#[derive(Debug)]
struct CoinProerty {
    decimal: u32,
}

fn iterator_hash_map(map_val: &HashMap<CoinEnum, CoinProerty>) {
    for (key, val) in map_val {
        println!("key {:?} val {:?}", key, val);
    }
}

fn count_text_num(str_val: &str, target: &str) -> u16 {
    let mut count_map: HashMap<&str, u16> = HashMap::new();
    for word in str_val.split_whitespace() {
        let count = count_map.entry(word).or_insert(0);
        *count += 1;
    }
    match count_map.get(target) {
        None => 0,
        Some(val) => *val,
    }
}

pub fn hash_map_usage_study() {
    let mut coinMap: HashMap<CoinEnum, CoinProerty> = HashMap::new();
    coinMap.insert(CoinEnum::BTC, CoinProerty { decimal: 8 });
    coinMap.insert(CoinEnum::ETH, CoinProerty { decimal: 9 });
    coinMap.insert(CoinEnum::ETH, CoinProerty { decimal: 18 });
    coinMap.insert(CoinEnum::USDT, CoinProerty { decimal: 6 });
    // 检查map是否存在
    coinMap
        .entry(CoinEnum::USDT)
        .or_insert(CoinProerty { decimal: (6) });
    // 读取hash操作
    let coin = CoinEnum::ETH;
    let eth_pro = coinMap.get(&coin);
    println!();
    println!("coin {:?}, pro {:?}", coin, eth_pro);
    iterator_hash_map(&coinMap);
    let text = "hello hello world rust people hello  hello .";
    let count = count_text_num(text, &text[..5]);
    println!("word {} count {}", &text[..5], count);
}
