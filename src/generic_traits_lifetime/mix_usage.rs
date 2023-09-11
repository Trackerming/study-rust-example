use core::fmt;
use std::fmt::Debug;
use std::fmt::Display;

pub fn longer_str_with_announcement<'a, T>(str1: &'a str, str2: &'a str, anno: T) -> &'a str
where
    T: Display + Debug,
{
    println!("Announcement: {}", anno);
    if str1.len() >= str2.len() {
        str1
    } else {
        str2
    }
}

#[derive(Debug)]
struct Announcement<'a> {
    anno: &'a str,
    time: &'a str,
}

impl<'a> Announcement<'a> {}

impl<'a> fmt::Display for Announcement<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Debug anno: {}, time: {}", self.anno, self.time)
    }
}

pub fn mix_usage_study() {
    let anno1 = "display in longer_str_with_announcement.";
    let str1 = "longer test str1";
    let str2 = String::from("longer test str2.");
    let str_longer = longer_str_with_announcement(&str1, &str2, &anno1);
    println!("str longer {}", str_longer);
    let time = String::from("2023年 9月11日 星期一 12时16分36秒 CST");
    let anno2 = Announcement {
        anno: &anno1,
        time: &time,
    };
    let str_longer2 = longer_str_with_announcement(&str1, &str2, &anno2);
    println!("str longer2 {}", str_longer2);
}
