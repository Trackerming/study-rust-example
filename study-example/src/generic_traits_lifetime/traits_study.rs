use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
trait Summary {
    fn summarize(&self) -> String;
}

struct Tweet {
    pub content: String,
    pub autor: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} by {}", self.autor, self.content)
    }
}

struct Article {
    pub headline: String,
    pub location: String,
    pub author: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!(
            "{} by {}, locate: {}",
            self.headline, self.author, self.location
        )
    }
}

fn notify(item: &impl Summary) {
    println!("Breaking news: {}", item.summarize());
}

pub fn traits_usage_study() {
    let article = Article {
        headline: String::from("article headline"),
        location: String::from("canada"),
        author: String::from("clare"),
    };
    println!("article summary {}", article.summarize());
    notify(&article);
}
