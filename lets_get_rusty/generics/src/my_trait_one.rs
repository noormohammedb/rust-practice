use std::fmt::{Debug, Display};
pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn sum_auth(&self) -> String {
        String::from(&self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn sum_auth(&self) -> String {
        String::from(&self.username)
    }

    fn summ_default(&self) -> String {
        /* for override trait implementation, re-implement implimentaion
         * for perticulart struct
         */
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary {
    fn sum_auth(&self) -> String;

    fn summ_with_auth(&self) -> String {
        format!("Read More with, {}", &self.sum_auth())
    }

    fn summ_default(&self) -> String {
        /* for defautl trait definition is implemented in trait */
        String::from("(Read More...)")
    }
}

pub fn trait_one() {
    let my_tweet = Tweet {
        username: String::from("john"),
        content: String::from("bar"),
        reply: false,
        retweet: false,
    };

    let my_article = NewsArticle {
        author: String::from("bob"),
        headline: String::from("koo"),
        content: String::from("lorem ipsum dollor"),
    };

    println!("\nTweet Summary: {}", my_tweet.summ_default());
    println!("News Summary: {}", my_article.summ_default());
    println!("Twtte Summary author: {}", my_tweet.summ_with_auth());
    println!("News Summary author: {}", my_article.summ_with_auth());

    notify(&my_article);
    notify_bound(&my_tweet);

    println!("{}", returns_summarizable().summ_default());
}

// trait as prameter
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summ_default());
}

// trait bound (syntax sugar)
pub fn notify_bound<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summ_default());
}

pub fn notify_ex_one(item1: &impl Summary, item2: &impl Summary) {
    //...
}
pub fn notify_ex_two<T: Summary>(item1: &T, item2: &T) {
    //...
}

pub fn multiple_trait_arg(item1: &(impl Summary + Display)) {}

pub fn multiple_trait_arg_as_bound<T: Summary + Display>(item1: &T) {}

pub fn multiple_trait_bound_as_arg<T: Summary + Display, U: Clone + Debug>(ar1: &T, ar2: &U) {}

pub fn clearar_trait_bounds_with_where_clauses<T, U>(arg1: &T, arg2: &U)
where
    T: Summary + Display,
    U: Display + Debug,
{
}

pub fn clearar_trait_bounds_with_where_clauses_and_return<T, U>(arg1: &T, arg2: &U) -> i32
where
    T: Summary + Display,
    U: Display + Debug,
{
    1
}

pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("Sunny"),
        content: String::from("foo"),
        reply: false,
        retweet: false,
    }
}
