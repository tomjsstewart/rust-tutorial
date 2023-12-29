// Define a public trait
pub trait Summary {
    fn Summarise_author(&self) -> String;

    fn Summarise(&self) -> String {
        format!("(Read more from {}...)", self.Summarise_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn Summarise_author(&self) -> String {
        format!("{}", self.author)
    }

    fn Summarise(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn Summarise_author(&self) -> String {
        format!("@{}", self.username)
    }
    // fn Summarise(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
}
