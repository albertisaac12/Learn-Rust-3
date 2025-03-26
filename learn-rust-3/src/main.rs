
// use learn_rust_3::eat_at_restaurant;


use std::fmt::{Debug, Display};

use learn_rust_3::{NewsArticle,Summary,Tweet,blah,balh1};

#[derive(Debug)]
struct Point<x1,y1>{
    x:x1,
    y:y1
}

impl<x2,y2> Point<x2,y2> {
    fn mixup<x3,y3>(self,other:Point<x3,y3>)-> Point<x2,y3>{
        Point { x: self.x, y: other.y }
    }
}


struct nnn {
    x: u16,
    y: usize,
    z: String
}

impl  nnn {
    
    fn mew1(&self){ // reference to the point
        println!("{}",&self.x);
        println!("{}",self.y);
        println!("{}",self.z);

        // self.z // will give an error
    }

    fn mew2(&mut self){ // mutable reference to the point

    }

    fn mew3(self) { // move the value completely

    }
}

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


    let mc = Point {
        x: 10,
        y:"heeee".to_string()
    };

    let bc = mc.mixup(Point { x: 15, y: 10 });

    println!("{:?}",bc);

    let mmm = nnn {
        x:1,y:1,z:"hello".to_string()
    };

    let ppp = nnn {
        ..mmm
    };

    println!("{:?}",mmm.x);

    ppp.mew1();
    

    let var = blah {
        username: "mewo".to_string()
    };

    var.callCat();

    catCaller(&var);

    catCaller2(&var);

}




// trait as a parameter

fn takeTraitVar(item: &impl Summary){ // take a reference to a variable that implements Summary trait
    item.blah();
}


fn catCaller(item: &impl balh1) {
    item.callCat();
} 



/*

    Notes:

    &self: When you use &self in a method signature, 
    it means the method takes a borrowed reference to the instance of the struct


    Implicit Dereferencing: Rust has a feature called implicit dereferencing. 
    When you access a field of a struct through a reference (like self.x or self.y or self.z when self is &self), 
    Rust automatically dereferences the reference for you.

*/






// Trait Bound Syntax

fn catCaller2<T: balh1> (item: &T) {
    item.callCat();
}

/*
    Use these when there are a billion traits that are being implemented

    pub fn notify(item: &(impl Summary + Display)) {
    pub fn notify<T: Summary + Display>(item: &T) {

*/


fn klj<T,U>(t:&T,u:&U) -> i32 where T: Display+Clone, U: Clone+Debug {
    5
}


fn aa() -> impl balh1 {
    blah {
        username: "bow".to_string()
    }

    // dirent types that implement blah1 cant be returned only a single type at a type for a single function scope can be used
}

