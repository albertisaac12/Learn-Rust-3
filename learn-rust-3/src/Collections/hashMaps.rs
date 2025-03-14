use std::collections::HashMap;

fn main() {

    let mut scores = HashMap::new();

    scores.insert("blah".to_string(), 10);
    scores.insert("meow".to_string(), 20);

    let score = scores.get("blah").copied().unwrap_or(0);
    
    for (key,values) in &scores {
        println!("{key} : {values}");
    }

    // updating the hashMap

    // overwriting a value
    let mut map = HashMap::new();
    map.insert(2, 2);
    map.insert(2, 5);

    // Adding a Key and Value only if a Key Isn't persistant
    // (Meaning if a key exists dont change the associated if not add the key and value );

    let mut blah = HashMap::new();
    blah.insert(1,2);

    blah.entry(1).or_insert(10);
    blah.entry(2).or_insert(3);

    println!("{blah:?}");


    let text = "hello world how are you world";

    let mut map = HashMap::new();
    for words in text.split_whitespace() {
        let count = map.entry(words).or_insert(0);
        *count+=1;
    }

    println!("{map:?}");

}