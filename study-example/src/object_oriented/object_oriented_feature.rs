#[derive(Debug)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = (total as f64) / (self.list.len() as f64);
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            None => None,
            Some(value) => {
                self.update_average();
                Some(value)
            }
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
}

/// 运行结果如下
/// ```txt
/// average_collect init: AveragedCollection { list: [12, 34, 53, 98], average: 49.25 }
/// average_collect after add: AveragedCollection { list: [12, 34, 53, 98, 46], average: 48.6 }
/// average_collect after remove: AveragedCollection { list: [12, 34, 53, 98], average: 49.25 }
/// ```
pub fn object_oriented_feature_study() {
    let mut average_collect = AveragedCollection {
        list: vec![12, 34, 53, 98],
        average: (12 + 34 + 53 + 98) as f64 / 4 as f64,
    };
    println!("average_collect init: {:?}", average_collect);
    average_collect.add(46);
    println!("average_collect after add: {:?}", average_collect);
    average_collect.remove();
    println!("average_collect after remove: {:?}", average_collect);
}
