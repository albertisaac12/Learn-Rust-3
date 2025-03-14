fn main() {


    let mut s = String::new();

    s.push_str("hell yeah");
    s.push(' ');


    let mut s1 = String::from("meow meow");

    s.push_str(&s1);

    let to_be_pushed = " blah";

    s.push_str(to_be_pushed);

    println!("{}",to_be_pushed);

    let s2 = String::from("Hello");
    let s3 = String::from(" World");

    let s4 = s2 + &s3; // the reason for this format is that the add function that is called has this signature add(self, s: &str) -> String;



    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    // the above concatenation is too complex

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    println!("{}",&s);

    // iterating over strings

    for chars in s.chars() {
        println!("{chars}");
    }

    for bytes in s.bytes(){
        println!("{bytes}");
    }
}