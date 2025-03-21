
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
}


// https://www.wired.com/story/prosocial-media-social-networks-discourse-decentralization/
// https://www.theverge.com/2023/4/18/23672769/social-media-inevitable-death-monetization-growth-hacks
// https://arstechnica.com/ai/2023/12/ai-created-virtual-influencers-are-stealing-business-from-humans/
// https://www.theverge.com/2024/9/25/24254042/mark-zuckerberg-creators-value-ai-meta
// https://www.wired.com/story/blockchain-open-web-user-data/
// https://www.cnet.com/news/social-media/how-were-bringing-back-the-social-part-of-social-media/

