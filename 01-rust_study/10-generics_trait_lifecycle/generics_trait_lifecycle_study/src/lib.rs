use std::fmt::{Debug, Display};

pub struct Twitter {
    pub content: String,
    pub time_span: usize,
}

pub struct WeChat {
    pub content: String,
    pub frind: String,
}

pub trait Summary {
    fn summary_content(&self) -> String {
        format!("summary...")
    }
}

impl Summary for WeChat {}

impl Summary for Twitter {
    fn summary_content(&self) -> String {
        format!("content:{},timespan:{}", self.content, self.time_span)
    }
}

pub fn print_summary<T, U>(item: T, item1: U)
where
    T: Display + Summary,
    U: Clone + Debug,
{
    println!("{}", item.summary_content());
}


struct Point<T>{
    x:T,
    y:T
}
impl<T:Copy> Point<T>{
    
}