pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news 2! {} {}", item1.summarize(), item1.summarize());
}

use core::fmt::Debug;
use core::fmt::Display;
pub fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: "horse_ebooks".to_string(),
        content: "of course, as you probably already know, people".to_string(),
        reply: false,
        retweet: false,
    }
}

use std::sync::Arc;
pub fn produces_arc_str() -> Arc<str> { 
    "poupa".into() 
}

pub fn accepts_arc_str(s: &str) {
}

pub fn passes_arc_str() {
    accepts_arc_str(&produces_arc_str());
    accepts_arc_str("poupa");
    accepts_arc_str(&"poupa".to_string());
}

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

#[cfg(test)]
mod tests {
    use super::*;

    fn na() -> NewsArticle {
        NewsArticle { 
            headline: "headline_v".to_string(), 
            location: "location_v".to_string(), 
            author: "author_v".to_string(), 
            content: "content_v".to_string(), 
        }
    }

    fn tweet() -> Tweet {
        Tweet {
            username: "poupathegreat".to_string(),
            content: "zzzz".to_string(),
            reply: false,
            retweet: false,
        }
    }

    #[test]
    fn works_for_news_article() {
        println!("{}", na().summarize_author());
        println!("{}", na().summarize());
    }

    #[test]
    fn works_for_default() {

        println!("{}", tweet().summarize_author());
        println!("{}", tweet().summarize());
    }

    #[test]
    fn impl_trait() {
        notify(&na());
        notify(&tweet());

        notify2(&na(), &tweet());
    }
}

