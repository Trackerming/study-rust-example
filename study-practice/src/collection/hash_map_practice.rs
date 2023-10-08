// 填空并修复错误
use std::collections::HashMap;
fn method_1() {
    let mut scores = HashMap::new();
    scores.insert("Sunface", 98);
    scores.insert("Daniel", 95);
    scores.insert("Ashley", 69);
    scores.insert("Katie", 58);

    // get 返回一个 Option<&V> 枚举值
    let score = scores.get("Sunface");
    assert_eq!(score, Some(&98));

    if scores.contains_key("Daniel") {
        // 索引返回一个值 V
        let score = scores["Daniel"];
        assert_eq!(score, 95);
        scores.remove("Daniel");
    }

    assert_eq!(scores.len(), 3);

    for (name, score) in scores {
        println!("The score of {} is {}", name, score)
    }
}

fn method_2() {
    let teams = [
        ("Chinese Team", 100),
        ("American Team", 10),
        ("France Team", 50),
    ];

    let mut teams_map1 = HashMap::new();
    for team in &teams {
        teams_map1.insert(team.0, team.1);
    }

    // 使用两种方法实现 team_map2
    // 提示:其中一种方法是使用 `collect` 方法
    // 方法1:
    //let teams_map2: HashMap<&str, i32> = teams.into_iter().collect();
    let teams_map2 = HashMap::from_iter(teams.into_iter());
    assert_eq!(teams_map1, teams_map2);

    println!("Success!")
}

// 填空
fn method_3() {
    // 编译器可以根据后续的使用情况帮我自动推断出 HashMap 的类型，当然你也可以显式地标注类型：HashMap<&str, u8>
    let mut player_stats = HashMap::new();

    // 查询指定的 key, 若不存在时，则插入新的 kv 值
    player_stats.entry("health").or_insert(100);

    assert_eq!(player_stats["health"], 100);

    // 通过函数来返回新的值
    player_stats
        .entry("health")
        .or_insert_with(random_stat_buff);
    assert_eq!(player_stats["health"], 100);

    let health = player_stats.entry("health").or_insert(50);
    assert_eq!(health, &mut 100);
    *health -= 50;
    assert_eq!(*health, 50);

    println!("Success!")
}

fn random_stat_buff() -> u8 {
    // 为了简单，我们没有使用随机，而是返回一个固定的值
    42
}

// 修复错误
// 提示: `derive` 是实现一些常用特征的好办法
#[derive(Debug, Hash, PartialEq, Eq)]
struct Viking {
    name: String,
    country: String,
}
impl Viking {
    fn new(name: &str, country: &str) -> Viking {
        Viking {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}
fn method_4() {
    // 使用 HashMap 来存储 viking 的生命值
    let vikings = HashMap::from([
        (Viking::new("Einar", "Norway"), 25),
        (Viking::new("Olaf", "Denmark"), 24),
        (Viking::new("Harald", "Iceland"), 12),
    ]);

    // 使用 derive 的方式来打印 viking 的当前状态
    for (viking, health) in &vikings {
        println!("{:?} has {} hp", viking, health);
    }
}

// 修复错误，尽可能少的去修改代码
// 不要移除任何代码行！
fn method_5() {
    let v1 = 10;
    let mut m1 = HashMap::new();
    m1.insert(v1, v1);
    println!("v1 is still usable after inserting to hashmap : {}", v1);

    let v2 = "hello".to_string();
    let mut m2 = HashMap::new();
    // 所有权在这里发生了转移
    m2.insert(&v2, v1);

    assert_eq!(v2, "hello".to_string());

    println!("Success!")
}

pub fn practice() {
    println!("Collection HashMap practice method1:");
    method_1();
    println!("Collection HashMap practice method2:");
    method_2();
    println!("Collection HashMap practice method3:");
    method_3();
    println!("Collection HashMap practice method4:");
    method_4();
    println!("Collection HashMap practice method5:");
    method_5();
}
