//使用trait关键字来声明一个trait Summary
pub trait Summary{
//    在方法签名后跟分号，而不是在大括号中提供其实现
//    fn summarize(&self) -> String;
//    可以带有默认实现

    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String{
        format!("(Read more from {}...)", self.summarize_author())
    }
//    fn summarize(&self) -> String{
//        String::from("(Read more...)")
//    }
}

pub struct NewsArticle{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        unimplemented!()
    }

    fn summarize(&self) -> String {
        unimplemented!()
    }
//    fn summarize(&self) -> String{
//        format!("{}, by {} ({})", self.headline, self.author, self.location)
//    }
}

pub struct Tweet{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {

    fn summarize_author(&self) -> String{
        format!("@{}", self.username)
    }
//    fn summarize(&self) -> String{
//        format!("{}: {}", self.username, self.content)
//    }
}

fn main() {
    let tweet = Tweet{
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
