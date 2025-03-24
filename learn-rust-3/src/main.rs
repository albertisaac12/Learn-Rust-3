
// use learn_rust_3::eat_at_restaurant;


use learn_rust_3::{NewsArticle,Summary,Tweet};

fn main() {
    println!("Hello, world!");
    // eat_at_restaurant();

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    article.blah();

    takeTraitVar(&article);

}





fn takeTraitVar(item: &impl Summary){
    item.blah();
}